pub mod models;
// Ignore schema.rs as it is not formatted sufficiently for rustfmt but we also do not care as it is auto-generated code
#[rustfmt::skip]
pub mod schema;

use crate::diesel::BelongingToDsl;
use crate::diesel::Connection;
use crate::diesel::ExpressionMethods;
use crate::diesel::OptionalExtension;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use diesel_migrations::MigrationHarness;

use std::convert::TryInto;
use std::fs::set_permissions;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;

use anyhow::Context;
use anyhow::Error;

use nix::unistd::chown;
use nix::unistd::geteuid;
use nix::unistd::Gid;
use nix::unistd::Group;
use nix::unistd::Uid;
use nix::unistd::User;

use crate::helper::Config;

pub struct DB<'a> {
    conn: diesel::SqliteConnection,
    global_settings: &'a Config,
}

impl<'a> DB<'a> {
    pub fn new(global_settings: &'a Config) -> Result<Self, Error> {
        log::trace!("Creating new DB object");
        let database_url = &global_settings.public_database_path;
        let mut conn = diesel::SqliteConnection::establish(database_url)
            .with_context(|| format!("Cannot connect to database {database_url}"))?;
        diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut conn)?;
        log::trace!("Enabled foreign key check on DB");

        // we use geteuid as when a user authenticates from itself (e.g. sudo) we are running under the users name but effectively as root
        if geteuid().is_root() {
            log::debug!("Setting permissions on database");
            chown(
                Path::new(&database_url),
                Some(Uid::from_raw(0)),
                Some(Gid::from_raw(0)),
            )?;
            set_permissions(Path::new(&database_url), PermissionsExt::from_mode(0o644))?;
        }

        // run migrations
        {
            const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
                diesel_migrations::embed_migrations!("./migrations");
            conn.run_pending_migrations(MIGRATIONS).unwrap();
        }

