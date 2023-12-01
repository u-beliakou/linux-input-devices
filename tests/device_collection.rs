#[cfg(test)]
mod test_device_collection {
    use linux_input_devices::devices::{Device, DeviceCollection};

    #[test]
    fn create_happy_path() {
        let collection = DeviceCollection::new();
        assert_eq!(0, collection.iter().count());
    }

    #[test]
    fn add_happy_path() {
        let mut collection = DeviceCollection::new();
        collection.add(Device::default());

        assert_eq!(1, collection.iter().count());
    }

    #[test]
    fn get_returns_none() {
        let collection = DeviceCollection::new();
        assert!(collection.get(2).is_none());
    }

    #[test]
    fn get_happy_path() {
        let mut collection = DeviceCollection::new();
        collection.add(create_dummy_device("Expected name"));

        assert_eq!("Expected name", collection.get(0).unwrap().name);
    }

    #[test]
    fn find_one_by_name_returns_none() {
        let mut collection = DeviceCollection::new();
        collection.add(create_dummy_device("Noise"));

        let actual = collection.find_one_by_name("Expected");
        assert!(actual.is_none());
    }

    #[test]
    fn find_one_by_name_happy_path() {
        let mut collection = DeviceCollection::new();

        collection.add(create_dummy_device("Noise"));
        collection.add(create_dummy_device("Expected"));

        let found_device = collection.find_one_by_name("Expected");
        assert_eq!("Expected", found_device.unwrap().name);
    }

    fn create_dummy_device(name: &str) -> Device {
        let mut device = Device::default();
        device.name = String::from(name);

        device
    }
}