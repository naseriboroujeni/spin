//! This crate is generated by the build script
//!
//! The contents of this crate include:
//! * constants for the paths to the test component wasm binaries
//! * a function which takes a test component name and returns the path to the wasm binary

include!(concat!(env!("OUT_DIR"), "/gen.rs"));
