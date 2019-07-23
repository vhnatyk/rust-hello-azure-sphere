/*
	Rust hello from C for Azure Sphere
	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq

	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
*/

mod woo;

fn main() {
    woo::rust_drun();    
    #[cfg(not(test))]
    {
        woo::hello_world();
        woo::run_sign_test();
        woo::run_keygen_test();
    }
}
