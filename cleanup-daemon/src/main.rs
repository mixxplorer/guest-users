#![deny(warnings)]
#![deny(clippy::all)]

use anyhow::Context;
use clap::Parser;
use tokio_stream::StreamExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
pub trait LoginManager {
    /// SessionNew signal
    #[zbus(signal)]
    fn session_new(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// SessionRemoved signal
    #[zbus(signal)]
    fn session_removed(
        &self,
        session_id: &str,
        object_path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}

async fn session_end_listener() -> anyhow::Result<()> {
    let global_settings = guest_users_lib::helper::get_config()?;

    let system_connection = zbus::Connection::system().await?;
    let login_interface = LoginManagerProxy::new(&system_connection).await?;

    let mut session_end_events = login_interface.receive_session_removed().await?;
    log::debug!("Set up receiver for session end events!");

    while let Some(msg) = session_end_events.next().await {
        let session_removed_args = msg.args().expect("Error parsing message");

        log::debug!("End session event received: {session_removed_args:?}");

        // Unfortunately, when receiving the session end event, we cannot get the metadata of the session anymore.
        // Therefore, we are left with two options:
        // 1. Store the metadata beforehand (but then we do have data duplicated)
        // 2. Look up which sessions are running after receiving an session end event and just try to clean up all users, which do not have a session
        // We chose the second option to prevent data duplication (and therefore nasty bugs).

        let mut db = guest_users_lib::db::DB::new(&global_settings)?;
        for user in db.get_users()? {
            // check if home directory of user still exists
            let home_path = std::path::Path::new(&user.home_path);
            if home_path.exists() {
                // failsafe: check whether this path is in home base directory
                let home_base_path = std::path::Path::new(&global_settings.home_base_path);
                if !home_path.starts_with(home_base_path) {
                    log::warn!("{home_path:?} not in home_base_path={home_base_path:?}, skipping deletion!")
                } else {
                    // check if user has a session
                    if !guest_users_lib::helper::has_active_user_sessions(&user.user_name)? {
                        log::info!(
                            "Removing home directory {home_path:?} of user {}",
                            user.user_name
                        );
                        std::fs::remove_dir_all(home_path).with_context(|| {
                            format!("Removing home directory of {} failed!", user.user_name)
                        })?;
                    } else {
                        log::info!(
                            "Skipping user {} as the user still has an active session.",
                            &user.user_name
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
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

    session_end_listener().await?;

    Ok(())
}
