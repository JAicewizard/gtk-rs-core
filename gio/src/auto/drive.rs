// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::DriveStartFlags;
use crate::DriveStartStopType;
use crate::Icon;
use crate::MountOperation;
use crate::MountUnmountFlags;
use crate::Volume;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use libc::c_char;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GDrive")]
    pub struct Drive(Interface<ffi::GDrive, ffi::GDriveIface>);

    match fn {
        type_ => || ffi::g_drive_get_type(),
    }
}

pub const NONE_DRIVE: Option<&Drive> = None;

pub trait DriveExt: 'static {
    #[doc(alias = "g_drive_can_eject")]
    fn can_eject(&self) -> bool;

    #[doc(alias = "g_drive_can_poll_for_media")]
    fn can_poll_for_media(&self) -> bool;

    #[doc(alias = "g_drive_can_start")]
    fn can_start(&self) -> bool;

    #[doc(alias = "g_drive_can_start_degraded")]
    fn can_start_degraded(&self) -> bool;

    #[doc(alias = "g_drive_can_stop")]
    fn can_stop(&self) -> bool;

    #[doc(alias = "g_drive_eject_with_operation")]
    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_enumerate_identifiers")]
    fn enumerate_identifiers(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_drive_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Icon;

    #[doc(alias = "g_drive_get_identifier")]
    #[doc(alias = "get_identifier")]
    fn identifier<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        kind: &'s P,
    ) -> Option<glib::GString>;

    #[doc(alias = "g_drive_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_drive_get_sort_key")]
    #[doc(alias = "get_sort_key")]
    fn sort_key(&self) -> Option<glib::GString>;

    #[doc(alias = "g_drive_get_start_stop_type")]
    #[doc(alias = "get_start_stop_type")]
    fn start_stop_type(&self) -> DriveStartStopType;

    #[doc(alias = "g_drive_get_symbolic_icon")]
    #[doc(alias = "get_symbolic_icon")]
    fn symbolic_icon(&self) -> Icon;

    #[doc(alias = "g_drive_get_volumes")]
    #[doc(alias = "get_volumes")]
    fn volumes(&self) -> Vec<Volume>;

    #[doc(alias = "g_drive_has_media")]
    fn has_media(&self) -> bool;

    #[doc(alias = "g_drive_has_volumes")]
    fn has_volumes(&self) -> bool;

    #[doc(alias = "g_drive_is_media_check_automatic")]
    fn is_media_check_automatic(&self) -> bool;

    #[doc(alias = "g_drive_is_media_removable")]
    fn is_media_removable(&self) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_drive_is_removable")]
    fn is_removable(&self) -> bool;

    #[doc(alias = "g_drive_poll_for_media")]
    fn poll_for_media<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_start")]
    fn start<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn start_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_stop")]
    fn stop<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn stop_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "disconnected")]
    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "eject-button")]
    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "stop-button")]
    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Drive>> DriveExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_poll_for_media(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_poll_for_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_start(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_start(self.as_ref().to_glib_none().0)) }
    }

    fn can_start_degraded(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_start_degraded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_stop(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_stop(self.as_ref().to_glib_none().0)) }
    }

    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn eject_with_operation_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_drive_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<R>;
        unsafe {
            ffi::g_drive_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.eject_with_operation(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn enumerate_identifiers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_enumerate_identifiers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_drive_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn identifier<'s, P: ToGlibPtr<'s, *const libc::c_char> + ?Sized + 's>(
        &self,
        kind: &'s P,
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_drive_get_identifier(
                self.as_ref().to_glib_none().0,
                kind.to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_drive_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_drive_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    fn start_stop_type(&self) -> DriveStartStopType {
        unsafe {
            from_glib(ffi::g_drive_get_start_stop_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_drive_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn volumes(&self) -> Vec<Volume> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_get_volumes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_media(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_has_media(self.as_ref().to_glib_none().0)) }
    }

    fn has_volumes(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_has_volumes(self.as_ref().to_glib_none().0)) }
    }

    fn is_media_check_automatic(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_check_automatic(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_media_removable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_removable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn is_removable(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_is_removable(self.as_ref().to_glib_none().0)) }
    }

    fn poll_for_media<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn poll_for_media_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_poll_for_media_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = poll_for_media_trampoline::<Q>;
        unsafe {
            ffi::g_drive_poll_for_media(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.poll_for_media(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn start<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn start_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_start_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = start_trampoline::<R>;
        unsafe {
            ffi::g_drive_start(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn start_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.start(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn stop<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn stop_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_stop_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = stop_trampoline::<R>;
        unsafe {
            ffi::g_drive_stop(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn stop_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.stop(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disconnected")]
    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "eject-button")]
    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn eject_button_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"eject-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    eject_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stop-button")]
    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_button_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Drive")
    }
}
