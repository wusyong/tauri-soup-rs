use AddressFamily;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use soup_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Address(Object<soup_sys::SoupAddress, soup_sys::SoupAddressClass, AddressClass>);

    match fn {
        get_type => || soup_sys::soup_address_get_type(),
    }
}

impl Address {
    pub fn new(name: &str, port: u32) -> Address {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_address_new(name.to_glib_none().0, port))
        }
    }

    pub fn new_any(family: AddressFamily, port: u32) -> Option<Address> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_address_new_any(family.to_glib(), port))
        }
    }

    //pub fn new_from_sockaddr(sa: /*Unimplemented*/Option<Fundamental: Pointer>, len: i32) -> Option<Address> {
    //    unsafe { TODO: call soup_sys:soup_address_new_from_sockaddr() }
    //}
}

pub const NONE_ADDRESS: Option<&Address> = None;

pub trait AddressExt: 'static {
    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn equal_by_ip<P: IsA<Address>>(&self, addr2: &P) -> bool;

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn equal_by_name<P: IsA<Address>>(&self, addr2: &P) -> bool;

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //fn get_gsockaddr(&self) -> /*Ignored*/Option<gio::SocketAddress>;

    fn get_name(&self) -> Option<GString>;

    fn get_physical(&self) -> Option<GString>;

    fn get_port(&self) -> u32;

    //fn get_sockaddr(&self, len: i32) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn hash_by_ip(&self) -> u32;

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn hash_by_name(&self) -> u32;

    fn is_resolved(&self) -> bool;

    fn resolve_async<P: IsA<gio::Cancellable>, Q: FnOnce(&Address, u32) + 'static>(&self, async_context: Option<&glib::MainContext>, cancellable: Option<&P>, callback: Q);

    fn resolve_sync<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> u32;

    fn get_property_family(&self) -> AddressFamily;

    fn get_property_protocol(&self) -> Option<GString>;

    fn connect_property_physical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Address>> AddressExt for O {
    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn equal_by_ip<P: IsA<Address>>(&self, addr2: &P) -> bool {
    //    unsafe {
    //        from_glib(soup_sys::soup_address_equal_by_ip(const_override(self.as_ref().to_glib_none().0), addr2.as_ref().to_glib_none().0))
    //    }
    //}

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn equal_by_name<P: IsA<Address>>(&self, addr2: &P) -> bool {
    //    unsafe {
    //        from_glib(soup_sys::soup_address_equal_by_name(const_override(self.as_ref().to_glib_none().0), addr2.as_ref().to_glib_none().0))
    //    }
    //}

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //fn get_gsockaddr(&self) -> /*Ignored*/Option<gio::SocketAddress> {
    //    unsafe { TODO: call soup_sys:soup_address_get_gsockaddr() }
    //}

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_address_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_physical(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_address_get_physical(self.as_ref().to_glib_none().0))
        }
    }

    fn get_port(&self) -> u32 {
        unsafe {
            soup_sys::soup_address_get_port(self.as_ref().to_glib_none().0)
        }
    }

    //fn get_sockaddr(&self, len: i32) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call soup_sys:soup_address_get_sockaddr() }
    //}

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn hash_by_ip(&self) -> u32 {
    //    unsafe {
    //        soup_sys::soup_address_hash_by_ip(const_override(self.as_ref().to_glib_none().0))
    //    }
    //}

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //fn hash_by_name(&self) -> u32 {
    //    unsafe {
    //        soup_sys::soup_address_hash_by_name(const_override(self.as_ref().to_glib_none().0))
    //    }
    //}

    fn is_resolved(&self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_address_is_resolved(self.as_ref().to_glib_none().0))
        }
    }

    fn resolve_async<P: IsA<gio::Cancellable>, Q: FnOnce(&Address, u32) + 'static>(&self, async_context: Option<&glib::MainContext>, cancellable: Option<&P>, callback: Q) {
        let callback_data: Box_<Q> = Box::new(callback);
        unsafe extern "C" fn callback_func<P: IsA<gio::Cancellable>, Q: FnOnce(&Address, u32) + 'static>(addr: *mut soup_sys::SoupAddress, status: libc::c_uint, user_data: glib_sys::gpointer) {
            let addr = from_glib_borrow(addr);
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            (*callback)(&addr, status);
        }
        let callback = Some(callback_func::<P, Q> as _);
        let super_callback0: Box_<Q> = callback_data;
        unsafe {
            soup_sys::soup_address_resolve_async(self.as_ref().to_glib_none().0, async_context.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, callback, Box::into_raw(super_callback0) as *mut _);
        }
    }

    fn resolve_sync<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>) -> u32 {
        unsafe {
            soup_sys::soup_address_resolve_sync(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0)
        }
    }

    fn get_property_family(&self) -> AddressFamily {
        unsafe {
            let mut value = Value::from_type(<AddressFamily as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"family\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_protocol(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"protocol\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_physical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_physical_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAddress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Address>
        {
            let f: &F = &*(f as *const F);
            f(&Address::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::physical\0".as_ptr() as *const _,
                Some(transmute(notify_physical_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address")
    }
}
