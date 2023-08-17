fn main() {
    println!("cargo:rustc-link-lib=dylib=uci");
    println!("cargo:rustc-link-lib=dylib=ubox");
}
