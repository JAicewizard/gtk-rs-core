// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Pixbuf;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GdkPixbufAnimation")]
    pub struct PixbufAnimation(Object<ffi::GdkPixbufAnimation, ffi::GdkPixbufAnimationClass>);

    match fn {
        type_ => || ffi::gdk_pixbuf_animation_get_type(),
    }
}

impl PixbufAnimation {
    #[doc(alias = "gdk_pixbuf_animation_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file<P: AsRef<std::path::Path>>(
        filename: P,
    ) -> Result<PixbufAnimation, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_animation_new_from_file(
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_animation_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        resource_path: &'s P,
    ) -> Result<PixbufAnimation, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_animation_new_from_resource(
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_animation_new_from_stream")]
    #[doc(alias = "new_from_stream")]
    pub fn from_stream<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
        stream: &P,
        cancellable: Option<&Q>,
    ) -> Result<PixbufAnimation, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_animation_new_from_stream(
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_animation_new_from_stream_async")]
    #[doc(alias = "new_from_stream_async")]
    pub fn from_stream_async<
        P: IsA<gio::InputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<PixbufAnimation, glib::Error>) + Send + 'static,
    >(
        stream: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn from_stream_async_trampoline<
            R: FnOnce(Result<PixbufAnimation, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_animation_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = from_stream_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_animation_new_from_stream_async(
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn from_stream_async_future<P: IsA<gio::InputStream> + Clone + 'static>(
        stream: &P,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<PixbufAnimation, glib::Error>> + 'static>>
    {
        let stream = stream.clone();
        Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::from_stream_async(&stream, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

pub const NONE_PIXBUF_ANIMATION: Option<&PixbufAnimation> = None;

pub trait PixbufAnimationExt: 'static {
    #[doc(alias = "gdk_pixbuf_animation_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> i32;

    #[doc(alias = "gdk_pixbuf_animation_get_static_image")]
    #[doc(alias = "get_static_image")]
    fn static_image(&self) -> Option<Pixbuf>;

    #[doc(alias = "gdk_pixbuf_animation_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> i32;

    #[doc(alias = "gdk_pixbuf_animation_is_static_image")]
    fn is_static_image(&self) -> bool;
}

impl<O: IsA<PixbufAnimation>> PixbufAnimationExt for O {
    fn height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_height(self.as_ref().to_glib_none().0) }
    }

    fn static_image(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_animation_get_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_animation_get_width(self.as_ref().to_glib_none().0) }
    }

    fn is_static_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_animation_is_static_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for PixbufAnimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PixbufAnimation")
    }
}
