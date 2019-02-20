extern crate cc;

fn main() {
    // for gcc users (tested on Linux)
    /*cc::Build::new()
        .file("src/soa.c")
        .flag("-mavx2")
        .flag("-ftree-vectorize")
        .flag("-ftree-loop-vectorize")
        .flag("-fopt-info-vec-all")
        .flag("-fopenmp")
        .flag("-fopenmp-simd")
        .opt_level(3)
        .compile("soa");*/

    // should work with all C compilers
    cc::Build::new()
        .file("src/soa.c")
        .opt_level(3)
        .compile("soa");
}
