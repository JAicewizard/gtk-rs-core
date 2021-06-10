// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
#[cfg(any(feature = "v2_36", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
use libc::c_char;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PixbufFormat(Boxed<ffi::GdkPixbufFormat>);

    match fn {
        copy => |ptr| ffi::gdk_pixbuf_format_copy(ptr),
        free => |ptr| ffi::gdk_pixbuf_format_free(ptr),
        type_ => || ffi::gdk_pixbuf_format_get_type(),
    }
}

impl PixbufFormat {
    #[doc(alias = "gdk_pixbuf_format_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_description(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_get_extensions")]
    #[doc(alias = "get_extensions")]
    pub fn extensions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_pixbuf_format_get_extensions(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_get_license")]
    #[doc(alias = "get_license")]
    pub fn license(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_license(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_get_mime_types")]
    #[doc(alias = "get_mime_types")]
    pub fn mime_types(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_pixbuf_format_get_mime_types(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_name(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_is_disabled")]
    pub fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_disabled(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    #[doc(alias = "gdk_pixbuf_format_is_save_option_supported")]
    pub fn is_save_option_supported<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        option_key: &'s P,
    ) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_save_option_supported(
                mut_override(self.to_glib_none().0),
                option_key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_is_scalable")]
    pub fn is_scalable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_scalable(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_is_writable")]
    pub fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_writable(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gdk_pixbuf_format_set_disabled")]
    pub fn set_disabled(&mut self, disabled: bool) {
        unsafe {
            ffi::gdk_pixbuf_format_set_disabled(self.to_glib_none_mut().0, disabled.into_glib());
        }
    }
}
