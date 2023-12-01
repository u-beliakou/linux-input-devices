#[cfg(test)]
mod tests {
    use linux_input_devices::devices::{Device, DeviceCollection};

    #[test]
    fn test_device_collection_create_happy_path() {
        let collection = DeviceCollection::create();
        assert_eq!(0, collection.iter().count());
    }

    #[test]
    fn test_device_collection_add_happy_path() {
        let mut collection = DeviceCollection::create();
        collection.add(create_dummy_device());

        assert_eq!(1, collection.iter().count());
    }

    fn create_dummy_device() -> Device {
        return Device::default();
    }
}