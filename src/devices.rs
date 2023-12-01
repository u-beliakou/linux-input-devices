pub struct DeviceCollection {
    pub devices: Vec<Device>,
}

impl DeviceCollection {
    pub fn create() -> DeviceCollection {
        DeviceCollection { devices: vec![] }
    }
}

pub struct Device {}