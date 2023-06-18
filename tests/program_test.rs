use windows_sys::Win32::{
    Graphics::Gdi::EnumDisplayDevicesW,
    UI::WindowsAndMessaging::{GetWindowTextLengthW, GetWindowTextW},
};
use DisplayConAudio::prelude::*;

#[test]
fn program() {
    let programs = get_all_windows_program_hwdn();
    let mut names = vec![];
    unsafe {
        for program in programs {
            let len = GetWindowTextLengthW(program);
            let mut name = vec![0_u16; len as usize + 1];
            GetWindowTextW(program, name.as_mut_ptr(), len + 1);
            names.push(String::from_utf16_lossy(&name));
        }
    }
    names.into_iter().for_each(|it| println!("{it}"))
}

#[test]
fn display() {
    let displays = get_hmonitor_to_monitorinfo();
    for display in displays.values() {
        println!("{}", String::from_utf16(&display.szDevice).unwrap());
    }
}

#[test]
fn classify() {
    let programs = get_all_windows_program_hwdn();
    let monitor_map = get_hmonitor_to_monitorinfo();
    let classify = classify_program_window(programs);
    for (hmonitor, hwnds) in classify {
        let monitor_info = monitor_map.get(&hmonitor).unwrap();
        let monitor_name = String::from_utf16_lossy(&monitor_info.szDevice);
        println!("{monitor_name}");
        for hwnd in hwnds {
            unsafe {
                let len = GetWindowTextLengthW(hwnd);
                let mut pro_name = vec![0_u16; len as usize + 1];
                GetWindowTextW(hwnd, pro_name.as_mut_ptr(), len + 1);
                let pro_name = String::from_utf16_lossy(&pro_name);
                println!("    {pro_name}");
            }
        }
    }
}