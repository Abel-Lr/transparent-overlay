use std::ptr::null_mut as NULL;
use winapi::um::winuser;

#[tauri::command]
pub fn create_warning_window() {
    let l_msg: Vec<u16> = "Invalid URL to parse\0".encode_utf16().collect();
    let l_title: Vec<u16> = "Error parsing URL\0".encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(
            NULL(),
            l_msg.as_ptr(),
            l_title.as_ptr(),
            winuser::MB_OK | winuser::MB_ICONEXCLAMATION,
        );
    }
}
