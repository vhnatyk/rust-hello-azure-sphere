/*
	Rust hello from C for Azure Sphere

	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq

	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
*/

#[no_mangle]
#[allow(dead_code)]
#[cfg(not(test))]
pub extern "C" fn hello_world() {
    println!("out from hello");
    let dd = 1 + 2;
    println!("dd: {}", dd)
}

#[no_mangle]
#[allow(dead_code)]
#[cfg(not(test))]
pub extern "C" fn run_sign_test() -> f32 {
    println!("hello from run_sign_test");

    let t = 1;
    let n = 2;

    println!("running signing (t={},n={})", t, n);

    let start = std::time::Instant::now();
    common::sign(t, n);
    let result = start.elapsed();
    let seconds = result.as_secs() as f32 + result.subsec_nanos() as f32 * 1e-9;
    println!("elapsed {:?}", seconds); // note :?
    seconds
}

#[cfg(not(test))]
extern crate other_lib_common;
#[cfg(not(test))]
use other_lib_common::common;

#[no_mangle]
#[allow(dead_code)]
#[cfg(not(test))]
pub extern "C" fn run_keygen_test() -> f32 {
    println!("hello from run_keygen_test");

    let t = 1;
    let n = 2;

    println!("running keygen (t={},n={})", t, n);
    let start = std::time::Instant::now();
    common::keygen(t, n);
    let result = start.elapsed();
    let seconds = result.as_secs() as f32 + result.subsec_nanos() as f32 * 1e-9;
    println!("elapsed {:?}", seconds); // note :?
    seconds
}

#[allow(dead_code)]
pub fn rust_drun() {
    println!("hello from rust_drun");
}
