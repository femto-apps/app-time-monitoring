use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::shared::windef::HWND__;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HINSTANCE__;

use winapi::um::winuser::GetForegroundWindow;
use winapi::um::winuser::GetWindowTextLengthW;
use winapi::um::winuser::GetWindowTextW;
use winapi::um::winuser::GetWindowModuleFileNameW;
use winapi::um::winuser::GetWindowThreadProcessId;

use winapi::um::libloaderapi::GetModuleFileNameW;

const WINDOWS_PATH_SIZE: u32 = 260;


/* WINDOWS HELPERS */

pub fn get_string_from_win_vec(v : Vec<u16>) -> String {
	OsString::from_wide(&v).to_string_lossy().into_owned()
}

pub fn get_windows_vec(length: i32) -> Vec<u16> {
	vec![0u16; (length) as usize]
}

/* WINDOWS APIs */

pub fn get_foreground_window() -> *mut HWND__ {
    unsafe { GetForegroundWindow() }
}

pub fn get_foreground_window_title() -> String {
    unsafe {
        let whdl = get_foreground_window();

        let length = GetWindowTextLengthW(whdl);
        let mut title = get_windows_vec(length);

        GetWindowTextW(whdl, title.as_mut_ptr(), length);
        get_string_from_win_vec(title)
	}
}

pub fn get_foreground_window_path() -> String {
    unsafe {
        let whdl = get_foreground_window();

        let mut title = get_windows_vec(WINDOWS_PATH_SIZE as i32);
        GetWindowModuleFileNameW(whdl, title.as_mut_ptr(), WINDOWS_PATH_SIZE);

        get_string_from_win_vec(title)
    }
}

pub fn get_module_file_name_ex() -> String {
    unsafe {
        let whdl = get_foreground_window();
        let mut pid : DWORD = 0;
        
        if GetWindowThreadProcessId(whdl, &mut pid as *mut DWORD) != 0 {
            // We found a window...

            let mut title = get_windows_vec(WINDOWS_PATH_SIZE as i32);
            GetModuleFileNameW(HWND__::from(whdl), title.as_mut_ptr(), WINDOWS_PATH_SIZE);

            return get_string_from_win_vec(title);
        }

        "".to_string()
    }
}