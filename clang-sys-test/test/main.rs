#![allow(bad_style)]

extern crate clang_sys;
use clang_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
