extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use libnss::group::{Group, GroupHooks};
use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};
use libnss::shadow::{Shadow, ShadowHooks};

mod group;
mod passwd;
mod shadow;

struct GuestUsersPasswd;
libnss_passwd_hooks!(guest_users, GuestUsersPasswd);

impl PasswdHooks for GuestUsersPasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_all_entries");
        match passwd::get_all_entries() {
            Ok(result) => result,
            Err(err) => {
                log::warn!("Could not get all entries (user): {:?}", err);
                Response::Unavail
            }
        }
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> Response<Passwd> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_entry_by_uid");
        match passwd::get_entry_by_uid(uid) {
            Ok(result) => {
                log::trace!("get_entry_by_uid: ok");
                result
            }
            Err(err) => {
                log::warn!("Could not get entry by uid (user): {:?}", err);
                Response::Unavail
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_entry_by_name (user)");
        match passwd::get_entry_by_name(&name) {
            Ok(result) => {
                log::trace!("get_entry_by_name (user): ok");
                result
            }
            Err(err) => {
                log::warn!("Could not get entry by user name (user): {:?}", err);
                Response::Unavail
            }
        }
    }
}

struct GuestUsersShadow;
libnss_shadow_hooks!(guest_users, GuestUsersShadow);

impl ShadowHooks for GuestUsersShadow {
    fn get_all_entries() -> Response<Vec<Shadow>> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_all_entries");
        match shadow::get_all_entries() {
            Ok(result) => result,
            Err(err) => {
                log::warn!("Could not get all entries (shadow): {:?}", err);
                Response::Unavail
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Shadow> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_entry_by_name (user)");
        match shadow::get_entry_by_name(&name) {
            Ok(result) => {
                log::trace!("get_entry_by_name (user): ok");
                result
            }
            Err(err) => {
                log::warn!("Could not get entry by user name (shadow): {:?}", err);
                Response::Unavail
            }
        }
    }
}

struct GuestUserGroups;
libnss_group_hooks!(guest_users, GuestUserGroups);

impl GroupHooks for GuestUserGroups {
    fn get_all_entries() -> Response<Vec<Group>> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_all_entries group");
        match group::get_all_entries() {
            Ok(result) => result,
            Err(err) => {
                log::warn!("Could not get all entries (group): {:?}", err);
                Response::Unavail
            }
        }
    }

    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_entry_by_gid");
        match group::get_entry_by_gid(gid) {
            Ok(result) => {
                log::trace!("get_entry_by_gid: ok");
                result
            }
            Err(err) => {
                log::warn!("Could not get entry by gid: {:?}", err);
                Response::Unavail
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        guest_users_lib::helper::init_logger();
        log::trace!("get_entry_by_name (group)");
        match group::get_entry_by_name(&name) {
            Ok(result) => {
                log::trace!("get_entry_by_name (group): ok");
                result
            }
            Err(err) => {
                log::warn!("Could not get entry by group name: {:?}", err);
                Response::Unavail
            }
        }
    }
}
