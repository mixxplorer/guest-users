use pam::{export_pam_module, PamHandle, PamModule, PamReturnCode};

mod handler;

pub struct GuestUserPAMModule;
impl PamModule for GuestUserPAMModule {
    fn account_management(
        handle: &PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        guest_users_lib::helper::init_logger();
        log::trace!("Account management");

        match handler::account_management(handle, args, flags) {
            Ok(result) => {
                log::trace!("account_management: ok");
                result
            }
            Err(err) => {
                log::warn!("account_management failure: {err:?}");
                PamReturnCode::Service_Err
            }
        }
    }

    fn authenticate(
        handle: &mut PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        guest_users_lib::helper::init_logger();
        log::trace!("authenticate");

        match handler::authenticate(handle, args, flags) {
            Ok(result) => {
                log::trace!("authenticate: ok");
                result
            }
            Err(err) => {
                log::warn!("authenticate failure: {err:?}");
                PamReturnCode::Service_Err
            }
        }
    }

    fn change_auth_token(
        _handle: &PamHandle,
        _args: Vec<&std::ffi::CStr>,
        _flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        log::trace!("Change auth token");
        PamReturnCode::Ignore
    }

    fn close_session(
        _handle: &PamHandle,
        _args: Vec<&std::ffi::CStr>,
        _flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        log::trace!("Close session");
        PamReturnCode::Ignore
    }

    fn open_session(
        _handle: &PamHandle,
        _args: Vec<&std::ffi::CStr>,
        _flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        log::trace!("Open session");
        PamReturnCode::Ignore
    }

    fn set_credentials(
        _handle: &PamHandle,
        _args: Vec<&std::ffi::CStr>,
        _flags: std::os::raw::c_uint,
    ) -> PamReturnCode {
        log::trace!("Set credentials");
        PamReturnCode::Ignore
    }
}

export_pam_module!(GuestUserPAMModule);
