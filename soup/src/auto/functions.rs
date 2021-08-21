// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::Cookie;
#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
use crate::Date;
use crate::HTTPVersion;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::Message;
use crate::MessageHeaders;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
use crate::Multipart;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;


#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "soup_add_completion")]
pub fn add_completion<P: Fn() -> bool + Send + Sync + 'static>(async_context: Option<&glib::MainContext>, function: P) -> Option<glib::Source> {
    assert_initialized_main_thread!();
    let function_data: Box_<P> = Box_::new(function);
    unsafe extern "C" fn function_func<P: Fn() -> bool + Send + Sync + 'static>(user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
        let callback: &P = &*(user_data as *mut _);
        let res = (*callback)();
        res.into_glib()
    }
    let function = Some(function_func::<P> as _);
    let super_callback0: Box_<P> = function_data;
    unsafe {
        from_glib_full(ffi::soup_add_completion(async_context.to_glib_none().0, function, Box_::into_raw(super_callback0) as *mut _))
    }
}

#[doc(alias = "soup_add_idle")]
pub fn add_idle<P: Fn() -> bool + Send + Sync + 'static>(async_context: Option<&glib::MainContext>, function: P) -> Option<glib::Source> {
    assert_initialized_main_thread!();
    let function_data: Box_<P> = Box_::new(function);
    unsafe extern "C" fn function_func<P: Fn() -> bool + Send + Sync + 'static>(user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
        let callback: &P = &*(user_data as *mut _);
        let res = (*callback)();
        res.into_glib()
    }
    let function = Some(function_func::<P> as _);
    let super_callback0: Box_<P> = function_data;
    unsafe {
        from_glib_full(ffi::soup_add_idle(async_context.to_glib_none().0, function, Box_::into_raw(super_callback0) as *mut _))
    }
}

//#[doc(alias = "soup_add_io_watch")]
//pub fn add_io_watch(async_context: Option<&glib::MainContext>, chan: &glib::IOChannel, condition: glib::IOCondition, function: /*Unimplemented*/Fn(&glib::IOChannel, &glib::IOCondition, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<glib::Source> {
//    unsafe { TODO: call ffi:soup_add_io_watch() }
//}

#[doc(alias = "soup_add_timeout")]
pub fn add_timeout<P: Fn() -> bool + Send + Sync + 'static>(async_context: Option<&glib::MainContext>, interval: u32, function: P) -> Option<glib::Source> {
    assert_initialized_main_thread!();
    let function_data: Box_<P> = Box_::new(function);
    unsafe extern "C" fn function_func<P: Fn() -> bool + Send + Sync + 'static>(user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
        let callback: &P = &*(user_data as *mut _);
        let res = (*callback)();
        res.into_glib()
    }
    let function = Some(function_func::<P> as _);
    let super_callback0: Box_<P> = function_data;
    unsafe {
        from_glib_full(ffi::soup_add_timeout(async_context.to_glib_none().0, interval, function, Box_::into_raw(super_callback0) as *mut _))
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "soup_check_version")]
pub fn check_version(major: u32, minor: u32, micro: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_check_version(major, minor, micro))
    }
}

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "soup_cookies_from_request")]
pub fn cookies_from_request<P: IsA<Message>>(msg: &P) -> Vec<Cookie> {
    skip_assert_initialized!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_cookies_from_request(msg.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "soup_cookies_from_response")]
pub fn cookies_from_response<P: IsA<Message>>(msg: &P) -> Vec<Cookie> {
    skip_assert_initialized!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_cookies_from_response(msg.as_ref().to_glib_none().0))
    }
}

//#[doc(alias = "soup_form_decode")]
//pub fn form_decode(encoded_form: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_form_decode() }
//}

//#[cfg(any(feature = "v2_26", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
//#[doc(alias = "soup_form_decode_multipart")]
//pub fn form_decode_multipart<P: IsA<Message>>(msg: &P, file_control_name: Option<&str>) -> (/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, glib::GString, glib::GString, Buffer) {
//    unsafe { TODO: call ffi:soup_form_decode_multipart() }
//}

//#[doc(alias = "soup_form_encode")]
//pub fn form_encode(first_field: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode() }
//}

//#[doc(alias = "soup_form_encode_datalist")]
//pub fn form_encode_datalist(form_data_set: /*Ignored*/&mut glib::Data) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_datalist() }
//}

//#[doc(alias = "soup_form_encode_hash")]
//pub fn form_encode_hash(form_data_set: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_hash() }
//}

//#[doc(alias = "soup_form_encode_valist")]
//pub fn form_encode_valist(first_field: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_form_encode_valist() }
//}

