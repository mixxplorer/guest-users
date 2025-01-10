#![deny(warnings)]
#![deny(clippy::all)]
#![allow(clippy::too_many_arguments)] // allow notify in Notifications trait

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity,
}

#[zbus::proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

async fn notify_if_guest_user() -> anyhow::Result<()> {
    let global_settings = guest_users_lib::helper::get_config()?;

    let cur_user_id = nix::unistd::Uid::current();
    // check whether this user id belongs to a guest user
    let mut db = guest_users_lib::db::DB::new(&global_settings)?;
    if db.find_user_by_id(cur_user_id.as_raw())?.is_none() {
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
            &global_settings.guest_user_warning_app_name,
            0,
            "warning",
            &global_settings.guest_user_warning_title,
            &global_settings.guest_user_warning_body,
            &[],
            std::collections::HashMap::from([("urgency", &zbus::zvariant::Value::I16(2))]),
            0,
        )
        .await?;
    log::debug!("Got notification ID={reply}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(main_async())
}

async fn main_async() -> anyhow::Result<()> {
    let args = Args::parse();

    simple_logger::SimpleLogger::new()
        .with_level(args.log_level.log_level().unwrap().to_level_filter())
        .with_utc_timestamps()
        .init()
        .unwrap();

    notify_if_guest_user().await
}
