mod woo;

fn main() {
    woo::rust_drun();
    // woo::hello_world();
    #[cfg(not(test))]
    {
        woo::run_sign_test();
        woo::run_keygen_test();
    }
}
