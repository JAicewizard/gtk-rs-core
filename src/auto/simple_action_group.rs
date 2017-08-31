// This file was generated by gir (e50fd21) from gir-files (db49619)
// DO NOT EDIT

use Action;
use ActionGroup;
use ActionMap;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SimpleActionGroup(Object<ffi::GSimpleActionGroup>): ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_simple_action_group_get_type(),
    }
}

impl SimpleActionGroup {
    pub fn new() -> SimpleActionGroup {
        unsafe {
            from_glib_full(ffi::g_simple_action_group_new())
        }
    }
}

impl Default for SimpleActionGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SimpleActionGroupExt {
    //fn add_entries<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, entries: /*Ignored*/&[&ActionEntry], user_data: P);

    fn insert<P: IsA<Action>>(&self, action: &P);

    fn lookup(&self, action_name: &str) -> Option<Action>;

    fn remove(&self, action_name: &str);
}

impl<O: IsA<SimpleActionGroup>> SimpleActionGroupExt for O {
    //fn add_entries<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, entries: /*Ignored*/&[&ActionEntry], user_data: P) {
    //    unsafe { TODO: call ffi::g_simple_action_group_add_entries() }
    //}

    fn insert<P: IsA<Action>>(&self, action: &P) {
        unsafe {
            ffi::g_simple_action_group_insert(self.to_glib_none().0, action.to_glib_none().0);
        }
    }

    fn lookup(&self, action_name: &str) -> Option<Action> {
        unsafe {
            from_glib_none(ffi::g_simple_action_group_lookup(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn remove(&self, action_name: &str) {
        unsafe {
            ffi::g_simple_action_group_remove(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }
}
