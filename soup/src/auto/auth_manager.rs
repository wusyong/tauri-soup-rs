// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Auth;
use Message;
use SessionFeature;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use URI;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use soup_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AuthManager(Object<soup_sys::SoupAuthManager, soup_sys::SoupAuthManagerClass, AuthManagerClass>) @implements SessionFeature;

    match fn {
        get_type => || soup_sys::soup_auth_manager_get_type(),
    }
}

pub const NONE_AUTH_MANAGER: Option<&AuthManager> = None;

pub trait AuthManagerExt: 'static {
    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn clear_cached_credentials(&self);

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn use_auth<P: IsA<Auth>>(&self, uri: &mut URI, auth: &P);

    fn connect_authenticate<F: Fn(&Self, &Message, &Auth, bool) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthManager>> AuthManagerExt for O {
    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn clear_cached_credentials(&self) {
        unsafe {
            soup_sys::soup_auth_manager_clear_cached_credentials(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn use_auth<P: IsA<Auth>>(&self, uri: &mut URI, auth: &P) {
        unsafe {
            soup_sys::soup_auth_manager_use_auth(self.as_ref().to_glib_none().0, uri.to_glib_none_mut().0, auth.as_ref().to_glib_none().0);
        }
    }

    fn connect_authenticate<F: Fn(&Self, &Message, &Auth, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn authenticate_trampoline<P, F: Fn(&P, &Message, &Auth, bool) + 'static>(this: *mut soup_sys::SoupAuthManager, msg: *mut soup_sys::SoupMessage, auth: *mut soup_sys::SoupAuth, retrying: glib_sys::gboolean, f: glib_sys::gpointer)
            where P: IsA<AuthManager>
        {
            let f: &F = &*(f as *const F);
            f(&AuthManager::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(msg), &from_glib_borrow(auth), from_glib(retrying))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"authenticate\0".as_ptr() as *const _,
                Some(transmute(authenticate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AuthManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthManager")
    }
}