        Ok(Self {
            conn,
            global_settings,
        })
    }

    fn find_next_unused_user_id_and_name(&mut self) -> Result<(i32, String), Error> {
        use schema::users::dsl::*;

        // find next unused ID
        let mut max_user_id: i32 =
            (self.global_settings.uid_minimum.saturating_sub(1)).try_into()?;
        if let Some(cur_max_id) = users.select(diesel::dsl::max(id)).first(&mut self.conn)? {
            max_user_id = std::cmp::max(cur_max_id, max_user_id)
        }
        if max_user_id < 1 {
            bail!("Negative (and 0) user ids are not supported!");
        }
        let mut next_user_id = max_user_id;
        let mut next_username: String;

        let username_prefix = &self.global_settings.guest_username_prefix;

        // check whether user id or name is already being used on system
        loop {
            next_user_id = next_user_id.checked_add(1).unwrap();

            if next_user_id > self.global_settings.uid_maximum.try_into()? {
                bail!("No free user id found!");
            }

            next_username = format!("{username_prefix}-{next_user_id}");

            if User::from_uid(Uid::from_raw(next_user_id.try_into()?))?.is_some() {
                log::debug!("User ID {next_user_id} already being used on system");
                continue;
            }
            if User::from_name(&next_username)?.is_some() {
                log::debug!("User name {next_username} already being used on system");
                continue;
            }

            break;
        }

        log::info!("Next free user id is {next_user_id} with name {next_username}");
        Ok((next_user_id, next_username))
    }

    fn find_next_unused_group_id_and_name(&mut self) -> Result<(i32, String), Error> {
        use schema::users::dsl::*;

        let mut max_group_id: i32 =
            (self.global_settings.gid_minimum.saturating_sub(1)).try_into()?;
        if let Some(cur_max_id) = users.select(diesel::dsl::max(id)).first(&mut self.conn)? {
            max_group_id = std::cmp::max(cur_max_id, max_group_id)
        }
        let mut next_group_id = max_group_id;
        let mut next_group_name: String;

        let group_name_prefix = &self.global_settings.guest_group_name_prefix;

        // check whether group id or name is already being used on system
        loop {
            next_group_id = next_group_id.checked_add(1).unwrap();
            next_group_name = format!("{group_name_prefix}-{next_group_id}");

            if Group::from_gid(Gid::from_raw(next_group_id.try_into()?))?.is_some() {
                log::debug!("Group ID {next_group_id} already being used on system");
                continue;
            }
            if Group::from_name(&next_group_name)?.is_some() {
                log::debug!("Group name {next_group_name} already being used on system");
                continue;
            }

            break;
        }

        if next_group_id > self.global_settings.gid_maximum.try_into()? {
            bail!("No free group id found!");
        }
        log::info!("Next free group id is {next_group_id} with name {next_group_name}");
        Ok((next_group_id, next_group_name))
    }

    pub fn create_guest_user(&mut self) -> Result<models::User, Error> {
        let (group_id, group_name) = self.find_next_unused_group_id_and_name()?;

        let target_group = models::Group {
            id: group_id,
            group_name,
        };

        let home_base_path = &self.global_settings.home_base_path;
        let (user_id, username) = self.find_next_unused_user_id_and_name()?;
        let current_boot_id = crate::helper::get_current_os_boot_id()?;
        let target_user = models::User {
            id: user_id,
            user_group_id: group_id,
            user_name: username.clone(),
            home_path: format!("{home_base_path}/{username}"),
            boot_id: current_boot_id,
        };

        if Path::new(&target_user.home_path).exists() {
            bail!("Home path {} already exists", &target_user.home_path);
        }
        std::fs::create_dir_all(home_base_path)?;
        chown(
            Path::new(&home_base_path),
            Some(Uid::from_raw(0)),
            Some(Gid::from_raw(0)),
        )?;
        set_permissions(Path::new(&home_base_path), PermissionsExt::from_mode(0o755))?;

        std::fs::create_dir_all(&target_user.home_path)?;
        chown(
            Path::new(&target_user.home_path),
            Some(Uid::from_raw(target_user.id as u32)),
            Some(Gid::from_raw(target_user.user_group_id as u32)),
        )?;
        let home_directory_permissions_mode = 0o700;
        set_permissions(
            Path::new(&target_user.home_path),
            PermissionsExt::from_mode(home_directory_permissions_mode),
        )?;

        // copy default home directory
        crate::helper::copy_dir_recursive_and_set_owner(
            &self.global_settings.home_skel,
            &target_user.home_path,
            Uid::from_raw(target_user.id as u32),
            Gid::from_raw(target_user.user_group_id as u32),
            false,
        )
        .context("Unable to copy skeleton home directory to new guest users home directory!")?;

        if Path::new(&target_user.home_path)
            .metadata()?
            .permissions()
            .mode()
            & 0o777
            != home_directory_permissions_mode
        {
            bail!(
                "Setting home directory permissions failed! (probably got changed in the process)"
            );
        }

        diesel::insert_into(schema::groups::dsl::groups)
            .values(&target_group)
            .execute(&mut self.conn)?;
        diesel::insert_into(schema::users::dsl::users)
            .values(&target_user)
            .execute(&mut self.conn)?;

        Ok(target_user)
    }

    pub fn get_users(&mut self) -> Result<Vec<models::User>, Error> {
        use schema::users::dsl::users;

        Ok(users.load::<models::User>(&mut self.conn)?)
    }

    pub fn find_user_by_id(&mut self, uid: i32) -> Result<Option<models::User>, Error> {
        use schema::users::dsl::{id, users};

        let result = users
            .filter(id.eq(uid))
            .first::<models::User>(&mut self.conn)
            .optional()?;

        Ok(result)
    }

    pub fn find_user_by_name(&mut self, name: &str) -> Result<Option<models::User>, Error> {
        use schema::users::dsl::{user_name, users};

        let result = users
            .filter(user_name.eq(name))
            .first::<models::User>(&mut self.conn)
            .optional()?;

        Ok(result)
    }

    pub fn get_groups(&mut self) -> Result<Vec<models::Group>, Error> {
        use schema::groups::dsl::groups;

        Ok(groups.load::<models::Group>(&mut self.conn)?)
    }

    pub fn find_group_by_id(&mut self, gid: i32) -> Result<Option<models::Group>, Error> {
        use schema::groups::dsl::{groups, id};

        let result = groups
            .filter(id.eq(gid))
            .first::<models::Group>(&mut self.conn)
            .optional()?;

        Ok(result)
    }

    pub fn find_group_by_name(&mut self, name: &str) -> Result<Option<models::Group>, Error> {
        use schema::groups::dsl::{group_name, groups};

        let result = groups
            .filter(group_name.eq(name))
            .first::<models::Group>(&mut self.conn)
            .optional()?;

        Ok(result)
    }

    pub fn find_users_for_group(
        &mut self,
        match_group: &models::Group,
    ) -> Result<Vec<(models::UserGroupMembership, models::User)>, Error> {
        Ok(models::UserGroupMembership::belonging_to(match_group)
            .inner_join(schema::users::dsl::users)
            .load::<(models::UserGroupMembership, models::User)>(&mut self.conn)?)
    }
}
