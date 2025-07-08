use anyhow::Error;
use libnss::interop::Response;
use libnss::shadow::Shadow;

fn db_to_shadow(user: &guest_users_lib::db::models::User) -> Result<Shadow, Error> {
    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    tokio_runtime.block_on(async {
        let passwd = {
            if guest_users_lib::helper::get_current_os_boot_id()? == user.boot_id
                && guest_users_lib::helper::has_active_user_sessions(user.id)
                    .await
                    .unwrap_or(false)
            {
                "x"
            } else {
                // "!" marks the user deactivated, which in turn hides the user in user lists like on the gdm login screen
                // see https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/34fd681936491333fe807b04ea677d47accd71cc/js/gdm/loginDialog.js#L254
                // and https://gitlab.freedesktop.org/accountsservice/accountsservice/-/blob/aef0cf1379a07e7d9819b15ff1045a181043ac8b/src/user.c#L466
                "!"
            }
        };

        let new_shadow_user = Shadow {
            name: user.user_name.to_string(),
            passwd: passwd.to_string(),
            last_change: 0,
            change_min_days: 0,
            change_max_days: 99999,
            change_warn_days: 7,
            change_inactive_days: -1,
            expire_date: -1,
            reserved: 0,
        };

        Ok(new_shadow_user)
    })
}

fn get_ghost_user(
    global_settings: guest_users_lib::helper::Config,
) -> Result<Option<Shadow>, Error> {
    if !global_settings.enable_ghost_user {
        return Ok(None);
    }

    Ok(Some(Shadow {
        name: global_settings.ghost_user_gecos_username,
        passwd: "x".to_string(),
        last_change: 0,
        change_min_days: 0,
        change_max_days: 99999,
        change_warn_days: 7,
        change_inactive_days: -1,
        expire_date: -1,
        reserved: 0,
    }))
}

pub fn get_all_entries() -> Result<Response<Vec<Shadow>>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    let users = db.get_users()?;

    let mut shadow_users = Vec::new();
    for user in users.iter() {
        shadow_users.push(db_to_shadow(user)?);
    }

    if let Some(ghost_user) = get_ghost_user(global_settings)? {
        shadow_users.push(ghost_user);
    }

    Ok(Response::Success(shadow_users))
}

pub fn get_entry_by_name(name: &str) -> Result<Response<Shadow>, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;

    if let Some(user) = db.find_user_by_name(name)? {
        return Ok(Response::Success(db_to_shadow(&user)?));
    }

    if let Some(ghost_user) = get_ghost_user(global_settings)? {
        if ghost_user.name == name {
            return Ok(Response::Success(ghost_user));
        }
    }

    Ok(Response::NotFound)
}
