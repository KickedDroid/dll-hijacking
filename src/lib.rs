#![cfg(windows)]
use std::ptr::null_mut;
use std::process::Command;
use winapi::um::winuser::MessageBoxA;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]

pub extern "C" fn voteforpedro() {
    unsafe {
        Command::new("cmd.exe")
        .arg("/c")
        .arg("")
    }
}

//#[no_mangle]
//#[allow(non_snake_case, unused_variables)]
// fn DllMain(
//     dll_module: HINSTANCE,
//     call_reason: DWORD,
//     reserved: LPVOID)
//     -> BOOL
// {
//     const DLL_PROCESS_ATTACH: DWORD = 1;
//     const DLL_PROCESS_DETACH: DWORD = 0;

//     match call_reason {
//         DLL_PROCESS_ATTACH => voteforpedro(),
//         DLL_PROCESS_DETACH => (),
//         _ => ()
//     }
//     TRUE
// }