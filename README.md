# Linux input devices

The repository is a Rust library designed to simplify parsing of such virtual files as `/proc/bus/input/devices`.

## Usage

```rust
use linux_input_devices::{parse_devices, INPUT_DEVICES_PATH, DeviceParseError};

fn main() {
    let result = parse_devices(INPUT_DEVICES_PATH);

    match result {
        Err(DeviceParseError::FormatError)
            => panic!("Something wrong with output format"),
        Err(DeviceParseError::IoError(_))
            => panic!("The file or resource is not found"),
        Ok(collection) => {
            for device in collection.iter() {
                println!("{:?}", device);
            }
        }
    }
}
```

## Constants
| Constant | Value |
|----------|-------|
|INPUT_DEVICES_PATH|/proc/bus/input/devices|

## Structs
### `DeviceCollection`

| Method                                      | Description                                                                   |
|---------------------------------------------|-------------------------------------------------------------------------------|
| `new() -> DeviceCollection`                 | Creates a new `DeviceCollection` instance.                                   |
| `iter() -> std::slice::Iter<Device>`        | Returns an iterator over the devices in the collection.                      |
| `add(device: Device)`                        | Adds a device to the collection.                                              |
| `get(idx: usize) -> Option<&Device>`        | Returns a reference to the device at the specified index, or `None` if the index is out of bounds. |
| `find_one_by_name(device_name: &str) -> Option<&Device>` | Finds and returns a reference to the first device in the collection whose name contains the specified substring. |

### `Device`

```rust
pub struct Device {
    pub identifier: DeviceIdentifier,
    pub name: String,
    pub sysfs: String,
    pub phys: String,
    pub uniq: String,
    pub handlers: String,
    pub bitmaps: DeviceBitmaps,
}
```

### `DeviceIdentifier`
```rust
pub struct DeviceIdentifier
{
    pub bus: String,
    pub vendor: String,
    pub product: String,
    pub version: String,
}
```

### `DeviceBitmaps`
```rust
pub struct DeviceBitmaps {
    pub prop: String,
    pub ev: String,
    pub key: String,
    pub rel: String,
    pub abs: String,
    pub msc: String,
    pub led: String,
    pub snd: String,
    pub ff: String,
    pub sw: String,
}
```