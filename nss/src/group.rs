use std::convert::TryFrom;

use anyhow::Error;
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

pub fn get_all_entries() -> Result<Response<Vec<Group>>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    let groups = db.get_groups()?;

    let mut passwd_users = Vec::new();
    for group in groups.iter() {
        passwd_users.push(db_to_group(&mut db, group)?);
    }

    Ok(Response::Success(passwd_users))
}

pub fn get_entry_by_gid(gid: libc::uid_t) -> Result<Response<Group>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(group) = db.find_group_by_id(i32::try_from(gid)?)? {
        return Ok(Response::Success(db_to_group(&mut db, &group)?));
    }

    Ok(Response::NotFound)
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Group>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(group) = db.find_group_by_name(name)? {
        return Ok(Response::Success(db_to_group(&mut db, &group)?));
    }

    Ok(Response::NotFound)
}
