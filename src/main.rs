use std::env;

extern crate blas_src;
extern crate serde_json;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
}
