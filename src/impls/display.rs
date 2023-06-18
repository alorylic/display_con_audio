use std::collections::HashMap;
use std::mem::size_of;
use std::ptr;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;

/// 获取监视器句柄和监视器信息的影视关系
pub fn get_hmonitor_to_monitorinfo() -> HashMap<HMONITOR, MONITORINFOEXW> {
    let mut displays: HashMap<HMONITOR, MONITORINFOEXW> = HashMap::new();
    unsafe {
        let displays_ptr = &mut displays as *mut HashMap<HMONITOR, MONITORINFOEXW>;
        EnumDisplayMonitors(
            ptr::null::<i32>() as HDC,
            ptr::null() as *const RECT,
            Some(enum_monitior),
            displays_ptr as LPARAM,
        );
    };
    displays
}

/// 回调函数，遍历监视器，建立映射表
#[allow(unused)]
unsafe extern "system" fn enum_monitior(
    handle: HMONITOR,
    hdc: HDC,
    rect: *mut RECT,
    param: LPARAM,
) -> BOOL {
    let mut displays = param as *mut HashMap<HMONITOR, MONITORINFOEXW>;
    let mut mi = MONITORINFOEXW {
        monitorInfo: MONITORINFO {
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
        },
        szDevice: [0_u16; 32],
    };
    mi.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as _;
    GetMonitorInfoW(handle, &mut mi.monitorInfo as _);
    (*displays).insert(handle, mi);
    1
}