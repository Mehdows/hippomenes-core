use std::env;

//no conditional compilation based on triple, so this is needed
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.starts_with("riscv") {
        // this is only used for conditional compilation
        println!("cargo:rustc-cfg=riscv")
    }
}