//#[doc(alias = "soup_form_request_new")]
//pub fn form_request_new(method: &str, uri: &str, first_field: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Message> {
//    unsafe { TODO: call ffi:soup_form_request_new() }
//}

//#[doc(alias = "soup_form_request_new_from_datalist")]
//pub fn form_request_new_from_datalist(method: &str, uri: &str, form_data_set: /*Ignored*/&mut glib::Data) -> Option<Message> {
//    unsafe { TODO: call ffi:soup_form_request_new_from_datalist() }
//}

//#[doc(alias = "soup_form_request_new_from_hash")]
//pub fn form_request_new_from_hash(method: &str, uri: &str, form_data_set: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<Message> {
//    unsafe { TODO: call ffi:soup_form_request_new_from_hash() }
//}

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
#[doc(alias = "soup_form_request_new_from_multipart")]
pub fn form_request_new_from_multipart(uri: &str, multipart: &mut Multipart) -> Option<Message> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::soup_form_request_new_from_multipart(uri.to_glib_none().0, multipart.to_glib_none_mut().0))
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "soup_get_major_version")]
#[doc(alias = "get_major_version")]
pub fn major_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::soup_get_major_version()
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "soup_get_micro_version")]
#[doc(alias = "get_micro_version")]
pub fn micro_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::soup_get_micro_version()
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "soup_get_minor_version")]
#[doc(alias = "get_minor_version")]
pub fn minor_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::soup_get_minor_version()
    }
}

//#[doc(alias = "soup_get_resource")]
//#[doc(alias = "get_resource")]
//pub fn resource() -> /*Ignored*/Option<gio::Resource> {
//    unsafe { TODO: call ffi:soup_get_resource() }
//}

#[doc(alias = "soup_header_contains")]
pub fn header_contains(header: &str, token: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_header_contains(header.to_glib_none().0, token.to_glib_none().0))
    }
}

//#[doc(alias = "soup_header_free_list")]
//pub fn header_free_list(list: /*Unimplemented*/&[&Fundamental: Pointer]) {
//    unsafe { TODO: call ffi:soup_header_free_list() }
//}

//#[doc(alias = "soup_header_free_param_list")]
//pub fn header_free_param_list(param_list: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
//    unsafe { TODO: call ffi:soup_header_free_param_list() }
//}

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
#[doc(alias = "soup_header_g_string_append_param")]
pub fn header_g_string_append_param(string: &mut glib::String, name: &str, value: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::soup_header_g_string_append_param(string.to_glib_none_mut().0, name.to_glib_none().0, value.to_glib_none().0);
    }
}

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "soup_header_g_string_append_param_quoted")]
pub fn header_g_string_append_param_quoted(string: &mut glib::String, name: &str, value: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::soup_header_g_string_append_param_quoted(string.to_glib_none_mut().0, name.to_glib_none().0, value.to_glib_none().0);
    }
}

#[doc(alias = "soup_header_parse_list")]
pub fn header_parse_list(header: &str) -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::soup_header_parse_list(header.to_glib_none().0))
    }
}

//#[doc(alias = "soup_header_parse_param_list")]
//pub fn header_parse_param_list(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_param_list() }
//}

//#[cfg(any(feature = "v2_66", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
//#[doc(alias = "soup_header_parse_param_list_strict")]
//pub fn header_parse_param_list_strict(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_param_list_strict() }
//}

//#[doc(alias = "soup_header_parse_quality_list")]
//pub fn header_parse_quality_list(header: &str, unacceptable: /*Unimplemented*/Vec<glib::GString>) -> Vec<glib::GString> {
//    unsafe { TODO: call ffi:soup_header_parse_quality_list() }
//}

//#[cfg(any(feature = "v2_24", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
//#[doc(alias = "soup_header_parse_semi_param_list")]
//pub fn header_parse_semi_param_list(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_semi_param_list() }
//}

//#[cfg(any(feature = "v2_66", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
//#[doc(alias = "soup_header_parse_semi_param_list_strict")]
//pub fn header_parse_semi_param_list_strict(header: &str) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:soup_header_parse_semi_param_list_strict() }
//}

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
#[doc(alias = "soup_headers_parse")]
pub fn headers_parse(str: &str, dest: &mut MessageHeaders) -> bool {
    assert_initialized_main_thread!();
    let len = str.len() as i32;
    unsafe {
        from_glib(ffi::soup_headers_parse(str.to_glib_none().0, len, dest.to_glib_none_mut().0))
    }
}

