#![allow(unused_imports)]
#![allow(deprecated)]

extern crate libc;

// Generated in `build.rs`.
include!(concat!(env!("OUT_DIR"), "/semver.rs"));

fn main() {
    #[cfg(target_os = "horizon")]
    linker_fix_3ds::init();
    #[cfg(target_os = "horizon")]
    pthread_3ds::init();

    // The test is about the imports created in `semver.rs`.
    println!("PASSED 1 tests");
}
