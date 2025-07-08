use std::convert::TryInto;

use anyhow::{Context, Error};
use libnss::group::Group;
use libnss::interop::Response;

fn db_to_group(
    db: &mut guest_users_lib::db::DB,
    group: &guest_users_lib::db::models::Group,
) -> Result<Group, Error> {
    let users_in_group = db.find_users_for_group(group)?;

    let new_group_obj = Group {
        name: group.group_name.to_string(),
        passwd: "x".to_string(), // disable password for group
        gid: group.id as u32,
        members: users_in_group
            .iter()
            .map(|(_, user)| user.user_name.to_string())
            .collect(),
    };

    Ok(new_group_obj)
}

fn get_ghost_group(
    global_settings: &guest_users_lib::helper::Config,
) -> Result<Option<Group>, Error> {
    if !global_settings.enable_ghost_user {
        return Ok(None);
    }

    Ok(Some(Group {
        name: global_settings.guest_username_new_user.clone(),
        passwd: "x".to_string(), // disable password for group
        gid: global_settings
            .ghost_user_gid
            .try_into()
            .context("Unable to parse ghost user gid as u32")?,
        members: vec![global_settings.guest_username_new_user.clone()],
    }))
}

fn get_common_group(
    db: &mut guest_users_lib::db::DB,
    global_settings: &guest_users_lib::helper::Config,
) -> Result<Group, Error> {
    Ok(Group {
        name: global_settings.guest_common_group_name.clone(),
        passwd: "x".to_string(), // disable password for group
        gid: global_settings.guest_common_group_gid,
        members: db
            .get_users()?
            .iter()
            .map(|user| user.user_name.clone())
            .collect(),
    })
}

pub fn get_all_entries() -> Result<Response<Vec<Group>>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    let db_groups = db.get_groups()?;

    let mut groups = Vec::new();
    for group in db_groups.iter() {
        groups.push(db_to_group(&mut db, group)?);
    }

    if let Some(ghost_group) = get_ghost_group(&global_settings)? {
        groups.push(ghost_group);
    }

    if global_settings.enable_guest_common_group {
        groups.push(get_common_group(&mut db, &global_settings)?);
    }

    Ok(Response::Success(groups))
}

pub fn get_entry_by_gid(gid: libc::uid_t) -> Result<Response<Group>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(group) = db.find_group_by_id(gid)? {
        return Ok(Response::Success(db_to_group(&mut db, &group)?));
    }

    if let Some(ghost_group) = get_ghost_group(&global_settings)? {
        if ghost_group.gid == gid {
            return Ok(Response::Success(ghost_group));
        }
    }

    if global_settings.enable_guest_common_group && global_settings.guest_common_group_gid == gid {
        return Ok(Response::Success(get_common_group(
            &mut db,
            &global_settings,
        )?));
    }

    Ok(Response::NotFound)
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Group>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(group) = db.find_group_by_name(name)? {
        return Ok(Response::Success(db_to_group(&mut db, &group)?));
    }

    if let Some(ghost_group) = get_ghost_group(&global_settings)? {
        if ghost_group.name == name {
            return Ok(Response::Success(ghost_group));
        }
    }

    if global_settings.enable_guest_common_group && global_settings.guest_common_group_name == name
    {
        return Ok(Response::Success(get_common_group(
            &mut db,
            &global_settings,
        )?));
    }

    Ok(Response::NotFound)
}
