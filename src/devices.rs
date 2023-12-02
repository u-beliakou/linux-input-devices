use std::collections::HashMap;
use crate::parser::{
    DV_BITMAP_ABS, DV_BITMAP_EV, DV_BITMAP_FF, DV_BITMAP_KEY, DV_BITMAP_LED,
    DV_BITMAP_MSC, DV_BITMAP_PROP, DV_BITMAP_REL, DV_BITMAP_SND, DV_BITMAP_SW, DV_PROP_BUS,
    DV_PROP_HANDLERS, DV_PROP_NAME, DV_PROP_PHYS, DV_PROP_PRODUCT, DV_PROP_SYSFS,
    DV_PROP_UNIQ, DV_PROP_VENDOR, DV_PROP_VERSION
};


#[derive(Debug)]
pub struct DeviceCollection {
    devices: Vec<Device>,
}

impl DeviceCollection {
    pub fn new() -> DeviceCollection {
        DeviceCollection { devices: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<Device> {
        self.devices.iter()
    }

    pub fn add(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn get(&self, idx: usize) -> Option<&Device> {
        self.devices.get(idx)
    }

    pub fn find_one_by_name(&self, device_name: &str) -> Option<&Device> {
        self.iter().find(|&device| device.name.contains(device_name))
    }
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct Device {
    pub identifier: DeviceIdentifier,
    pub name: String,
    pub sysfs: String,
    pub phys: String,
    pub uniq: String,
    pub handlers: String,
    pub bitmaps: DeviceBitmaps,
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct DeviceIdentifier
{
    pub bus: String,
    pub vendor: String,
    pub product: String,
    pub version: String,
}

#[allow(unused)]
#[derive(Default, Debug)]
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

#[derive(Default)]
pub struct DeviceBuilder {
    state: HashMap<String, String>,
}

impl DeviceBuilder {

    pub fn with_state(hashmap: HashMap<String, String>) -> DeviceBuilder {
        DeviceBuilder {
            state: hashmap,
        }
    }

    pub fn build_with_name(name: &str) -> Device {
        let mut builder = DeviceBuilder::default();
        builder.set(DV_PROP_NAME, name);

        builder.build()
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.state.insert(String::from(key), String::from(value));
    }

    pub fn build(&self) -> Device {
        Device {
            identifier: DeviceIdentifier {
                bus: self.get_prop(DV_PROP_BUS),
                vendor: self.get_prop(DV_PROP_VENDOR),
                product: self.get_prop(DV_PROP_PRODUCT),
                version: self.get_prop(DV_PROP_VERSION),
            },
            name: self.get_prop(DV_PROP_NAME),
            sysfs: self.get_prop(DV_PROP_SYSFS),
            phys: self.get_prop(DV_PROP_PHYS),
            uniq: self.get_prop(DV_PROP_UNIQ),
            handlers: self.get_prop(DV_PROP_HANDLERS),
            bitmaps: DeviceBitmaps {
                prop: self.get_prop(DV_BITMAP_PROP),
                ev: self.get_prop(DV_BITMAP_EV),
                key: self.get_prop(DV_BITMAP_KEY),
                rel: self.get_prop(DV_BITMAP_REL),
                abs: self.get_prop(DV_BITMAP_ABS),
                msc: self.get_prop(DV_BITMAP_MSC),
                led: self.get_prop(DV_BITMAP_LED),
                snd: self.get_prop(DV_BITMAP_SND),
                ff: self.get_prop(DV_BITMAP_FF),
                sw: self.get_prop(DV_BITMAP_SW),
            },
        }
    }

    fn get_prop(&self, key: &str) -> String {
        self.state.get(key).cloned().unwrap_or_default()
    }
}