#[doc(alias = "soup_headers_parse_request")]
pub fn headers_parse_request(str: &str, req_headers: &mut MessageHeaders) -> (u32, glib::GString, glib::GString, HTTPVersion) {
    assert_initialized_main_thread!();
    let len = str.len() as i32;
    unsafe {
        let mut req_method = ptr::null_mut();
        let mut req_path = ptr::null_mut();
        let mut ver = mem::MaybeUninit::uninit();
        let ret = ffi::soup_headers_parse_request(str.to_glib_none().0, len, req_headers.to_glib_none_mut().0, &mut req_method, &mut req_path, ver.as_mut_ptr());
        let ver = ver.assume_init();
        (ret, from_glib_full(req_method), from_glib_full(req_path), from_glib(ver))
    }
}

#[doc(alias = "soup_headers_parse_response")]
pub fn headers_parse_response(str: &str, headers: &mut MessageHeaders) -> Option<(HTTPVersion, u32, glib::GString)> {
    assert_initialized_main_thread!();
    let len = str.len() as i32;
    unsafe {
        let mut ver = mem::MaybeUninit::uninit();
        let mut status_code = mem::MaybeUninit::uninit();
        let mut reason_phrase = ptr::null_mut();
        let ret = from_glib(ffi::soup_headers_parse_response(str.to_glib_none().0, len, headers.to_glib_none_mut().0, ver.as_mut_ptr(), status_code.as_mut_ptr(), &mut reason_phrase));
        let ver = ver.assume_init();
        let status_code = status_code.assume_init();
        if ret { Some((from_glib(ver), status_code, from_glib_full(reason_phrase))) } else { None }
    }
}

#[doc(alias = "soup_headers_parse_status_line")]
pub fn headers_parse_status_line(status_line: &str) -> Option<(HTTPVersion, u32, glib::GString)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut ver = mem::MaybeUninit::uninit();
        let mut status_code = mem::MaybeUninit::uninit();
        let mut reason_phrase = ptr::null_mut();
        let ret = from_glib(ffi::soup_headers_parse_status_line(status_line.to_glib_none().0, ver.as_mut_ptr(), status_code.as_mut_ptr(), &mut reason_phrase));
        let ver = ver.assume_init();
        let status_code = status_code.assume_init();
        if ret { Some((from_glib(ver), status_code, from_glib_full(reason_phrase))) } else { None }
    }
}

#[doc(alias = "soup_http_error_quark")]
pub fn http_error_quark() -> glib::Quark {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_http_error_quark())
    }
}

//#[doc(alias = "soup_str_case_equal")]
//pub fn str_case_equal(v1: /*Unimplemented*/Option<Fundamental: Pointer>, v2: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:soup_str_case_equal() }
//}

//#[doc(alias = "soup_str_case_hash")]
//pub fn str_case_hash(key: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call ffi:soup_str_case_hash() }
//}

