#[cfg(test)]
mod tests {
    use linux_input_devices::parse_devices;

    #[test]
    fn test_parse_devices_happy_path() {
        let collection = parse_devices("./assets/output_example.txt");
        assert_eq!(collection.devices.iter().count(), 2);
    }
}
