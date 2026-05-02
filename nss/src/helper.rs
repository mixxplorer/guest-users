/// Returns whether the nss module should return all users (even if they have been e.g. cleaned up)
pub(crate) fn should_return_all_users() -> bool {
    std::env::var_os("GUEST_USERS_SHOW_ALL_USERS").is_some()
}
