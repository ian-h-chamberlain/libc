#![allow(bad_style, improper_ctypes, deprecated)]
extern crate libc;

extern crate ctru;
extern crate scopeguard;

use libc::*;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
