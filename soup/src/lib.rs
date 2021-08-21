#![allow(dead_code)]

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gio;
extern crate gio_sys;
extern crate gobject_sys;
extern crate soup_sys as ffi;
extern crate futures_util;
extern crate fragile;

extern crate libc;
#[macro_use]
extern crate bitflags;


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

