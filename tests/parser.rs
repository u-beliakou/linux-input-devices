#[cfg(test)]
mod test_input_device_parser {

    use linux_input_devices::parser::{
        DV_BITMAP_KEY, DV_PROP_BUS, DV_PROP_NAME,
        DV_PROP_PRODUCT, DV_PROP_VENDOR, DV_PROP_VERSION, into_hashmap
    };

    #[test]
    fn into_hashmap_happy_path() {
        let device_vector: Vec<String> = vec![
            String::from("I: Bus=0020 Vendor=0000 Product=0004 Version=1010"),
            String::from("B: KEY=ffff0000 0 0 0 0"),
            String::from("N: Name=\"Sleep Button\"")
        ];
        let hashmap = into_hashmap(&device_vector);

        assert_eq!(6, hashmap.len());

        assert_eq!("0020", hashmap.get(DV_PROP_BUS).unwrap());
        assert_eq!("0000", hashmap.get(DV_PROP_VENDOR).unwrap());
        assert_eq!("0004", hashmap.get(DV_PROP_PRODUCT).unwrap());
        assert_eq!("1010", hashmap.get(DV_PROP_VERSION).unwrap());
        assert_eq!("ffff0000 0 0 0 0", hashmap.get(DV_BITMAP_KEY).unwrap());
        assert_eq!("Sleep Button", hashmap.get(DV_PROP_NAME).unwrap());
    }
}