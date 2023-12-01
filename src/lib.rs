use crate::devices::DeviceCollection;

pub mod devices;

pub const INPUT_DEVICES_PATH: &str = "/proc/bus/input/devices";

#[allow(unused)]
pub fn parse_devices(file_path: &str) -> DeviceCollection {
    DeviceCollection::new()
}