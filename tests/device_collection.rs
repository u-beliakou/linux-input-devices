#[cfg(test)]
mod tests {
    use linux_input_devices::devices::{DeviceCollection};

    #[test]
    fn test_device_collection_create_happy_path() {
        let mut collection = DeviceCollection::create();
        assert_eq!(0, collection.devices.iter().count());
    }
}