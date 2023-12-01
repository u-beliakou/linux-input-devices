pub struct DeviceCollection {
    pub devices: Vec<Device>,
}

impl DeviceCollection {
    pub fn create() -> DeviceCollection {
        DeviceCollection { devices: vec![] }
    }

    pub fn iter(&self) -> std::slice::Iter<Device> {
        self.devices.iter()
    }
}

pub struct Device {}