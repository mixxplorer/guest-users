pub struct Gecos {
    pub full_name: Option<String>,   // like Guest
    pub room_number: Option<String>, // like 874
    pub work_phone: Option<String>,  // like 574
    pub home_phone: Option<String>,  // like +491606799999
    pub other: Option<String>,       // like a mail address or other important information
}

macro_rules! gecos_string_sanitize {
    ($sts:expr) => {
        if let Some(unpacked) = $sts.clone() {
            if unpacked.contains(",") {
                log::warn!(
                    "{unpacked} does contain a ',', which is not allowed in gecos strings, omitting it..."
                );
            }
            unpacked.replace(",", "")
        } else {
            "".to_string()
        }
    };
}

impl Gecos {
    pub fn to_gecos_string(&self) -> String {
        format!(
            "{},{},{},{},{}",
            gecos_string_sanitize!(self.full_name),
            gecos_string_sanitize!(self.room_number),
            gecos_string_sanitize!(self.work_phone),
            gecos_string_sanitize!(self.home_phone),
            gecos_string_sanitize!(self.other),
        )
    }
}
