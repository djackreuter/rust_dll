use std::{ffi::c_void};
use windows::{Win32::{Foundation::{HINSTANCE, BOOL, HWND}, System::SystemServices::DLL_PROCESS_ATTACH, UI::WindowsAndMessaging::{MessageBoxA, MB_OK}}, s};

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(_hinst: HINSTANCE, fdwReason: u32, _lpvReserved: c_void) -> BOOL {
    match fdwReason {
        DLL_PROCESS_ATTACH => msg_box(),
        _ => ()
    }
    BOOL(1)
}

fn msg_box() {
    unsafe {
        MessageBoxA(
            HWND(0),
            s!("Hello world"),
            s!("Success"),
            MB_OK
        );
    }
}