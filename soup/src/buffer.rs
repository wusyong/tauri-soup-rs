//use MemoryUse;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib;
use glib::translate::*;
use soup_sys;
//#[cfg(any(feature = "v2_32", feature = "dox"))]
//use std::mem;
//#[cfg(any(feature = "v2_32", feature = "dox"))]
//use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Buffer(Boxed<soup_sys::SoupBuffer>);

    match fn {
        copy => |ptr| soup_sys::soup_buffer_copy(mut_override(ptr)),
        free => |ptr| soup_sys::soup_buffer_free(ptr),
        get_type => || soup_sys::soup_buffer_get_type(),
    }
}

impl Buffer {
    //pub fn new(use_: MemoryUse, data: &[u8]) -> Buffer {
    //    assert_initialized_main_thread!();
    //    let length = data.len() as usize;
    //    unsafe {
    //        from_glib_full(soup_sys::soup_buffer_new(use_.to_glib(), data.to_glib_none().0, length))
    //    }
    //}

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn new_take(data: &[u8]) -> Buffer {
        assert_initialized_main_thread!();
        let length = data.len() as usize;
        unsafe {
            from_glib_full(soup_sys::soup_buffer_new_take(data.to_glib_full(), length))
        }
    }

    //pub fn new_with_owner(data: &[u8], owner: /*Unimplemented*/Option<Fundamental: Pointer>) -> Buffer {
    //    unsafe { TODO: call soup_sys:soup_buffer_new_with_owner() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_as_bytes(&mut self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(soup_sys::soup_buffer_get_as_bytes(self.to_glib_none_mut().0))
        }
    }

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //pub fn get_data(&mut self) -> Vec<u8> {
    //    unsafe {
    //        let mut data = ptr::null_mut();
    //        let mut length = mem::uninitialized();
    //        soup_sys::soup_buffer_get_data(self.to_glib_none_mut().0, data, &mut length);
    //        FromGlibContainer::from_glib_none_num(data, length as usize)
    //    }
    //}

    //pub fn get_owner(&mut self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call soup_sys:soup_buffer_get_owner() }
    //}

    pub fn new_subbuffer(&mut self, offset: usize, length: usize) -> Option<Buffer> {
        unsafe {
            from_glib_full(soup_sys::soup_buffer_new_subbuffer(self.to_glib_none_mut().0, offset, length))
        }
    }
}