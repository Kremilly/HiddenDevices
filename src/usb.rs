extern crate winapi;

use std::{
    ptr,
    ffi::OsString,
    os::windows::ffi::OsStringExt,
};

use winapi::{
    shared::{
        minwindef::DWORD,
        ntdef::LPWSTR,
        guiddef::GUID,
    },

    um::setupapi::{
        DIGCF_PRESENT,
        SP_DEVINFO_DATA,
        SetupDiGetClassDevsW, 
        SetupDiEnumDeviceInfo, 
        SetupDiDestroyDeviceInfoList, 
        SetupDiGetDeviceInstanceIdW, 
    },
};

pub struct USB;

impl USB {

    pub fn get_devices() {
        unsafe {
            let usb_guid = GUID {
                Data1: 0x36FC9E60,
                Data2: 0xC465,
                Data3: 0x11CF,
                Data4: [0x80, 0x56, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
            };

            let hdevinfo = SetupDiGetClassDevsW(&usb_guid, ptr::null(), ptr::null_mut(), DIGCF_PRESENT);
            if hdevinfo.is_null() {
                eprintln!("Failed to get device information set");
                return;
            }

            let mut devinfo_data: SP_DEVINFO_DATA = std::mem::zeroed();
            devinfo_data.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as DWORD;

            let mut index = 0;

            while SetupDiEnumDeviceInfo(hdevinfo, index, &mut devinfo_data) != 0 {
                let device_name = Self::get_device_name(hdevinfo as *mut _, &mut devinfo_data);
                if let Some(name) = device_name {
                    if name.contains("USB") {
                        println!("Device (USB) {}: {}", index, name);
                    } else if name.contains("PCI") {
                        println!("Device (PCI) {}: {}", index, name);
                    }

                    index += 1;
                }
            }

            SetupDiDestroyDeviceInfoList(hdevinfo);
        }
    }

    unsafe fn get_device_name(hdevinfo: *mut winapi::ctypes::c_void, devinfo_data: &mut SP_DEVINFO_DATA) -> Option<String> {
        let mut buffer: [u16; 256] = [0; 256];
        let result = SetupDiGetDeviceInstanceIdW(
            hdevinfo as *mut _,
            devinfo_data,
            buffer.as_mut_ptr() as LPWSTR,
            buffer.len() as DWORD,
            ptr::null_mut(),
        );

        if result == 0 {
            return None;
        }

        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        let device_name = OsString::from_wide(&buffer[..len])
            .to_string_lossy()
            .into_owned();
        
        Some(device_name)
    }

}