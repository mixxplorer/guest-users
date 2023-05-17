use std::convert::{TryFrom, TryInto};

use anyhow::{Context, Error};
use libnss::interop::Response;
use libnss::passwd::Passwd;

fn db_to_passwd(
    global_settings: &guest_users_lib::helper::Config,
    user: &guest_users_lib::db::models::User,
) -> Result<Passwd, Error> {
    let gecos = crate::gecos::Gecos {
        full_name: Some(format!(
            "{} ({})",
            global_settings.guest_username_human_readable_prefix, user.id
        )),
        room_number: None,
        work_phone: None,
        home_phone: None,
        other: None,
    };

    let new_passwd_user = Passwd {
        name: user.user_name.clone(),
        passwd: "x".to_string(), // no password set
        uid: user.id as u32,
        gid: user.user_group_id as u32,
        gecos: gecos.to_gecos_string(),
        dir: user.home_path.clone(),
        shell: global_settings.guest_shell.clone(),
    };

    Ok(new_passwd_user)
}

fn get_ghost_user(
    global_settings: guest_users_lib::helper::Config,
) -> Result<Option<Passwd>, Error> {
    if !global_settings.enable_ghost_user {
        return Ok(None);
    }

    // Set human readable username
    let gecos = crate::gecos::Gecos {
        full_name: Some(global_settings.ghost_user_gecos_username),
        room_number: None,
        work_phone: None,
        home_phone: None,
        other: None,
    };

    Ok(Some(Passwd {
        name: global_settings.guest_username_new_user,
        passwd: "x".to_string(), // no password set
        uid: global_settings
            .ghost_user_uid
            .try_into()
            .context("Unable to parse ghost user uid as u32")?,
        gid: global_settings
            .ghost_user_gid
            .try_into()
            .context("Unable to parse ghost user gid as u32")?,
        gecos: gecos.to_gecos_string(),
        dir: "/dev/null".to_string(),
        shell: "/bin/false".to_string(),
    }))
}

pub fn get_all_entries() -> Result<Response<Vec<Passwd>>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    let users = db.get_users()?;

    let mut passwd_users = Vec::new();
    for user in users.iter() {
        passwd_users.push(db_to_passwd(&global_settings, user)?);
    }

    if let Some(ghost_user) = get_ghost_user(global_settings)? {
        passwd_users.push(ghost_user);
    }

    Ok(Response::Success(passwd_users))
}

pub fn get_entry_by_uid(uid: libc::uid_t) -> Result<Response<Passwd>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_id(i32::try_from(uid)?)? {
        return Ok(Response::Success(db_to_passwd(&global_settings, &user)?));
    }

    if let Some(ghost_user) = get_ghost_user(global_settings)? {
        if ghost_user.uid == uid {
            return Ok(Response::Success(ghost_user));
        }
    }

    Ok(Response::NotFound)
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Passwd>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_name(name)? {
        return Ok(Response::Success(db_to_passwd(&global_settings, &user)?));
    }

    if let Some(ghost_user) = get_ghost_user(global_settings)? {
        if ghost_user.name == name {
            return Ok(Response::Success(ghost_user));
        }
    }

    Ok(Response::NotFound)
}
