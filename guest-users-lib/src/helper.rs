use std::path::Path;

use anyhow::Result;
use config::Config;

const CONFIG_FILE_PATH: &str = "/etc/guest-users/settings.toml";

pub fn load_settings() -> Result<Config> {
    let mut builder = Config::builder()
        .set_default("GUEST_USERNAME_NEW_USER", "guest")?
        .set_default("GUEST_USERNAME_PREFIX", "guest")?
        .set_default("GUEST_GROUP_NAME_PREFIX", "guest")?
        .set_default("HOME_BASE_PATH", "/tmp/guest-users-home")?
        .set_default("GUEST_SHELL", "/bin/bash")?
        .set_default("GUEST_USER_DATABASE_PATH", "/etc/guest-users/public.db")?
        .set_default("UID_MINIMUM", 31000)?
        .set_default("UID_MAXIMUM", 31999)?
        .set_default("GID_MINIMUM", 31000)?
        .set_default("GID_MAXIMUM", 31999)?;

    // Only load config if config file really exists
    if Path::new(&CONFIG_FILE_PATH).exists() {
        builder = builder.add_source(config::File::with_name(CONFIG_FILE_PATH));
    } else {
        log::debug!("Config file {} does not exist", CONFIG_FILE_PATH)
    }

    Ok(builder.build().unwrap())
}

pub fn init_logger() {
    // Ignore error here as this could be called multiple times and then a SetLoggerError will be thrown
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .env()
        .init()
        .ok();
}
