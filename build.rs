fn main() {
    println!("cargo:rerun-if-changed=src/boot.S");
    cc::Build::new()
        .file("src/boot.S")
        .compile("boot");
}
