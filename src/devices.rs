pub struct DeviceCollection {
    pub devices: Vec<Device>,
}

impl DeviceCollection {
    pub fn new() -> DeviceCollection {
        DeviceCollection { devices: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<Device> {
        self.devices.iter()
    }

    pub fn add(&mut self, device: Device) {
        self.devices.push(device);
    }
}

#[allow(unused)]
#[derive(Default)]
pub struct Device {
    pub identifier: DeviceIdentifier,
    pub name: String,
    pub sysfs: String,
    pub phys: String,
    pub uniq: String,
    pub handlers: String,
    pub bitmaps: DeviceBitmaps,
}

#[allow(unused)]
#[derive(Default)]
pub struct DeviceIdentifier
{
    pub bus: String,
    pub vendor: String,
    pub product: String,
    pub version: String,
}

#[allow(unused)]
#[derive(Default)]
pub struct DeviceBitmaps {
    pub prop: String,
    pub ev: String,
    pub key: String,
    pub rel: String,
    pub abs: String,
    pub msc: String,
    pub led: String,
    pub snd: String,
    pub ff: String,
    pub sw: String,
}