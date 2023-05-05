use anyhow::Error;
use libnss::interop::Response;
use libnss::shadow::Shadow;

fn db_to_shadow(user: &guest_users_lib::db::models::User) -> Result<Shadow, Error> {
    let new_shadow_user = Shadow {
        name: user.user_name.to_string(),
        // "!" marks the user deactivated, which in turn hides the user in user lists like on the gdm login screen
        // see https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/34fd681936491333fe807b04ea677d47accd71cc/js/gdm/loginDialog.js#L254
        // and https://gitlab.freedesktop.org/accountsservice/accountsservice/-/blob/aef0cf1379a07e7d9819b15ff1045a181043ac8b/src/user.c#L466
        passwd: "!".to_string(),
        last_change: 0,
        change_min_days: 0,
        change_max_days: 99999,
        change_warn_days: 7,
        change_inactive_days: -1,
        expire_date: -1,
        reserved: 0,
    };

    Ok(new_shadow_user)
}

pub fn get_all_entries() -> Result<Response<Vec<Shadow>>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    let users = db.get_users()?;

    let mut passwd_users = Vec::new();
    for user in users.iter() {
        passwd_users.push(db_to_shadow(user)?);
    }

    Ok(Response::Success(passwd_users))
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Shadow>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_name(name)? {
        return Ok(Response::Success(db_to_shadow(&user)?));
    }

    Ok(Response::NotFound)
}
