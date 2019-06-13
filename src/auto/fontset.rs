// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use pango_sys;
use std::fmt;
use Font;
use FontMetrics;

glib_wrapper! {
    pub struct Fontset(Object<pango_sys::PangoFontset, pango_sys::PangoFontsetClass, FontsetClass>);

    match fn {
        get_type => || pango_sys::pango_fontset_get_type(),
    }
}

pub const NONE_FONTSET: Option<&Fontset> = None;

pub trait FontsetExt: 'static {
    fn foreach<P: FnMut(&Fontset, &Font) -> bool>(&self, func: P);

    fn get_font(&self, wc: u32) -> Option<Font>;

    fn get_metrics(&self) -> Option<FontMetrics>;
}

impl<O: IsA<Fontset>> FontsetExt for O {
    fn foreach<P: FnMut(&Fontset, &Font) -> bool>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Fontset, &Font) -> bool>(
            fontset: *mut pango_sys::PangoFontset,
            font: *mut pango_sys::PangoFont,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let fontset = from_glib_borrow(fontset);
            let font = from_glib_borrow(font);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&fontset, &font);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            pango_sys::pango_fontset_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn get_font(&self, wc: u32) -> Option<Font> {
        unsafe {
            from_glib_full(pango_sys::pango_fontset_get_font(
                self.as_ref().to_glib_none().0,
                wc,
            ))
        }
    }

    fn get_metrics(&self) -> Option<FontMetrics> {
        unsafe {
            from_glib_full(pango_sys::pango_fontset_get_metrics(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Fontset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fontset")
    }
}
