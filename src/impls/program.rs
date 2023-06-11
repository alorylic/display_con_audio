use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetParent, IsWindowVisible, GetWindowTextLengthW,
};

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
