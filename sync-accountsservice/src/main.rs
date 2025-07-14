#![deny(warnings)]
#![deny(clippy::all)]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity,
}

async fn update_ghost_user() -> anyhow::Result<()> {
    let global_settings = guest_users_lib::helper::get_config()?;

    let connection = zbus::Connection::system().await?;
    let proxy = guest_users_lib::zbus::accounts_service::AccountsProxy::new(&connection).await?;

    if global_settings.enable_ghost_user {
        let reply = proxy
            .cache_user(&global_settings.guest_username_new_user)
            .await?;
        log::debug!("Cache user reply: {reply:?}");
    } else {
        // unfortunately, we cannot test, whether the user is still active as when searching for a user,
        // the nss library is used, which does not return the user anymore at this point
        log::debug!(
            "Removing user {} from cache as ghost user is deactivated",
            global_settings.guest_username_new_user
        );
        // The guest user is still cached, remove it
        let reply = proxy
            .uncache_user(&global_settings.guest_username_new_user)
            .await;

        log::debug!("Uncache result: {reply:?}");
    }

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

    update_ghost_user().await
}
