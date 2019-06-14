use Buffer;
//use MemoryUse;
use glib::translate::*;
use gobject_sys;
use soup_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MessageBody(Boxed<soup_sys::SoupMessageBody>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(soup_sys::soup_message_body_get_type(), ptr as *mut _) as *mut soup_sys::SoupMessageBody,
        free => |ptr| gobject_sys::g_boxed_free(soup_sys::soup_message_body_get_type(), ptr as *mut _),
        get_type => || soup_sys::soup_message_body_get_type(),
    }
}

impl MessageBody {
    pub fn new() -> MessageBody {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_message_body_new())
        }
    }

    //pub fn append(&mut self, use_: MemoryUse, data: &[u8]) {
    //    let length = data.len() as usize;
    //    unsafe {
    //        soup_sys::soup_message_body_append(self.to_glib_none_mut().0, use_.to_glib(), data.to_glib_none().0, length);
    //    }
    //}

    pub fn append_buffer(&mut self, buffer: &mut Buffer) {
        unsafe {
            soup_sys::soup_message_body_append_buffer(self.to_glib_none_mut().0, buffer.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn append_take(&mut self, data: &[u8]) {
        let length = data.len() as usize;
        unsafe {
            soup_sys::soup_message_body_append_take(self.to_glib_none_mut().0, data.to_glib_full(), length);
        }
    }

    pub fn complete(&mut self) {
        unsafe {
            soup_sys::soup_message_body_complete(self.to_glib_none_mut().0);
        }
    }

    pub fn flatten(&mut self) -> Option<Buffer> {
        unsafe {
            from_glib_full(soup_sys::soup_message_body_flatten(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn get_accumulate(&mut self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_message_body_get_accumulate(self.to_glib_none_mut().0))
        }
    }

    pub fn get_chunk(&mut self, offset: i64) -> Option<Buffer> {
        unsafe {
            from_glib_full(soup_sys::soup_message_body_get_chunk(self.to_glib_none_mut().0, offset))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn got_chunk(&mut self, chunk: &mut Buffer) {
        unsafe {
            soup_sys::soup_message_body_got_chunk(self.to_glib_none_mut().0, chunk.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_accumulate(&mut self, accumulate: bool) {
        unsafe {
            soup_sys::soup_message_body_set_accumulate(self.to_glib_none_mut().0, accumulate.to_glib());
        }
    }

    pub fn truncate(&mut self) {
        unsafe {
            soup_sys::soup_message_body_truncate(self.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn wrote_chunk(&mut self, chunk: &mut Buffer) {
        unsafe {
            soup_sys::soup_message_body_wrote_chunk(self.to_glib_none_mut().0, chunk.to_glib_none_mut().0);
        }
    }
}

impl Default for MessageBody {
    fn default() -> Self {
        Self::new()
    }
}
