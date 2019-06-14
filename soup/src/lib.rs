#![allow(dead_code)]

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gio;
extern crate gio_sys;
extern crate gobject_sys;
extern crate soup_sys;
#[cfg(feature = "futures")]
extern crate futures;
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
pub mod prelude;
pub use prelude::*;

mod auto;
pub use auto::*;

mod uri;
pub use uri::*;

mod address;
pub use address::*;

mod buffer;
pub use buffer::*;

mod message_body;
pub use message_body::*;

mod message;
pub use message::*;
