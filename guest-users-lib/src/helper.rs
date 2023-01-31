use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

const CONFIG_FILE_PATH: &str = "/etc/guest-users/settings.toml";

macro_rules! config_default_item {
    ( $z:expr, $a:ident, String ) => {
        $z.get_string(stringify!($a))?
    };
    ( $z:expr, $a:ident, i64 ) => {
        $z.get_int(stringify!($a))?
    };
    ( $z:expr, $a:ident, bool ) => {
        $z.get_bool(stringify!($a))?
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
    guest_username_human_readable_prefix, String, "Guest",
    guest_group_name_prefix, String, "guest",
    home_base_path, String, "/tmp/guest-users-home",
    home_skel, String, "/etc/skel",
    guest_shell, String, "/bin/bash",
    public_database_path, String, "/etc/guest-users/public.db",
    uid_minimum, i64, 31001,
    uid_maximum, i64, 31999,
    gid_minimum, i64, 31001,
    gid_maximum, i64, 31999,
    guest_user_warning_title, String, "You are using a guest account",
    guest_user_warning_body, String, "All data will be deleted on logout. Make sure to store your data on a safe location apart from this device.",
    enable_ghost_user, bool, true,
    ghost_user_gecos_username, String, "Guest",
    ghost_user_uid, i64, 31000,
    ghost_user_gid, i64, 31000
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

pub fn get_current_os_boot_id() -> Result<String> {
    let random_boot_id = std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
        .context("Unable to read the current boot id from /proc/sys/kernel/random/boot_id")?;
    Ok(random_boot_id.trim_end_matches(&['\n']).to_string())
}

pub fn copy_dir_recursive_and_set_owner(
    gsrc: impl Into<std::path::PathBuf>,
    gdst: impl Into<std::path::PathBuf>,
    uid: nix::unistd::Uid,
    gid: nix::unistd::Gid,
) -> Result<()> {
    let mut dir_queue: std::collections::LinkedList<(std::path::PathBuf, std::path::PathBuf)> =
        std::collections::LinkedList::new();
    dir_queue.push_back((gsrc.into(), gdst.into()));

    while let Some((src, dst)) = dir_queue.pop_front() {
        if !dst.is_dir() {
            fs::create_dir(&dst).with_context(|| format!("Unable to create directory {dst:?}"))?;
        }
        nix::unistd::chown(&dst, Some(uid), Some(gid))
            .with_context(|| format!("Unable to chown {dst:?}"))?;
        fs::set_permissions(&dst, fs::metadata(&src)?.permissions())
            .with_context(|| format!("Unable to set permissions for path {dst:?}!"))?;

        for entry_res in
            fs::read_dir(&src).with_context(|| format!("Unable to read_dir {src:?}"))?
        {
            let entry = entry_res?;
            if entry.file_type()?.is_dir() {
                dir_queue.push_back((entry.path(), dst.join(entry.file_name())));
            } else {
                let target = dst.join(entry.file_name());
                fs::copy(entry.path(), &target)?;
                nix::unistd::chown(&target, Some(uid), Some(gid))
                    .with_context(|| format!("Unable to chown {dst:?}"))?;
            }
        }
    }

    Ok(())
}
