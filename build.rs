fn main() {
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=openblas");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/blas");
    println!("cargo:rustc-link-lib=blas");
}