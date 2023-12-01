use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub struct InputDeviceReader {
    reader: BufReader<File>,
}

impl InputDeviceReader {
    pub fn open(file_path: &str) -> Result<InputDeviceReader> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        Ok(InputDeviceReader { reader })
    }
}

impl Iterator for InputDeviceReader {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut device_block: Vec<String> = vec![];
        let mut buf = String::new();

        while self.reader.read_line(&mut buf).map(|len| len > 1).unwrap() {
            device_block.push(buf.clone());
            buf.clear();
        }

        if device_block.len() > 0 {
            return Some(device_block);
        }

        return None;
    }
}