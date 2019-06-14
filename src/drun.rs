mod woo;

fn main() {
    // woo::hello_world();
    #[cfg(not(test))] {
        woo::run_sign_test();
    }
    woo::rust_drun();
}
