use std::mem::size_of;
use std::ptr;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;

pub fn get_all_display() -> Vec<MONITORINFO> {
    let mut displays: Vec<MONITORINFO> = Vec::new();
    unsafe {
        let displays_ptr = &mut displays as *mut Vec<MONITORINFO>;
        EnumDisplayMonitors(
            ptr::null::<i32>() as HDC,
            ptr::null() as *const RECT,
            Some(enum_monitior),
            displays_ptr as LPARAM,
        );
    };
    displays
}

#[allow(unused)]
unsafe extern "system" fn enum_monitior(
    handle: HMONITOR,
    hdc: HDC,
    rect: *mut RECT,
    param: LPARAM,
) -> BOOL {
    let mut displays = param as *mut Vec<MONITORINFO>;
    let mut mi = MONITORINFO {
        cbSize: 0,
        rcMonitor: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        rcWork: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        dwFlags: 0,
    };
    mi.cbSize = size_of::<MONITORINFO>() as _;
    GetMonitorInfoA(handle, &mut mi);
    (*displays).push(mi);
    1
}