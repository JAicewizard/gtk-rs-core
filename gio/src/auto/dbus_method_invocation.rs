// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DBusConnection;
use crate::DBusMessage;
use crate::DBusMethodInfo;
use crate::DBusPropertyInfo;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use crate::UnixFDList;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GDBusMethodInvocation")]
    pub struct DBusMethodInvocation(Object<ffi::GDBusMethodInvocation>);

    match fn {
        type_ => || ffi::g_dbus_method_invocation_get_type(),
    }
}

impl DBusMethodInvocation {
    #[doc(alias = "g_dbus_method_invocation_get_connection")]
    #[doc(alias = "get_connection")]
    pub fn connection(&self) -> DBusConnection {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_connection(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_interface_name")]
    #[doc(alias = "get_interface_name")]
    pub fn interface_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_interface_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_message")]
    #[doc(alias = "get_message")]
    pub fn message(&self) -> DBusMessage {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_message(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_method_info")]
    #[doc(alias = "get_method_info")]
    pub fn method_info(&self) -> Option<DBusMethodInfo> {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_method_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_method_name")]
    #[doc(alias = "get_method_name")]
    pub fn method_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_method_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_object_path")]
    #[doc(alias = "get_object_path")]
    pub fn object_path(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_object_path(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_parameters")]
    #[doc(alias = "get_parameters")]
    pub fn parameters(&self) -> glib::Variant {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_parameters(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_property_info")]
    #[doc(alias = "get_property_info")]
    pub fn property_info(&self) -> Option<DBusPropertyInfo> {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_property_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_method_invocation_get_sender")]
    #[doc(alias = "get_sender")]
    pub fn sender(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_method_invocation_get_sender(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_dbus_method_invocation_get_user_data")]
    //#[doc(alias = "get_user_data")]
    //pub fn user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_dbus_method_invocation_get_user_data() }
    //}

    #[doc(alias = "g_dbus_method_invocation_return_dbus_error")]
    pub fn return_dbus_error<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + 's,
    >(
        &self,
        error_name: &'s P,
        error_message: &'s Q,
    ) {
        unsafe {
            ffi::g_dbus_method_invocation_return_dbus_error(
                self.to_glib_full(),
                error_name.to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_dbus_method_invocation_return_error")]
    //pub fn return_error<'s, P: ToGlibPtr<'s, *mut libc::c_char> + 's>(&self, domain: glib::Quark, code: i32, format: & 's P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_dbus_method_invocation_return_error() }
    //}

    //#[doc(alias = "g_dbus_method_invocation_return_error_valist")]
    //pub fn return_error_valist<'s, P: ToGlibPtr<'s, *mut libc::c_char> + 's>(&self, domain: glib::Quark, code: i32, format: & 's P, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:g_dbus_method_invocation_return_error_valist() }
    //}

    #[doc(alias = "g_dbus_method_invocation_return_value")]
    pub fn return_value(&self, parameters: Option<&glib::Variant>) {
        unsafe {
            ffi::g_dbus_method_invocation_return_value(
                self.to_glib_full(),
                parameters.to_glib_none().0,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_method_invocation_return_value_with_unix_fd_list")]
    pub fn return_value_with_unix_fd_list<P: IsA<UnixFDList>>(
        &self,
        parameters: Option<&glib::Variant>,
        fd_list: Option<&P>,
    ) {
        unsafe {
            ffi::g_dbus_method_invocation_return_value_with_unix_fd_list(
                self.to_glib_full(),
                parameters.to_glib_none().0,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for DBusMethodInvocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DBusMethodInvocation")
    }
}
