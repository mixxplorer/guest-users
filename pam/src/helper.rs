use utmp_rs::{parse_from_path, UtmpEntry};

use anyhow::Context;
use anyhow::Result;

pub fn has_active_user_sessions(user_name: &str) -> Result<bool> {
    let utmp_entries = parse_from_path("/var/run/utmp").context("Parsing /var/run/utmp failed!")?;
    let has_session = utmp_entries
        .iter()
        .filter(|entry| matches!(entry, UtmpEntry::UserProcess { .. }))
        .map(|entry| {
            if let UtmpEntry::UserProcess { user, .. } = entry {
                user.as_str()
            } else {
                panic!("Invalid utmp entry found after filtering!")
            }
        })
        .any(|user_name_proc| user_name_proc == user_name);
    Ok(has_session)
}
