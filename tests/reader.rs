#[cfg(test)]
mod test_input_device_reader {
    use linux_input_devices::reader::InputDeviceReader;
    #[test]
    fn open_returns_error_on_file_does_not_exist() {
        let actual = InputDeviceReader::open("/something/not/existing");
        assert!(actual.is_err());
    }

    #[test]
    fn open_happy_path() {
        let actual = InputDeviceReader::open("./assets/example.txt");
        assert!(actual.is_ok());
    }

    #[test]
    fn iterator_reads_all_lines_and_groups_into_blocks_correctly() {
        let mut count_of_filled_lines = 0;
        let mut count_of_blocks = 0;

        let actual = InputDeviceReader::open("./assets/example.txt").unwrap();
        for block in actual {
            count_of_filled_lines += block.iter().count();
            count_of_blocks += 1;
        }

        assert_eq!(2, count_of_blocks);
        assert_eq!(28, count_of_filled_lines);
    }
}