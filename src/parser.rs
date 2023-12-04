use std::collections::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::errors::DeviceParseError;

static REGEXP_IDENTIFIER_LINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Bus=(?P<bus>[\w\d]{4}) Vendor=(?P<vendor>[\w\d]{4}) Product=(?P<product>[\w\d]{4}) Version=(?P<version>[\w\d]{4})").unwrap()
});

static REGEXP_SINGLE_VALUE_LINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<key>[\w]{1,})=(?P<value>[\W\w]{0,})$").unwrap()
});

type ParseResult<T> = Result<T, DeviceParseError>;

pub const DV_ID_LINE_PREFIX: char = 'I';
pub const DV_PROP_BUS: &'static str = "bus";
pub const DV_PROP_VENDOR: &'static str = "vendor";
pub const DV_PROP_PRODUCT: &'static str = "product";
pub const DV_PROP_VERSION: &'static str = "version";
pub const DV_PROP_NAME: &'static str = "Name";
pub const DV_PROP_SYSFS: &'static str = "Sysfs";
pub const DV_PROP_PHYS: &'static str = "Phys";
pub const DV_PROP_UNIQ: &'static str = "Uniq";
pub const DV_PROP_HANDLERS: &'static str = "Handlers";

pub const DV_BITMAP_PROP: &'static str = "PROP";
pub const DV_BITMAP_EV: &'static str = "EV";
pub const DV_BITMAP_KEY: &'static str = "KEY";
pub const DV_BITMAP_REL: &'static str = "REL";
pub const DV_BITMAP_ABS: &'static str = "ABS";
pub const DV_BITMAP_MSC: &'static str = "MSC";
pub const DV_BITMAP_LED: &'static str = "LED";
pub const DV_BITMAP_SND: &'static str = "SND";
pub const DV_BITMAP_FF: &'static str = "FF";
pub const DV_BITMAP_SW: &'static str = "SW";

const IDENTIFIER_LINE_KEYS: [&str; 4] = [DV_PROP_BUS, DV_PROP_VENDOR, DV_PROP_PRODUCT, DV_PROP_VERSION];

pub fn into_hashmap(lines: &Vec<String>) -> ParseResult<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in lines {
        parse_line_into_hashmap(&line, &mut map)?;
    }

    Ok(map)
}

fn parse_line_into_hashmap(line: &String, map: &mut HashMap<String, String>) -> ParseResult<()> {
    if is_identifier_line(&line) {
        parse_identifier_line_into_hashmap(&line, map)?;
    } else {
        parse_single_value_line_into_hashmap(&line, map)?;
    }

    Ok(())
}

fn is_identifier_line(line: &String) -> bool {
    line
        .chars().nth(0)
        .is_some_and(|char| char == DV_ID_LINE_PREFIX)
}

fn parse_identifier_line_into_hashmap(line: &String, map: &mut HashMap<String, String>) -> ParseResult<()> {
    let caps = REGEXP_IDENTIFIER_LINE
        .captures(line)
        .ok_or(DeviceParseError::FormatError)?;

    for key in IDENTIFIER_LINE_KEYS {
        map.insert(String::from(key), standardize_value(&caps[key]));
    }

    Ok(())
}

fn parse_single_value_line_into_hashmap(line: &String, map: &mut HashMap<String, String>) -> ParseResult<()> {
    let caps = REGEXP_SINGLE_VALUE_LINE
        .captures(line)
        .ok_or(DeviceParseError::FormatError)?;

    map.insert(caps["key"].to_string(), standardize_value(&caps["value"]));

    Ok(())
}

fn standardize_value(value: &str) -> String {
    String::from(value.trim().trim_matches('"'))
}