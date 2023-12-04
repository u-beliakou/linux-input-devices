#[cfg(test)]
mod test_parse_devices {
    use std::{io, mem};
    use std::io::Error;
    use linux_input_devices::errors::DeviceParseError;
    use linux_input_devices::parse_devices;

    #[test]
    fn returns_error_on_file_does_not_exist() {
        let error = parse_devices("/something/not/existing").unwrap_err();

        let expected = DeviceParseError::IoError (
            Error::new(io::ErrorKind::NotFound, "No such file or directory")
        );
        assert_eq!(mem::discriminant(&expected), mem::discriminant(&error));
        assert_eq!("No such file or directory (os error 2)", error.to_string());
    }

    #[test]
    fn happy_path() {
        let result = parse_devices("./assets/example.txt");

        assert!(result.is_ok());

        let collection = result.unwrap();
        assert_eq!(collection.iter().count(), 2);

        let device = collection.get(0).unwrap();
        assert_eq!(device.identifier.bus, "0020");
        assert_eq!(device.identifier.vendor, "0000");
        assert_eq!(device.identifier.product, "0004");
        assert_eq!(device.identifier.version, "0000");
        assert_eq!(device.name, "Sleep Button");
        assert_eq!(device.phys, "PNP0C1E/button/input0");
        assert_eq!(device.sysfs, "/devices/LNXSYSTM:00/LNXSYBUS:00/PNP0C0E:00/input/input0");
        assert_eq!(device.uniq, "test");
        assert_eq!(device.handlers, "kbd event0");

        assert_eq!(device.bitmaps.prop, "0");
        assert_eq!(device.bitmaps.ev, "14");
        assert_eq!(device.bitmaps.key, "ffff0000 0 0 0 0");
        assert_eq!(device.bitmaps.rel, "1943");
        assert_eq!(device.bitmaps.abs, "100000000");
        assert_eq!(device.bitmaps.msc, "10");
        assert_eq!(device.bitmaps.led, "1f");
        assert_eq!(device.bitmaps.snd, "0");
        assert_eq!(device.bitmaps.ff, "10");
        assert_eq!(device.bitmaps.sw, "1");
    }
}
