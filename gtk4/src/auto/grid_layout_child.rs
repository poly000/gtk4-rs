// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::LayoutChild;
use crate::LayoutManager;
use crate::Widget;
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
    pub struct GridLayoutChild(Object<ffi::GtkGridLayoutChild, ffi::GtkGridLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || ffi::gtk_grid_layout_child_get_type(),
    }
}

impl GridLayoutChild {
    #[doc(alias = "gtk_grid_layout_child_get_column")]
    pub fn get_column(&self) -> i32 {
        unsafe { ffi::gtk_grid_layout_child_get_column(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_child_get_column_span")]
    pub fn get_column_span(&self) -> i32 {
        unsafe { ffi::gtk_grid_layout_child_get_column_span(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_child_get_row")]
    pub fn get_row(&self) -> i32 {
        unsafe { ffi::gtk_grid_layout_child_get_row(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_child_get_row_span")]
    pub fn get_row_span(&self) -> i32 {
        unsafe { ffi::gtk_grid_layout_child_get_row_span(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_child_set_column")]
    pub fn set_column(&self, column: i32) {
        unsafe {
            ffi::gtk_grid_layout_child_set_column(self.to_glib_none().0, column);
        }
    }

    #[doc(alias = "gtk_grid_layout_child_set_column_span")]
    pub fn set_column_span(&self, span: i32) {
        unsafe {
            ffi::gtk_grid_layout_child_set_column_span(self.to_glib_none().0, span);
        }
    }

    #[doc(alias = "gtk_grid_layout_child_set_row")]
    pub fn set_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_layout_child_set_row(self.to_glib_none().0, row);
        }
    }

    #[doc(alias = "gtk_grid_layout_child_set_row_span")]
    pub fn set_row_span(&self, span: i32) {
        unsafe {
            ffi::gtk_grid_layout_child_set_row_span(self.to_glib_none().0, span);
        }
    }

    pub fn connect_property_column_notify<F: Fn(&GridLayoutChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_trampoline<F: Fn(&GridLayoutChild) + 'static>(
            this: *mut ffi::GtkGridLayoutChild,
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
                b"notify::column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_column_span_notify<F: Fn(&GridLayoutChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_span_trampoline<F: Fn(&GridLayoutChild) + 'static>(
            this: *mut ffi::GtkGridLayoutChild,
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
                b"notify::column-span\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_span_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_row_notify<F: Fn(&GridLayoutChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_trampoline<F: Fn(&GridLayoutChild) + 'static>(
            this: *mut ffi::GtkGridLayoutChild,
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
                b"notify::row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_row_span_notify<F: Fn(&GridLayoutChild) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_span_trampoline<F: Fn(&GridLayoutChild) + 'static>(
            this: *mut ffi::GtkGridLayoutChild,
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
                b"notify::row-span\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_span_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct GridLayoutChildBuilder {
    column: Option<i32>,
    column_span: Option<i32>,
    row: Option<i32>,
    row_span: Option<i32>,
    child_widget: Option<Widget>,
    layout_manager: Option<LayoutManager>,
}

impl GridLayoutChildBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GridLayoutChild {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref column) = self.column {
            properties.push(("column", column));
        }
        if let Some(ref column_span) = self.column_span {
            properties.push(("column-span", column_span));
        }
        if let Some(ref row) = self.row {
            properties.push(("row", row));
        }
        if let Some(ref row_span) = self.row_span {
            properties.push(("row-span", row_span));
        }
        if let Some(ref child_widget) = self.child_widget {
            properties.push(("child-widget", child_widget));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        let ret = glib::Object::new::<GridLayoutChild>(&properties).expect("object new");
        ret
    }

    pub fn column(mut self, column: i32) -> Self {
        self.column = Some(column);
        self
    }

    pub fn column_span(mut self, column_span: i32) -> Self {
        self.column_span = Some(column_span);
        self
    }

    pub fn row(mut self, row: i32) -> Self {
        self.row = Some(row);
        self
    }

    pub fn row_span(mut self, row_span: i32) -> Self {
        self.row_span = Some(row_span);
        self
    }

    pub fn child_widget<P: IsA<Widget>>(mut self, child_widget: &P) -> Self {
        self.child_widget = Some(child_widget.clone().upcast());
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }
}

impl fmt::Display for GridLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GridLayoutChild")
    }
}
