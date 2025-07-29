use nusb::transfer::{Bulk, ControlOut, ControlType, In, Out, Recipient};
use nusb::{MaybeFuture, list_devices};
use std::io::{Error, ErrorKind, Read, Write};
use std::time::Duration;

struct UsbInfo {
    vid: u16,
    pid: u16,
}

#[allow(dead_code)]
fn get_usb() -> Option<UsbInfo> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usb() {
        assert!(get_usb().is_some());
    }
    /// Print all connected USB devices for manual debugging
    /// Run with: `cargo test -- --ignored --show-output`
    #[test]
    fn output_devices() {
        let devices = list_devices().wait().expect("Some Failure");
        for device in devices {
            println!("{device:?}");
        }
    }
}
