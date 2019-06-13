// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SessionFeature;
use glib::object::IsA;
use glib::translate::*;
use soup_sys;
use std::fmt;

glib_wrapper! {
    pub struct ContentSniffer(Object<soup_sys::SoupContentSniffer, soup_sys::SoupContentSnifferClass, ContentSnifferClass>) @implements SessionFeature;

    match fn {
        get_type => || soup_sys::soup_content_sniffer_get_type(),
    }
}

impl ContentSniffer {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn new() -> ContentSniffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_content_sniffer_new())
        }
    }
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
impl Default for ContentSniffer {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CONTENT_SNIFFER: Option<&ContentSniffer> = None;

pub trait ContentSnifferExt: 'static {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_buffer_size(&self) -> usize;

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //fn sniff<P: IsA<Message>>(&self, msg: &P, buffer: /*Ignored*/&mut Buffer, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<GString>;
}

impl<O: IsA<ContentSniffer>> ContentSnifferExt for O {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_buffer_size(&self) -> usize {
        unsafe {
            soup_sys::soup_content_sniffer_get_buffer_size(self.as_ref().to_glib_none().0)
        }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //fn sniff<P: IsA<Message>>(&self, msg: &P, buffer: /*Ignored*/&mut Buffer, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<GString> {
    //    unsafe { TODO: call soup_sys:soup_content_sniffer_sniff() }
    //}
}

impl fmt::Display for ContentSniffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContentSniffer")
    }
}
