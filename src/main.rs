extern crate winapi;

pub mod usb;

use usb::USB;

fn main() {
    USB::get_devices();
}