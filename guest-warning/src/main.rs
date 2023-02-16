#![deny(warnings)]
#![deny(clippy::all)]
#![allow(clippy::too_many_arguments)] // allow notify in Notifications trait

use std::{collections::HashMap, convert::TryInto, error::Error};

use clap::Parser;
use futures::executor::block_on;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity,
}

#[zbus::dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

async fn notify_if_guest_user() -> Result<(), Box<dyn Error>> {
    let global_settings = guest_users_lib::helper::get_config()?;

    let cur_user_id = nix::unistd::Uid::current();
    // check whether this user id belongs to a guest user
    let db = guest_users_lib::db::DB::new(&global_settings)?;
    if db
        .find_user_by_id(cur_user_id.as_raw().try_into()?)?
        .is_none()
    {
        log::debug!("User does not seem to be a guest user (not found in guest users DB)");
        return Ok(());
    }

    log::trace!("Setting up zbus connection...");
    let connection = zbus::Connection::session().await?;
    log::trace!("Setting up zbus NotificationsProxy...");
    let proxy = NotificationsProxy::new(&connection).await?;

    log::trace!("Sending notification...");
    let reply = proxy
        .notify(
            "guest-users",
            0,
            "warning",
            &global_settings.guest_user_warning_title,
            &global_settings.guest_user_warning_body,
            &[],
            HashMap::from([("urgency", &zbus::zvariant::Value::I16(2))]),
            0,
        )
        .await?;
    log::debug!("Got notification ID={reply}");

    Ok(())
}

fn main() {
    let args = Args::parse();

    simple_logger::SimpleLogger::new()
        .with_level(args.log_level.log_level().unwrap().to_level_filter())
        .with_utc_timestamps()
        .init()
        .unwrap();

    let future = notify_if_guest_user();
    block_on(future).unwrap();
}
