// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BaselinePosition;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use glib::object::Cast;
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
    pub struct BoxLayout(Object<ffi::GtkBoxLayout, ffi::GtkBoxLayoutClass>) @extends LayoutManager, @implements Orientable;

    match fn {
        get_type => || ffi::gtk_box_layout_get_type(),
    }
}

impl BoxLayout {
    #[doc(alias = "gtk_box_layout_new")]
    pub fn new(orientation: Orientation) -> BoxLayout {
        assert_initialized_main_thread!();
        unsafe {
            LayoutManager::from_glib_full(ffi::gtk_box_layout_new(orientation.to_glib()))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_box_layout_get_baseline_position")]
    pub fn get_baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_layout_get_baseline_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_box_layout_get_homogeneous")]
    pub fn get_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_box_layout_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_box_layout_get_spacing")]
    pub fn get_spacing(&self) -> u32 {
        unsafe { ffi::gtk_box_layout_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_box_layout_set_baseline_position")]
    pub fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_layout_set_baseline_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_layout_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_spacing")]
    pub fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_box_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn connect_property_baseline_position_notify<F: Fn(&BoxLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_position_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                b"notify::baseline-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_baseline_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_homogeneous_notify<F: Fn(&BoxLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_spacing_notify<F: Fn(&BoxLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct BoxLayoutBuilder {
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    orientation: Option<Orientation>,
}

impl BoxLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BoxLayout {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<BoxLayout>(&properties).expect("object new");
        ret
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BoxLayout")
    }
}
