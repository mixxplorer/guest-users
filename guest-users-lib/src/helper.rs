use std::path::Path;

use anyhow::Result;

const CONFIG_FILE_PATH: &str = "/etc/guest-users/settings.toml";

macro_rules! config_default_item {
    ( $z:expr, $a:ident, String ) => {
        $z.get_string(stringify!($a))?
    };
    ( $z:expr, $a:ident, i64 ) => {
        $z.get_int(stringify!($a))?
    };
}

/// Wrapper for having a config object pre-filled with default values when building via Config::default from a ConfigBuilder
/// When called, this macro will create a Config struct containing all config values.
/// To fill the object, the default function can be used together with a pre-configured config::ConfigBuilder
/// Call format: config_default(name1, type1, default1, name2, type2, default2, ...)
macro_rules! config_default {
    ( $( $a:ident, $b:ident, $c:expr ),+ ) => {
        pub struct Config {
            $(
                pub $a: $b,
            )+
        }

        impl Config {
            pub fn default(mut conf: config::ConfigBuilder<config::builder::DefaultState>) -> Result<Self> {
                $(
                    conf = conf.set_default(stringify!($a), $c)?;
                )+
                let built_conf = conf.build()?;
                Ok(Config {
                    $(
                        $a: config_default_item!(built_conf, $a, $b),
                    )+
                })
            }
        }
    }
}

#[rustfmt::skip::macros(config_default)]
config_default!(
    guest_username_new_user, String, "guest",
    guest_username_prefix, String, "guest",
    guest_group_name_prefix, String, "guest",
    home_base_path, String, "/tmp/guest-users-home",
    guest_shell, String, "/bin/bash",
    public_database_path, String, "/etc/guest-users/public.db",
    uid_minimum, i64, 31000,
    uid_maximum, i64, 31999,
    gid_minimum, i64, 31000,
    gid_maximum, i64, 31999
);

pub fn get_config() -> Result<Config> {
    let mut builder = config::Config::builder();

    // Only load config if config file really exists
    if Path::new(&CONFIG_FILE_PATH).exists() {
        builder = builder.add_source(config::File::with_name(CONFIG_FILE_PATH));
    } else {
        log::debug!("Config file {} does not exist", CONFIG_FILE_PATH)
    }

    Config::default(builder)
}

pub fn init_logger() {
    // Ignore error here as this could be called multiple times and then a SetLoggerError will be thrown
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .env()
        .init()
        .ok();
}
