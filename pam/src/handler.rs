use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};

use anyhow::{Context, Error};
use pam::{PamHandle, PamItemType, PamReturnCode};

pub fn account_management(
    handle: &PamHandle,
    _args: Vec<&std::ffi::CStr>,
    _flags: std::os::raw::c_uint,
) -> Result<PamReturnCode, Error> {
    let login_user = pam::get_user(handle, Some("login"))?;
    log::trace!("login_user={}", login_user);

    let global_settings = guest_users_lib::helper::get_config()?;
    let db = guest_users_lib::db::DB::new(&global_settings)?;

    if db.find_user_by_name(login_user)?.is_some() {
        return Ok(PamReturnCode::Success);
    }

    Ok(PamReturnCode::Ignore)
}

fn get_user_from_handle(handle: &mut PamHandle) -> Result<String, Error> {
    Ok(
        unsafe { CStr::from_ptr(pam::get_item(handle, PamItemType::User)? as *mut c_char) }
            .to_str()
            .context("Invalid return for PAM get_item (no UTF-8)")?
            .to_string(),
    )
}

pub fn authenticate(
    handle: &mut PamHandle,
    _args: Vec<&std::ffi::CStr>,
    _flags: std::os::raw::c_uint,
) -> Result<PamReturnCode, Error> {
    let global_settings = guest_users_lib::helper::get_config()?;
    let guest_username_new_user = &global_settings.guest_username_new_user;
    log::debug!("PAM handle={:?}", handle);
    let login_username = pam::get_user(handle, Some("login"))?;

    let db = guest_users_lib::db::DB::new(&global_settings)?;

    // check whether the login is matching the new guest user username, so we have to create a new user
    if guest_username_new_user == login_username {
        log::debug!("Username {} matches!", login_username);
        pam::putenv(handle, "IS_GUEST_USER=true")?;

        // create completely new user
        let new_user = db.create_guest_user()?;
        let new_user_name = CString::new(new_user.user_name)?;
        pam::set_item(
            handle,
            PamItemType::User,
            new_user_name.as_ptr() as *const c_void,
        )?;

        let user_name_set = get_user_from_handle(handle)?;

        // make sure setting the new user worked
        if new_user_name.to_str()? != user_name_set {
            log::error!(
                "Set user name does not match: user_name_set={}, new_user_name={}",
                user_name_set,
                new_user_name.into_string()?
            );
            return Ok(PamReturnCode::Service_Err);
        }
        Ok(PamReturnCode::Success)
    } else if let Some(user) = db.find_user_by_name(login_username)? {
        // we found the guest user
        // as guest users do not have any password, we just let them through if the boot id is still the same (system did not reboot)
        if user.boot_id != guest_users_lib::helper::get_current_os_boot_id()? {
            return Ok(PamReturnCode::Auth_Err);
        }
        Ok(PamReturnCode::Success)
    } else {
        log::debug!("Username {} does not match.", login_username);
        Ok(PamReturnCode::Ignore)
    }
}
