#![allow(dead_code)]

extern crate soup_sys as ffi;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::Error;

mod auto;
pub use auto::*;

