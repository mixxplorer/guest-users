//! # D-Bus interface proxy for: `org.freedesktop.Accounts`
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
#[zbus::proxy(
    interface = "org.freedesktop.Accounts",
    default_service = "org.freedesktop.Accounts",
    default_path = "/org/freedesktop/Accounts"
)]
pub trait Accounts {
    /// CacheUser method
    fn cache_user(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CreateUser method
    fn create_user(
        &self,
        name: &str,
        fullname: &str,
        account_type: i32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// DeleteUser method
    fn delete_user(&self, id: i64, remove_files: bool) -> zbus::Result<()>;

    /// FindUserById method
    fn find_user_by_id(&self, id: i64) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// FindUserByName method
    fn find_user_by_name(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetUsersLanguages method
    fn get_users_languages(&self) -> zbus::Result<Vec<String>>;

    /// ListCachedUsers method
    fn list_cached_users(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// UncacheUser method
    fn uncache_user(&self, name: &str) -> zbus::Result<()>;

    /// UserAdded signal
    #[zbus(signal)]
    fn user_added(&self, user: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// UserDeleted signal
    #[zbus(signal)]
    fn user_deleted(&self, user: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// AutomaticLoginUsers property
    #[zbus(property)]
    fn automatic_login_users(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// DaemonVersion property
    #[zbus(property)]
    fn daemon_version(&self) -> zbus::Result<String>;

    /// HasMultipleUsers property
    #[zbus(property)]
    fn has_multiple_users(&self) -> zbus::Result<bool>;

    /// HasNoUsers property
    #[zbus(property)]
    fn has_no_users(&self) -> zbus::Result<bool>;
}
