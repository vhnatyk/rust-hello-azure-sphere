
#[cfg(not(feature = "std"))]
use crate::log;
#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::format;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    log::log_debug("panic!");
    loop {}
}

#[cfg(not(feature = "std"))]
#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn hello_world() {
    log::log_debug("out from hello");
    let dd = 1 + 2;
    log::log_debug(&format!("dd: {}", dd))
}

#[no_mangle]
#[allow(dead_code)]
#[cfg(not(test))]
pub extern "C" fn run_sign_test() {
    //TODO sign test
     println!("out from hello run_sign_test");
}

#[allow(dead_code)]
pub fn rust_drun(){
    println!("out from hello rust_drun");
}

#[cfg(not(feature = "std"))]
#[no_mangle]
#[allow(dead_code)]
extern "C" fn rust_eh_personality() {}

#[allow(dead_code)]
pub extern "C" fn fix_linking_when_not_using_stdlib() {
    panic!()
}
