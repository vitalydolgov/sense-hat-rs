fn main() {
    println!(r"cargo:rustc-link-search=native=RTIMULib/RTIMULib/build");
    println!(r"cargo:rustc-link-lib=dylib=RTIMULib");
}
