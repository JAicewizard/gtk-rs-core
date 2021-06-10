// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AppInfo;
use crate::AppLaunchContext;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_60", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
use std::mem;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GDesktopAppInfo")]
    pub struct DesktopAppInfo(Object<ffi::GDesktopAppInfo, ffi::GDesktopAppInfoClass>) @implements AppInfo;

    match fn {
        type_ => || ffi::g_desktop_app_info_get_type(),
    }
}

impl DesktopAppInfo {
    #[doc(alias = "g_desktop_app_info_new")]
    pub fn new<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        desktop_id: &'s P,
    ) -> Option<DesktopAppInfo> {
        unsafe { from_glib_full(ffi::g_desktop_app_info_new(desktop_id.to_glib_none().0)) }
    }

    #[doc(alias = "g_desktop_app_info_new_from_filename")]
    #[doc(alias = "new_from_filename")]
    pub fn from_filename<P: AsRef<std::path::Path>>(filename: P) -> Option<DesktopAppInfo> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_new_from_filename(
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_new_from_keyfile")]
    #[doc(alias = "new_from_keyfile")]
    pub fn from_keyfile(key_file: &glib::KeyFile) -> Option<DesktopAppInfo> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_new_from_keyfile(
                key_file.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_action_name")]
    #[doc(alias = "get_action_name")]
    pub fn action_name<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        action_name: &'s P,
    ) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_action_name(
                self.to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_boolean")]
    #[doc(alias = "get_boolean")]
    pub fn boolean<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        key: &'s P,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_boolean(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_categories")]
    #[doc(alias = "get_categories")]
    pub fn categories(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_categories(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_filename")]
    #[doc(alias = "get_filename")]
    pub fn filename(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(ffi::g_desktop_app_info_get_filename(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_desktop_app_info_get_generic_name")]
    #[doc(alias = "get_generic_name")]
    pub fn generic_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_generic_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_is_hidden")]
    #[doc(alias = "get_is_hidden")]
    pub fn is_hidden(&self) -> bool {
        unsafe { from_glib(ffi::g_desktop_app_info_get_is_hidden(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_desktop_app_info_get_keywords")]
    #[doc(alias = "get_keywords")]
    pub fn keywords(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_desktop_app_info_get_keywords(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_desktop_app_info_get_locale_string")]
    #[doc(alias = "get_locale_string")]
    pub fn locale_string<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        key: &'s P,
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_locale_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_nodisplay")]
    #[doc(alias = "get_nodisplay")]
    pub fn is_nodisplay(&self) -> bool {
        unsafe { from_glib(ffi::g_desktop_app_info_get_nodisplay(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_desktop_app_info_get_show_in")]
    #[doc(alias = "get_show_in")]
    pub fn shows_in(&self, desktop_env: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_get_show_in(
                self.to_glib_none().0,
                desktop_env.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_startup_wm_class")]
    #[doc(alias = "get_startup_wm_class")]
    pub fn startup_wm_class(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_desktop_app_info_get_startup_wm_class(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_string")]
    #[doc(alias = "get_string")]
    pub fn string<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        key: &'s P,
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_desktop_app_info_get_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_desktop_app_info_get_string_list")]
    #[doc(alias = "get_string_list")]
    pub fn string_list<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        key: &'s P,
    ) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::g_desktop_app_info_get_string_list(
                    self.to_glib_none().0,
                    key.to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "g_desktop_app_info_has_key")]
    pub fn has_key<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        key: &'s P,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_desktop_app_info_has_key(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_launch_action")]
    pub fn launch_action<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: IsA<AppLaunchContext>,
    >(
        &self,
        action_name: &'s P,
        launch_context: Option<&Q>,
    ) {
        unsafe {
            ffi::g_desktop_app_info_launch_action(
                self.to_glib_none().0,
                action_name.to_glib_none().0,
                launch_context.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_desktop_app_info_launch_uris_as_manager")]
    pub fn launch_uris_as_manager<P: IsA<AppLaunchContext>>(
        &self,
        uris: &[&str],
        launch_context: Option<&P>,
        spawn_flags: glib::SpawnFlags,
        user_setup: Option<Box_<dyn FnOnce() + 'static>>,
        pid_callback: Option<&mut dyn (FnMut(&DesktopAppInfo, glib::Pid))>,
    ) -> Result<(), glib::Error> {
        let user_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(user_setup);
        unsafe extern "C" fn user_setup_func<P: IsA<AppLaunchContext>>(
            user_data: glib::ffi::gpointer,
        ) {
            let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
                Box_::from_raw(user_data as *mut _);
            let callback = (*callback).expect("cannot get closure...");
            callback()
        }
        let user_setup = if user_setup_data.is_some() {
            Some(user_setup_func::<P> as _)
        } else {
            None
        };
        let pid_callback_data: Option<&mut dyn (FnMut(&DesktopAppInfo, glib::Pid))> = pid_callback;
        unsafe extern "C" fn pid_callback_func<P: IsA<AppLaunchContext>>(
            appinfo: *mut ffi::GDesktopAppInfo,
            pid: glib::ffi::GPid,
            user_data: glib::ffi::gpointer,
        ) {
            let appinfo = from_glib_borrow(appinfo);
            let pid = from_glib(pid);
            let callback: *mut Option<&mut dyn (FnMut(&DesktopAppInfo, glib::Pid))> =
                user_data as *const _ as usize
                    as *mut Option<&mut dyn (FnMut(&DesktopAppInfo, glib::Pid))>;
            if let Some(ref mut callback) = *callback {
                callback(&appinfo, pid)
            } else {
                panic!("cannot get closure...")
            };
        }
        let pid_callback = if pid_callback_data.is_some() {
            Some(pid_callback_func::<P> as _)
        } else {
            None
        };
        let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = user_setup_data;
        let super_callback1: &Option<&mut dyn (FnMut(&DesktopAppInfo, glib::Pid))> =
            &pid_callback_data;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_desktop_app_info_launch_uris_as_manager(
                self.to_glib_none().0,
                uris.to_glib_none().0,
                launch_context.map(|p| p.as_ref()).to_glib_none().0,
                spawn_flags.into_glib(),
                user_setup,
                Box_::into_raw(super_callback0) as *mut _,
                pid_callback,
                super_callback1 as *const _ as usize as *mut _,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_desktop_app_info_list_actions")]
    pub fn list_actions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_desktop_app_info_list_actions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_desktop_app_info_get_implementations")]
    #[doc(alias = "get_implementations")]
    pub fn implementations<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        interface: &'s P,
    ) -> Vec<DesktopAppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_desktop_app_info_get_implementations(
                interface.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for DesktopAppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DesktopAppInfo")
    }
}
