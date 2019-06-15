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
    println!("out from hello run_sign_test");
    //
    let t = 1;
    let n = 3;
    let ttag = 2;
    let s = vec![0, 2];

    println!("running signing (t={},n={},ttag={})", t, n, ttag);
    // println!("memory used: {} bytes", GLOBAL.get());
    {
        let start = std::time::Instant::now();
        common::sign(t, n, ttag, s);
        println!("elapsed {:?}", start.elapsed());
    }
    //#[allow(unreachable_code)]
    //0
    // For some reason this does not print zero =/
    // println!("memory used: {} bytes", GLOBAL.get());
}

#[cfg(not(test))]
extern crate emerald_city_common;
#[cfg(not(test))]
use emerald_city_common::common;

#[no_mangle]
#[allow(dead_code)]
#[cfg(not(test))]
pub extern "C" fn run_keygen_test() {
    println!("out from hello run_keygen_test");

    //
    let t = 1;
    let n = 2;

    println!("running keygen (t={},n={})", t, n);
    // println!("memory used: {} bytes", GLOBAL.get());
    {
        let start = std::time::Instant::now();
        common::keygen_t_n_parties(t, n);
        println!("elapsed {:?}", start.elapsed()); // note :?
    }
    //#[allow(unreachable_code)]
    //0
    // For some reason this does not print zero =/
    // println!("memory used: {} bytes", GLOBAL.get());    
}

#[allow(dead_code)]
pub fn rust_drun() {
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
