use nusb::transfer::{Bulk, ControlOut, ControlType, In, Out, Recipient};
use nusb::{Device, MaybeFuture, list_devices};
use std::io::{Error, ErrorKind, Read, Write};
use std::time::Duration;

struct UsbInfo {
    vid: u16,
    pid: u16,
}

#[allow(dead_code)]
fn get_info() -> Option<UsbInfo> {
    let devices = list_devices().wait().ok()?;
    devices
        .filter(|dev| {
            dev.manufacturer_string() == Some("Alienware") && dev.product_string() == Some("AW-ELC")
        })
        .map(|dev| UsbInfo {
            vid: dev.vendor_id(),
            pid: dev.product_id(),
        })
        .next()
}

#[allow(dead_code)]
fn get_descriptor(info: &UsbInfo) -> Option<Device> {
    let mut devices = list_devices().wait().ok()?;
    devices
        .find(|dev| dev.vendor_id() == info.vid && dev.product_id() == info.pid)
        .and_then(|dev| dev.open().wait().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Print all connected USB devices for manual debugging
    /// Run with: `cargo test -- --ignored --show-output`
    #[test]
    #[ignore]
    fn output_devices() {
        let devices = list_devices().wait().expect("Some Failure");
        for device in devices {
            println!("{device:?}");
        }
    }
    #[test]
    fn test_usb() {
        assert!(get_info().is_some());
    }
    #[test]
    /// Requires setting udev rules or sudo privileges to work
    fn test_get_descriptor() {
        let info = get_info().expect("Device not found");
        assert!(get_descriptor(&info).is_some());
    }
}
