use std::ptr;
use std::ffi::{CString, CStr};
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_TERMINATE};
use winapi::um::winnt::{HANDLE, LPCWSTR};
use winapi::um::winuser::MessageBoxW;
use winapi::shared::minwindef::{DWORD, FALSE};

fn win_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn main() {
    let pid = 26508; // the process ID you want to terminate
    // get the process handle
    /*
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_TERMINATE, FALSE, pid as DWORD) };
    if handle.is_null() {
        panic!("Failed to open process");
    }
    // terminate the process
    let result = unsafe { TerminateProcess(handle, 0) };
    if result == FALSE {
        panic!("Failed to terminate process");
    }
    */
    let text: Vec<u16> = win_str("Hello World");
    let caption: Vec<u16> = win_str("Greetings");
    let result = unsafe { MessageBoxW(ptr::null_mut(), text.as_ptr(), caption.as_ptr(), 1)};
    if result == FALSE {
        panic!("Failed to show message box");
    }
}
