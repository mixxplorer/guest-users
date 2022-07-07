use std::convert::TryFrom;

use anyhow::Error;
use libnss::interop::Response;
use libnss::passwd::Passwd;

fn db_to_passwd(
    global_settings: &config::Config,
    user: &guest_users_lib::db::models::User,
) -> Result<Passwd, Error> {
    let new_passwd_user = Passwd {
        name: user.user_name.clone(),
        passwd: "x".to_string(), // no password set
        uid: user.id as u32,
        gid: user.user_group_id as u32,
        gecos: "".to_string(), // empty gecos as we don't have any infos about the user
        dir: user.home_path.clone(),
        shell: global_settings.get_string("GUEST_SHELL")?,
    };

    Ok(new_passwd_user)
}

pub fn get_all_entries() -> Result<Response<Vec<Passwd>>, Error> {
    let global_settings = guest_users_lib::helper::load_settings()?;
    let db = guest_users_lib::db::DB::new(&global_settings)?;

    let users = db.get_users()?;

    let mut passwd_users = Vec::new();
    for user in users.iter() {
        passwd_users.push(db_to_passwd(&global_settings, user)?);
    }

    Ok(Response::Success(passwd_users))
}

pub fn get_entry_by_uid(uid: libc::uid_t) -> Result<Response<Passwd>, Error> {
    let global_settings = guest_users_lib::helper::load_settings()?;
    let db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_id(i32::try_from(uid)?)? {
        return Ok(Response::Success(db_to_passwd(&global_settings, &user)?));
    }

    Ok(Response::NotFound)
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Passwd>, Error> {
    let global_settings = guest_users_lib::helper::load_settings()?;
    let db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_name(name)? {
        return Ok(Response::Success(db_to_passwd(&global_settings, &user)?));
    }

    Ok(Response::NotFound)
}
