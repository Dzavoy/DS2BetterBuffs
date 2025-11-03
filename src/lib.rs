#![allow(non_snake_case)]

use dll_proxy::proxy_dll;
use std::thread;
use std::time::Duration;

mod logic;
use logic::apply;

proxy_dll!("dinput8.dll");

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(hinstDLL: usize, dwReason: u32, _lpReserved: *mut usize) -> i32 {
    match dwReason {
        DLL_PROCESS_ATTACH => unsafe {
            if let Err(_) = init_proxy(hinstDLL) {
                return 1;
            }

            thread::spawn(|| {
                thread::sleep(Duration::from_secs(5));
                let _ = apply();
            });

            1
        },
        DLL_PROCESS_DETACH => 1,
        _ => 0,
    }
}
