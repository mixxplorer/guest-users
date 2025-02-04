#![deny(warnings)]
#![deny(clippy::all)]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity,
}

#[zbus::proxy(
    default_service = "org.freedesktop.Accounts",
    default_path = "/org/freedesktop/Accounts"
)]
trait Accounts {
    fn cache_user(&self, username: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    fn uncache_user(&self, username: &str) -> zbus::Result<()>;

    fn find_user_by_name(&self, username: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

async fn update_ghost_user() -> anyhow::Result<()> {
    let global_settings = guest_users_lib::helper::get_config()?;

    let connection = zbus::Connection::system().await?;
    let proxy = AccountsProxy::new(&connection).await?;

    if global_settings.enable_ghost_user {
        let reply = proxy
            .cache_user(&global_settings.guest_username_new_user)
            .await?;
        log::debug!("Cache user reply: {reply:?}");
    } else if proxy
        .find_user_by_name(&global_settings.guest_username_new_user)
        .await
        .is_ok()
    {
        log::debug!(
            "User {} does seem to exist, going to remove it",
            global_settings.guest_username_new_user
        );
        // The guest user is still cached, remove it
        proxy
            .uncache_user(&global_settings.guest_username_new_user)
            .await?;
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
