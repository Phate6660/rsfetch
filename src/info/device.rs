use log::error;
use std::fs::{metadata, read_to_string};

pub fn device() -> String {
    if metadata("/sys/devices/virtual/dmi/id/product_name").is_ok() {
        read_to_string("/sys/devices/virtual/dmi/id/product_name").unwrap()
    } else if metadata("/sys/firmware/devicetree/base/model").is_ok() {
        read_to_string("/sys/firmware/devicetree/base/model").unwrap()
    } else {
        error!("Could not obtain the name of the device.");
        "N/A".to_string()
    }
}
