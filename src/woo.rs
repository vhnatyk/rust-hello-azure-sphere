use crate::log;
use core::panic::PanicInfo;
extern crate alloc;
use alloc::format;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    log::log_debug("panic!");
    loop {}
}

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn hello_world() {
    log::log_debug("out from hello");
    let dd = 1 + 2;
    log::log_debug(&format!("dd: {}", dd))
}

#[cfg(not(test))]
#[no_mangle]
#[allow(dead_code)]
extern "C" fn rust_eh_personality() {}

#[allow(dead_code)]
pub extern "C" fn fix_linking_when_not_using_stdlib() {
    panic!()
}
