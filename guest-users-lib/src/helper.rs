use anyhow::Context;
use nix::libc::{gid_t, uid_t};

const CONFIG_FILE_PATH: &str = "/etc/guest-users/settings.toml";

macro_rules! config_default_item {
    ( $z:expr, $a:ident, String ) => {
        $z.get_string(stringify!($a))?
    };
    ( $z:expr, $a:ident, i64 ) => {
        $z.get_int(stringify!($a))?
    };
    ( $z:expr, $a:ident, uid_t ) => {
        std::convert::TryInto::<u32>::try_into($z.get_int(stringify!($a))?)?
    };
    ( $z:expr, $a:ident, gid_t ) => {
        std::convert::TryInto::<u32>::try_into($z.get_int(stringify!($a))?)?
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
            pub fn default(mut conf: config::ConfigBuilder<config::builder::DefaultState>) -> anyhow::Result<Self> {
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
    home_base_path, String, "/home/guest-users",
    home_skel, String, "/etc/skel",
    guest_shell, String, "/bin/bash",
    public_database_path, String, "/etc/guest-users/public.db",
    uid_minimum, uid_t, 31001,
    uid_maximum, uid_t, 31999,
    gid_minimum, gid_t, 31001,
    gid_maximum, gid_t, 31999,
    guest_user_warning_app_name, String, "Guest User",
    guest_user_warning_title, String, "You are using a guest account",
    guest_user_warning_body, String, "All data will be deleted on logout. Make sure to store your data on a safe location apart from this device.",
    enable_ghost_user, bool, true,
    ghost_user_gecos_username, String, "Guest",
    ghost_user_uid, i64, 31000,
    ghost_user_gid, i64, 31000
);

pub fn get_config() -> anyhow::Result<Config> {
    let mut builder = config::Config::builder();

    // Only load config if config file really exists
    if std::path::Path::new(&CONFIG_FILE_PATH).exists() {
        builder = builder.add_source(config::File::with_name(CONFIG_FILE_PATH));
    } else {
        log::debug!("Config file {CONFIG_FILE_PATH} does not exist")
    }

    Config::default(builder)
}

pub fn init_logger() {
    // Ignore error here as this could be called multiple times and then a SetLoggerError will be thrown
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .with_utc_timestamps()
        .env()
        .init()
        .ok();
}

pub fn get_current_os_boot_id() -> anyhow::Result<String> {
    let random_boot_id = std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
        .context("Unable to read the current boot id from /proc/sys/kernel/random/boot_id")?;
    Ok(random_boot_id.trim_end_matches(['\n']).to_string())
}

/// Creates home base path if it does not exist yet and ensures correct permissions on it.
pub fn ensure_home_base_path(settings: &Config) -> anyhow::Result<()> {
    std::fs::create_dir_all(&settings.home_base_path)
        .context("Unable to create home base directory!")?;
    nix::unistd::chown(
        std::path::Path::new(&settings.home_base_path),
        Some(nix::unistd::Uid::from_raw(0)),
        Some(nix::unistd::Gid::from_raw(0)),
    )
    .context("Unable to set owner for home base directory!")?;
    std::fs::set_permissions(
        std::path::Path::new(&settings.home_base_path),
        std::os::unix::fs::PermissionsExt::from_mode(0o755),
    )
    .context("Unable to set permissions for home base directory!")?;
    Ok(())
}

/// Copies a directory and all of its contents and sets to all files a new owner but preserves the access rights.
/// Whether it preserves the access rights, sets the owner and creates the topmost directory if it does not exists
/// can be configured via `touch_topmost_directory`
pub fn copy_dir_recursive_and_set_owner(
    gsrc: impl Into<std::path::PathBuf>,
    gdst: impl Into<std::path::PathBuf>,
    uid: nix::unistd::Uid,
    gid: nix::unistd::Gid,
    touch_topmost_directory: bool,
) -> anyhow::Result<()> {
    // list of tuple (source, destination, touch_directory)
    let mut dir_queue: std::collections::LinkedList<(
        std::path::PathBuf,
        std::path::PathBuf,
        bool,
    )> = std::collections::LinkedList::new();
    dir_queue.push_back((gsrc.into(), gdst.into(), touch_topmost_directory));

    while let Some((src, dst, touch_directory)) = dir_queue.pop_front() {
        if !dst.is_dir() {
            if touch_directory {
                std::fs::create_dir(&dst)
                    .with_context(|| format!("Unable to create directory {dst:?}"))?;
            } else {
                bail!(
                    "Directory {dst:?} does not exist, but we are not allowed to create it either."
                );
            }
        }
        if touch_directory {
            nix::unistd::chown(&dst, Some(uid), Some(gid))
                .with_context(|| format!("Unable to chown {dst:?}"))?;
            std::fs::set_permissions(&dst, std::fs::metadata(&src)?.permissions())
                .with_context(|| format!("Unable to set permissions for path {dst:?}!"))?;
        }

        for entry_res in
            std::fs::read_dir(&src).with_context(|| format!("Unable to read_dir {src:?}"))?
        {
            let entry = entry_res?;
            if entry.file_type()?.is_dir() {
                dir_queue.push_back((entry.path(), dst.join(entry.file_name()), true));
            } else {
                let target = dst.join(entry.file_name());
                std::fs::copy(entry.path(), &target)?;
                nix::unistd::chown(&target, Some(uid), Some(gid))
                    .with_context(|| format!("Unable to chown {dst:?}"))?;
            }
        }
    }

    Ok(())
}

/// Returns whether a user has running/active sessions
pub fn has_active_user_sessions(user_name: &str) -> anyhow::Result<bool> {
    let utmp_entries =
        utmp_rs::parse_from_path("/var/run/utmp").context("Parsing /var/run/utmp failed!")?;
    let has_session = utmp_entries
        .iter()
        .filter(|entry| matches!(entry, utmp_rs::UtmpEntry::UserProcess { .. }))
        .map(|entry| {
            if let utmp_rs::UtmpEntry::UserProcess { user, .. } = entry {
                user.as_str()
            } else {
                panic!("Invalid utmp entry found after filtering!")
            }
        })
        .any(|user_name_proc| user_name_proc == user_name);
    Ok(has_session)
}
