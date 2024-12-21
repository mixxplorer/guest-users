#![deny(warnings)]
#![deny(clippy::all)]

use std::str::FromStr;

use anyhow::Context;
use clap::Parser;

#[derive(Clone, Debug)]
enum CliActions {
    Install,
    Uninstall,
}

impl FromStr for CliActions {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "install" => Ok(CliActions::Install),
            "uninstall" => Ok(CliActions::Uninstall),
            _ => Err("unable to match input to a action!"),
        }
    }
}

impl std::fmt::Display for CliActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliActions::Install => write!(f, "install"),
            CliActions::Uninstall => write!(f, "uninstall"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    log_level: clap_verbosity_flag::Verbosity,

    #[clap(
        help = "Action to perform",
        long_help = "Available actions: [install, uninstall]"
    )]
    action: CliActions,

    /// Force overwriting configuration directives, may break other integrations!
    #[clap(long, short, action)]
    force: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    simple_logger::SimpleLogger::new()
        .with_level(args.log_level.log_level().unwrap().to_level_filter())
        .with_utc_timestamps()
        .init()
        .unwrap();

    let global_settings = guest_users_lib::helper::get_config()?;

    let current_snap_extra_home_dirs = std::process::Command::new("snap")
        .arg("get")
        .arg("system")
        .arg("homedirs")
        .output()?;

    // check whether no extra home dir is set
    if current_snap_extra_home_dirs.status.code() == Some(0) {
        // home dir already set, check if it is the same as configured
        let current_home_dirs_string = String::from_utf8(current_snap_extra_home_dirs.stdout)?;
        let current_home_dirs = current_home_dirs_string.trim();
        if current_home_dirs != global_settings.home_base_path {
            log::error!("Configure snapd home dirs are '{current_home_dirs}', which is different to the guest users base home dir '{}'", global_settings.home_base_path);
            if args.force {
                log::error!("Continuing due to force arg!");
            } else {
                anyhow::bail!("Exiting as we cannot ensure this script is overwriting other important configuration! If you want really to overwrite the current setting consider the force option.");
            }
        }
    } else if current_snap_extra_home_dirs.status.code() == Some(1) {
        // no home dir currently set (this should be the default config)
        // we can set the correct home dir
    }

    match args.action {
        CliActions::Install => {
            // ensure home base directory exists as the config value setting of snap would fail otherwise
            guest_users_lib::helper::ensure_home_base_path(&global_settings)?;

            log::info!(
                "Setting snap system homedirs={}",
                global_settings.home_base_path
            );
            std::process::Command::new("snap")
                .arg("set")
                .arg("system")
                .arg(format!("homedirs={}", global_settings.home_base_path))
                .status()
                .context("Unable to set homedirs config directive for snapd!")?;
        }
        CliActions::Uninstall => {
            log::info!("Unsetting snap system homedirs");
            std::process::Command::new("snap")
                .arg("unset")
                .arg("system")
                .arg("homedirs")
                .status()
                .context("Unable to unset homedirs config directive for snapd!")?;
        }
    }

    Ok(())
}
