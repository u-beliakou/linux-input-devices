#[cfg(test)]
mod test_device_collection {
    use linux_input_devices::devices::{Device, DeviceBuilder, DeviceCollection};

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
        collection.add(
            DeviceBuilder::build_with_name("Expected name")
        );

        assert_eq!("Expected name", collection.get(0).unwrap().name);
    }

    #[test]
    fn find_one_by_name_returns_none() {
        let mut collection = DeviceCollection::new();
        collection.add(
            DeviceBuilder::build_with_name("Noise")
        );

        let actual = collection.find_one_by_name("Expected");
        assert!(actual.is_none());
    }

    #[test]
    fn find_one_by_name_happy_path() {
        let mut collection = DeviceCollection::new();

        collection.add(
            DeviceBuilder::build_with_name("Noise")
        );
        collection.add(
            DeviceBuilder::build_with_name("Expected")
        );

        let found_device = collection.find_one_by_name("Expected");
        assert_eq!("Expected", found_device.unwrap().name);
    }
}