// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use crate::Bytes;
use crate::Error;
use crate::KeyFileFlags;
use libc::c_char;
use std::mem;
use std::ptr;

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct KeyFile(Shared<ffi::GKeyFile>);

    match fn {
        ref => |ptr| ffi::g_key_file_ref(ptr),
        unref => |ptr| ffi::g_key_file_unref(ptr),
        type_ => || ffi::g_key_file_get_type(),
    }
}

impl KeyFile {
    #[doc(alias = "g_key_file_new")]
    pub fn new() -> KeyFile {
        unsafe { from_glib_full(ffi::g_key_file_new()) }
    }

    #[doc(alias = "g_key_file_get_comment")]
    #[doc(alias = "get_comment")]
    pub fn comment(
        &self,
        group_name: Option<&str>,
        key: Option<&str>,
    ) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_comment(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_double")]
    #[doc(alias = "get_double")]
    pub fn double<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<f64, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_double(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_double_list")]
    #[doc(alias = "get_double_list")]
    pub fn double_list<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<Vec<f64>, crate::Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_double_list(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_container_num(
                    ret,
                    length.assume_init() as usize,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_groups")]
    #[doc(alias = "get_groups")]
    pub fn groups(&self) -> (Vec<crate::GString>, usize) {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::g_key_file_get_groups(
                self.to_glib_none().0,
                length.as_mut_ptr(),
            ));
            let length = length.assume_init();
            (ret, length)
        }
    }

    #[doc(alias = "g_key_file_get_int64")]
    #[doc(alias = "get_int64")]
    pub fn int64<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<i64, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_int64(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_integer")]
    #[doc(alias = "get_integer")]
    pub fn integer<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<i32, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_integer(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_integer_list")]
    #[doc(alias = "get_integer_list")]
    pub fn integer_list<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<Vec<i32>, crate::Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_integer_list(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_container_num(
                    ret,
                    length.assume_init() as usize,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_keys")]
    #[doc(alias = "get_keys")]
    pub fn keys<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        group_name: &'s P,
    ) -> Result<(Vec<crate::GString>, usize), crate::Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_keys(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            let length = length.assume_init();
            if error.is_null() {
                Ok((FromGlibPtrContainer::from_glib_full(ret), length))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_key_file_get_locale_for_key")]
    #[doc(alias = "get_locale_for_key")]
    pub fn locale_for_key<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        locale: Option<&str>,
    ) -> Option<crate::GString> {
        unsafe {
            from_glib_full(ffi::g_key_file_get_locale_for_key(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                locale.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_key_file_get_start_group")]
    #[doc(alias = "get_start_group")]
    pub fn start_group(&self) -> Option<crate::GString> {
        unsafe { from_glib_full(ffi::g_key_file_get_start_group(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_key_file_get_uint64")]
    #[doc(alias = "get_uint64")]
    pub fn uint64<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<u64, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_uint64(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_value")]
    #[doc(alias = "get_value")]
    pub fn value<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_value(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_has_group")]
    pub fn has_group<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        group_name: &'s P,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_key_file_has_group(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_key_file_load_from_bytes")]
    pub fn load_from_bytes(&self, bytes: &Bytes, flags: KeyFileFlags) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_bytes(
                self.to_glib_none().0,
                bytes.to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_load_from_data")]
    pub fn load_from_data<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        data: &'s P,
        flags: KeyFileFlags,
    ) -> Result<(), crate::Error> {
        let length = data.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_data(
                self.to_glib_none().0,
                data.to_glib_none().0,
                length,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_load_from_file")]
    pub fn load_from_file<P: AsRef<std::path::Path>>(
        &self,
        file: P,
        flags: KeyFileFlags,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_file(
                self.to_glib_none().0,
                file.as_ref().to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_remove_comment")]
    pub fn remove_comment(
        &self,
        group_name: Option<&str>,
        key: Option<&str>,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_comment(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_remove_group")]
    pub fn remove_group<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        group_name: &'s P,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_group(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_remove_key")]
    pub fn remove_key<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_key(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_set_boolean")]
    pub fn set_boolean<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: bool,
    ) {
        unsafe {
            ffi::g_key_file_set_boolean(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    //#[doc(alias = "g_key_file_set_boolean_list")]
    //pub fn set_boolean_list<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's, Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(&self, group_name: & 's P, key: & 's Q, list: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 1 }) {
    //    unsafe { TODO: call ffi:g_key_file_set_boolean_list() }
    //}

    #[doc(alias = "g_key_file_set_comment")]
    pub fn set_comment<'s, P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's>(
        &self,
        group_name: Option<&str>,
        key: Option<&str>,
        comment: &'s P,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_set_comment(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                comment.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_set_double")]
    pub fn set_double<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: f64,
    ) {
        unsafe {
            ffi::g_key_file_set_double(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "g_key_file_set_int64")]
    pub fn set_int64<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: i64,
    ) {
        unsafe {
            ffi::g_key_file_set_int64(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "g_key_file_set_integer")]
    pub fn set_integer<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: i32,
    ) {
        unsafe {
            ffi::g_key_file_set_integer(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "g_key_file_set_list_separator")]
    pub fn set_list_separator(&self, separator: crate::Char) {
        unsafe {
            ffi::g_key_file_set_list_separator(self.to_glib_none().0, separator.into_glib());
        }
    }

    #[doc(alias = "g_key_file_set_locale_string")]
    pub fn set_locale_string<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        R: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        S: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        locale: &'s R,
        string: &'s S,
    ) {
        unsafe {
            ffi::g_key_file_set_locale_string(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                locale.to_glib_none().0,
                string.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_key_file_set_string")]
    pub fn set_string<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        R: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        string: &'s R,
    ) {
        unsafe {
            ffi::g_key_file_set_string(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                string.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_key_file_set_uint64")]
    pub fn set_uint64<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: u64,
    ) {
        unsafe {
            ffi::g_key_file_set_uint64(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "g_key_file_set_value")]
    pub fn set_value<
        's,
        P: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        Q: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
        R: ToGlibPtr<'s, *mut libc::c_char> + ?Sized + 's,
    >(
        &self,
        group_name: &'s P,
        key: &'s Q,
        value: &'s R,
    ) {
        unsafe {
            ffi::g_key_file_set_value(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl Default for KeyFile {
    fn default() -> Self {
        Self::new()
    }
}
