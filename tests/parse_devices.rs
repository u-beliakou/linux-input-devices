#[cfg(test)]
mod test_parse_devices {
    use linux_input_devices::parse_devices;

    #[test]
    fn happy_path() {
        let collection = parse_devices("./assets/output_example.txt");
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
