// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_42", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use gio_sys;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use gobject_sys;
use soup_sys;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use std::pin::Pin;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use Session;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use URI;

glib_wrapper! {
    pub struct Request(Object<soup_sys::SoupRequest, soup_sys::SoupRequestClass, RequestClass>);

    match fn {
        get_type => || soup_sys::soup_request_get_type(),
    }
}

pub const NONE_REQUEST: Option<&Request> = None;

pub trait RequestExt: 'static {
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_content_length(&self) -> i64;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_content_type(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_session(&self) -> Option<Session>;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_uri(&self) -> Option<URI>;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<gio::InputStream, glib::Error>;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send_async_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::InputStream, glib::Error>> + 'static>>;
}

impl<O: IsA<Request>> RequestExt for O {
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_content_length(&self) -> i64 {
        unsafe {
            soup_sys::soup_request_get_content_length(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_content_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_request_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_session(&self) -> Option<Session> {
        unsafe {
            from_glib_none(soup_sys::soup_request_get_session(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_uri(&self) -> Option<URI> {
        unsafe {
            from_glib_none(soup_sys::soup_request_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> Result<gio::InputStream, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = soup_sys::soup_request_send(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn send_async_trampoline<Q: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = soup_sys::soup_request_send_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = send_async_trampoline::<Q>;
        unsafe {
            soup_sys::soup_request_send_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn send_async_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::InputStream, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.send_async(
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request")
    }
}
