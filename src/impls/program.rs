use std::collections::HashMap;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::{HMONITOR, MonitorFromWindow, MONITOR_DEFAULTTONEAREST};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetParent, GetWindowTextLengthW, IsWindowVisible,
};

/// 获取所有具有窗口，且窗口名称长度不为零的应用句柄
pub fn get_all_windows_program_hwdn() -> Vec<HWND> {
    let mut all_window: Vec<HWND> = vec![];
    unsafe {
        let all_window_point = &mut all_window as *mut Vec<HWND>;
        EnumWindows(Some(enum_windows), all_window_point as LPARAM);
    }
    all_window
}

unsafe extern "system" fn enum_windows(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let all_window = lparam as *mut Vec<HWND>;
    let len = GetWindowTextLengthW(hwnd);
    if len <= 0 || IsWindowVisible(hwnd) == 0 || GetParent(hwnd) != 0 {
        return 1;
    }
    (*all_window).push(hwnd);
    1
}

/// 将给定应用按照所在显示器分类
pub fn classify_program_window(programs: Vec<HWND>) -> HashMap<HMONITOR, Vec<HWND>> {
    let mut map = HashMap::new();
    for program in programs {
        unsafe {
            let hmonitor = MonitorFromWindow(program, MONITOR_DEFAULTTONEAREST);
            map.entry(hmonitor).or_insert(vec![]).push(program);
        }
    }
    return map;
} 