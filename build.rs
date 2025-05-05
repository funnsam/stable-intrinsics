fn main() {
    println!("cargo::rustc-check-cfg=cfg(nightly)");

    #[cfg(not(feature = "stable-fallback"))]
    if rustversion::cfg!(nightly) {
        println!("cargo:rustc-cfg=nightly");
    }
}
