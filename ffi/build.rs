use cc;

fn main() {
    cc::Build::new()
        .file("src/add.c")
        .compile("libadd.a");

    // println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..." 
    // println!("cargo:rustc-link-lib=dylib=add.obj"); // works like "rustc -l doubler.o"
}