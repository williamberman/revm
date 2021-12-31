extern crate bindgen;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/intx.cpp")
        .include("cpp")
        .flag("-std=c++17")
        .compile("intx");
}