// extern crate cmake;

// use cmake::Config;

// fn main(){
//     let dst = Config::new("libbadmath").build();

//     println!("Rust: cargo:rust-link-search=native={}", dst.display());
//     println!("Rust cargo:rustc-link-lib=static=badmath");
// }

fn main() {
    cc::Build::new()
        .file("./libbadmath/badmath.c")
        .static_flag(true)
        .compile("badmath");
}
