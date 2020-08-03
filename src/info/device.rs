use std::fs::{metadata, read_to_string};

pub fn device() -> String {
    if metadata("/sys/devices/virtual/dmi/id/product_name").is_ok() {
        read_to_string("/sys/devices/virtual/dmi/id/product_name").unwrap()
    } else if metadata("/sys/firmware/devicetree/base/model").is_ok() {
        read_to_string("/sys/firmware/devicetree/base/model").unwrap()
    } else {
        "N/A (could not obtain name of device)".to_string()
    }
}
