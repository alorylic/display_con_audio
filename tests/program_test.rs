use DisplayConAudio::prelude::*;
use windows_sys::Win32::{UI::WindowsAndMessaging::{GetWindowTextLengthW, GetWindowTextW}, Graphics::Gdi::EnumDisplayDevicesW};

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
    let displays = get_all_display();
    println!("{}", displays.len())
}