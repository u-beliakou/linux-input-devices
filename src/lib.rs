use crate::devices::{DeviceBuilder, DeviceCollection};
use reader::{InputDeviceReader};
use crate::errors::DeviceParseError;

pub mod devices;
pub mod reader;
pub mod parser;
pub mod errors;

pub const INPUT_DEVICES_PATH: &str = "/proc/bus/input/devices";

pub fn parse_devices(file_path: &str) -> Result<DeviceCollection, DeviceParseError> {
    let device_reader = InputDeviceReader::open(file_path)?;

    let mut device_collection = DeviceCollection::new();
    for device_block in device_reader {
        let device = DeviceBuilder::with_state(
            parser::into_hashmap(&device_block)?
        ).build();

        device_collection.add(device);
    }

    return Ok(device_collection);
}