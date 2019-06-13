// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_24", feature = "dox"))]
use Session;
#[cfg(any(feature = "v2_34", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use soup_sys;
use std::fmt;

glib_wrapper! {
    pub struct SessionFeature(Interface<soup_sys::SoupSessionFeature>);

    match fn {
        get_type => || soup_sys::soup_session_feature_get_type(),
    }
}

pub const NONE_SESSION_FEATURE: Option<&SessionFeature> = None;

pub trait SessionFeatureExt: 'static {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn add_feature(&self, type_: glib::types::Type) -> bool;

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn attach<P: IsA<Session>>(&self, session: &P);

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn detach<P: IsA<Session>>(&self, session: &P);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn has_feature(&self, type_: glib::types::Type) -> bool;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn remove_feature(&self, type_: glib::types::Type) -> bool;
}

impl<O: IsA<SessionFeature>> SessionFeatureExt for O {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn add_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(soup_sys::soup_session_feature_add_feature(self.as_ref().to_glib_none().0, type_.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn attach<P: IsA<Session>>(&self, session: &P) {
        unsafe {
            soup_sys::soup_session_feature_attach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    fn detach<P: IsA<Session>>(&self, session: &P) {
        unsafe {
            soup_sys::soup_session_feature_detach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn has_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(soup_sys::soup_session_feature_has_feature(self.as_ref().to_glib_none().0, type_.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn remove_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(soup_sys::soup_session_feature_remove_feature(self.as_ref().to_glib_none().0, type_.to_glib()))
        }
    }
}

impl fmt::Display for SessionFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SessionFeature")
    }
}
