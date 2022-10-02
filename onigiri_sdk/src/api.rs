//! API calls

trait Device {
    fn get_id() -> String;
}

pub struct LCDDevice {
    id: String,
}

impl LCDDevice {
    pub fn new(id: &str) -> Result<Self, anyhow::Error> {
        Ok(LCDDevice { id: id.to_owned() })
    }

    pub fn write_line(line: u8) {}

    pub fn clear() {}
}

pub struct LEDDevice {}

impl LEDDevice {}

// general api calls
pub fn get_devices() {}