#[cfg(any(feature = "v2_40", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
#[doc(alias = "soup_tld_domain_is_public_suffix")]
pub fn tld_domain_is_public_suffix(domain: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::soup_tld_domain_is_public_suffix(domain.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_40", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
#[doc(alias = "soup_tld_get_base_domain")]
pub fn tld_get_base_domain(hostname: &str) -> Result<glib::GString, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_tld_get_base_domain(hostname.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

//#[doc(alias = "soup_value_array_append")]
//pub fn value_array_append(array: /*Ignored*/&mut glib::ValueArray, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_value_array_append() }
//}

//#[doc(alias = "soup_value_array_append_vals")]
//pub fn value_array_append_vals(array: /*Ignored*/&mut glib::ValueArray, first_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_value_array_append_vals() }
//}

//#[doc(alias = "soup_value_array_from_args")]
//pub fn value_array_from_args(args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> /*Ignored*/Option<glib::ValueArray> {
//    unsafe { TODO: call ffi:soup_value_array_from_args() }
//}

//#[doc(alias = "soup_value_array_get_nth")]
//pub fn value_array_get_nth(array: /*Ignored*/&mut glib::ValueArray, index_: u32, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:soup_value_array_get_nth() }
//}

//#[doc(alias = "soup_value_array_insert")]
//pub fn value_array_insert(array: /*Ignored*/&mut glib::ValueArray, index_: u32, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_value_array_insert() }
//}

//#[doc(alias = "soup_value_array_new")]
//pub fn value_array_new() -> /*Ignored*/Option<glib::ValueArray> {
//    unsafe { TODO: call ffi:soup_value_array_new() }
//}

//#[doc(alias = "soup_value_array_new_with_vals")]
//pub fn value_array_new_with_vals(first_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> /*Ignored*/Option<glib::ValueArray> {
//    unsafe { TODO: call ffi:soup_value_array_new_with_vals() }
//}

//#[doc(alias = "soup_value_array_to_args")]
//pub fn value_array_to_args(array: /*Ignored*/&mut glib::ValueArray, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> bool {
//    unsafe { TODO: call ffi:soup_value_array_to_args() }
//}

//#[doc(alias = "soup_value_hash_insert")]
//pub fn value_hash_insert(hash: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }, key: &str, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_value_hash_insert() }
//}

//#[doc(alias = "soup_value_hash_insert_vals")]
//pub fn value_hash_insert_vals(hash: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }, first_key: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_value_hash_insert_vals() }
//}

//#[doc(alias = "soup_value_hash_insert_value")]
//pub fn value_hash_insert_value(hash: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }, key: &str, value: /*Ignored*/&mut glib::Value) {
//    unsafe { TODO: call ffi:soup_value_hash_insert_value() }
//}

//#[doc(alias = "soup_value_hash_lookup")]
//pub fn value_hash_lookup(hash: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }, key: &str, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:soup_value_hash_lookup() }
//}

//#[doc(alias = "soup_value_hash_lookup_vals")]
//pub fn value_hash_lookup_vals(hash: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }, first_key: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:soup_value_hash_lookup_vals() }
//}

//#[doc(alias = "soup_value_hash_new")]
//pub fn value_hash_new() -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 } {
//    unsafe { TODO: call ffi:soup_value_hash_new() }
//}

//#[doc(alias = "soup_value_hash_new_with_vals")]
//pub fn value_hash_new_with_vals(first_key: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 } {
//    unsafe { TODO: call ffi:soup_value_hash_new_with_vals() }
//}

#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
#[doc(alias = "soup_websocket_client_prepare_handshake")]
pub fn websocket_client_prepare_handshake<P: IsA<Message>>(msg: &P, origin: Option<&str>, protocols: &[&str]) {
    skip_assert_initialized!();
    unsafe {
        ffi::soup_websocket_client_prepare_handshake(msg.as_ref().to_glib_none().0, origin.to_glib_none().0, protocols.to_glib_none().0);
    }
}

//#[cfg(any(feature = "v2_68", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
//#[doc(alias = "soup_websocket_client_prepare_handshake_with_extensions")]
//pub fn websocket_client_prepare_handshake_with_extensions<P: IsA<Message>>(msg: &P, origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass]) {
//    unsafe { TODO: call ffi:soup_websocket_client_prepare_handshake_with_extensions() }
//}

#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
#[doc(alias = "soup_websocket_client_verify_handshake")]
pub fn websocket_client_verify_handshake<P: IsA<Message>>(msg: &P) -> Result<(), glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::soup_websocket_client_verify_handshake(msg.as_ref().to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//#[cfg(any(feature = "v2_68", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
//#[doc(alias = "soup_websocket_client_verify_handshake_with_extensions")]
//pub fn websocket_client_verify_handshake_with_extensions<P: IsA<Message>>(msg: &P, supported_extensions: /*Ignored*/&[&glib::TypeClass], accepted_extensions: /*Unimplemented*/Vec<WebsocketExtension>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:soup_websocket_client_verify_handshake_with_extensions() }
//}

#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
#[doc(alias = "soup_websocket_server_check_handshake")]
pub fn websocket_server_check_handshake<P: IsA<Message>>(msg: &P, origin: Option<&str>, protocols: &[&str]) -> Result<(), glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::soup_websocket_server_check_handshake(msg.as_ref().to_glib_none().0, origin.to_glib_none().0, protocols.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//#[cfg(any(feature = "v2_68", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
//#[doc(alias = "soup_websocket_server_check_handshake_with_extensions")]
//pub fn websocket_server_check_handshake_with_extensions<P: IsA<Message>>(msg: &P, origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass]) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:soup_websocket_server_check_handshake_with_extensions() }
//}

#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
#[doc(alias = "soup_websocket_server_process_handshake")]
pub fn websocket_server_process_handshake<P: IsA<Message>>(msg: &P, expected_origin: Option<&str>, protocols: &[&str]) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::soup_websocket_server_process_handshake(msg.as_ref().to_glib_none().0, expected_origin.to_glib_none().0, protocols.to_glib_none().0))
    }
}

//#[cfg(any(feature = "v2_68", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
//#[doc(alias = "soup_websocket_server_process_handshake_with_extensions")]
//pub fn websocket_server_process_handshake_with_extensions<P: IsA<Message>>(msg: &P, expected_origin: Option<&str>, protocols: &[&str], supported_extensions: /*Ignored*/&[&glib::TypeClass], accepted_extensions: /*Unimplemented*/Vec<WebsocketExtension>) -> bool {
//    unsafe { TODO: call ffi:soup_websocket_server_process_handshake_with_extensions() }
//}

//#[doc(alias = "soup_xmlrpc_build_fault")]
//pub fn xmlrpc_build_fault(fault_code: i32, fault_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_xmlrpc_build_fault() }
//}

//#[doc(alias = "soup_xmlrpc_build_method_call")]
//pub fn xmlrpc_build_method_call(method_name: &str, params: /*Ignored*/&[&glib::Value]) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_xmlrpc_build_method_call() }
//}

//#[doc(alias = "soup_xmlrpc_build_method_response")]
//pub fn xmlrpc_build_method_response(value: /*Ignored*/&mut glib::Value) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_xmlrpc_build_method_response() }
//}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_build_request")]
pub fn xmlrpc_build_request(method_name: &str, params: &glib::Variant) -> Result<glib::GString, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_xmlrpc_build_request(method_name.to_glib_none().0, params.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_build_response")]
pub fn xmlrpc_build_response(value: &glib::Variant) -> Result<glib::GString, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_xmlrpc_build_response(value.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

//#[doc(alias = "soup_xmlrpc_extract_method_call")]
//pub fn xmlrpc_extract_method_call(method_call: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_xmlrpc_extract_method_call() }
//}

//#[doc(alias = "soup_xmlrpc_extract_method_response")]
//pub fn xmlrpc_extract_method_response(method_response: &str, error: &mut glib::Error, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:soup_xmlrpc_extract_method_response() }
//}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_message_new")]
pub fn xmlrpc_message_new(uri: &str, method_name: &str, params: &glib::Variant) -> Result<Message, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_xmlrpc_message_new(uri.to_glib_none().0, method_name.to_glib_none().0, params.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

//#[cfg(any(feature = "v2_52", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
//#[doc(alias = "soup_xmlrpc_message_set_fault")]
//pub fn xmlrpc_message_set_fault<P: IsA<Message>>(msg: &P, fault_code: i32, fault_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_xmlrpc_message_set_fault() }
//}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_message_set_response")]
pub fn xmlrpc_message_set_response<P: IsA<Message>>(msg: &P, value: &glib::Variant) -> Result<(), glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::soup_xmlrpc_message_set_response(msg.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//#[doc(alias = "soup_xmlrpc_parse_method_call")]
//pub fn xmlrpc_parse_method_call(method_call: &str, params: /*Ignored*/glib::ValueArray) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:soup_xmlrpc_parse_method_call() }
//}

//#[doc(alias = "soup_xmlrpc_parse_method_response")]
//pub fn xmlrpc_parse_method_response(method_response: &str, value: /*Ignored*/glib::Value) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:soup_xmlrpc_parse_method_response() }
//}

//#[cfg(any(feature = "v2_52", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
//#[doc(alias = "soup_xmlrpc_parse_request")]
//pub fn xmlrpc_parse_request(method_call: &str, params: /*Ignored*/XMLRPCParams) -> Result<glib::GString, glib::Error> {
//    unsafe { TODO: call ffi:soup_xmlrpc_parse_request() }
//}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_parse_response")]
pub fn xmlrpc_parse_response(method_response: &str, signature: Option<&str>) -> Result<glib::Variant, glib::Error> {
    assert_initialized_main_thread!();
    let length = method_response.len() as i32;
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_xmlrpc_parse_response(method_response.to_glib_none().0, length, signature.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

//#[doc(alias = "soup_xmlrpc_request_new")]
//pub fn xmlrpc_request_new(uri: &str, method_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Message> {
//    unsafe { TODO: call ffi:soup_xmlrpc_request_new() }
//}

//#[doc(alias = "soup_xmlrpc_set_fault")]
//pub fn xmlrpc_set_fault<P: IsA<Message>>(msg: &P, fault_code: i32, fault_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_xmlrpc_set_fault() }
//}

//#[doc(alias = "soup_xmlrpc_set_response")]
//pub fn xmlrpc_set_response<P: IsA<Message>>(msg: &P, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:soup_xmlrpc_set_response() }
//}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_variant_get_datetime")]
pub fn xmlrpc_variant_get_datetime(variant: &glib::Variant) -> Result<Date, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::soup_xmlrpc_variant_get_datetime(variant.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v2_52", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
#[doc(alias = "soup_xmlrpc_variant_new_datetime")]
pub fn xmlrpc_variant_new_datetime(date: &mut Date) -> Option<glib::Variant> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::soup_xmlrpc_variant_new_datetime(date.to_glib_none_mut().0))
    }
}