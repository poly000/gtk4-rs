// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TreeListRow;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct TreeListModel(Object<ffi::GtkTreeListModel, ffi::GtkTreeListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_tree_list_model_get_type(),
    }
}

impl TreeListModel {
    #[doc(alias = "gtk_tree_list_model_new")]
    pub fn new<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static>(
        root: &P,
        passthrough: bool,
        autoexpand: bool,
        create_func: Q,
    ) -> TreeListModel {
        assert_initialized_main_thread!();
        let create_func_data: Box_<Q> = Box_::new(create_func);
        unsafe extern "C" fn create_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static,
        >(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut gio::ffi::GListModel {
            let item = from_glib_borrow(item);
            let callback: &Q = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_func = Some(create_func_func::<P, Q> as _);
        unsafe extern "C" fn user_destroy_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call5 = Some(user_destroy_func::<P, Q> as _);
        let super_callback0: Box_<Q> = create_func_data;
        unsafe {
            from_glib_full(ffi::gtk_tree_list_model_new(
                root.as_ref().to_glib_full(),
                passthrough.to_glib(),
                autoexpand.to_glib(),
                create_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call5,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_model_get_autoexpand")]
    pub fn get_autoexpand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_list_model_get_autoexpand(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_model_get_child_row")]
    pub fn get_child_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(ffi::gtk_tree_list_model_get_child_row(
                self.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_model_get_model")]
    pub fn get_model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_tree_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_model_get_passthrough")]
    pub fn get_passthrough(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_list_model_get_passthrough(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_model_get_row")]
    pub fn get_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(ffi::gtk_tree_list_model_get_row(
                self.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_model_set_autoexpand")]
    pub fn set_autoexpand(&self, autoexpand: bool) {
        unsafe {
            ffi::gtk_tree_list_model_set_autoexpand(self.to_glib_none().0, autoexpand.to_glib());
        }
    }

    pub fn connect_property_autoexpand_notify<F: Fn(&TreeListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoexpand_trampoline<F: Fn(&TreeListModel) + 'static>(
            this: *mut ffi::GtkTreeListModel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::autoexpand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoexpand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&TreeListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&TreeListModel) + 'static>(
            this: *mut ffi::GtkTreeListModel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct TreeListModelBuilder {
    autoexpand: Option<bool>,
    passthrough: Option<bool>,
}

impl TreeListModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeListModel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref autoexpand) = self.autoexpand {
            properties.push(("autoexpand", autoexpand));
        }
        if let Some(ref passthrough) = self.passthrough {
            properties.push(("passthrough", passthrough));
        }
        let ret = glib::Object::new::<TreeListModel>(&properties).expect("object new");
        ret
    }

    pub fn autoexpand(mut self, autoexpand: bool) -> Self {
        self.autoexpand = Some(autoexpand);
        self
    }

    pub fn passthrough(mut self, passthrough: bool) -> Self {
        self.passthrough = Some(passthrough);
        self
    }
}

impl fmt::Display for TreeListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeListModel")
    }
}
