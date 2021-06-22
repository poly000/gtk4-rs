// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::PropagationLimit;
use crate::PropagationPhase;
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
    #[doc(alias = "GtkGestureZoom")]
    pub struct GestureZoom(Object<ffi::GtkGestureZoom, ffi::GtkGestureZoomClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[doc(alias = "gtk_gesture_zoom_new")]
    pub fn new() -> GestureZoom {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_zoom_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureZoom`] objects.
    ///
    /// This method returns an instance of [`GestureZoomBuilder`] which can be used to create [`GestureZoom`] objects.
    pub fn builder() -> GestureZoomBuilder {
        GestureZoomBuilder::default()
    }

    #[doc(alias = "gtk_gesture_zoom_get_scale_delta")]
    #[doc(alias = "get_scale_delta")]
    pub fn scale_delta(&self) -> f64 {
        unsafe { ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0) }
    }

    #[doc(alias = "scale-changed")]
    pub fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scale_changed_trampoline<F: Fn(&GestureZoom, f64) + 'static>(
            this: *mut ffi::GtkGestureZoom,
            scale: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), scale)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scale-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scale_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureZoom {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureZoom`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct GestureZoomBuilder {
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureZoomBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureZoomBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureZoom`].
    pub fn build(self) -> GestureZoom {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<GestureZoom>(&properties)
            .expect("Failed to create an instance of GestureZoom")
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for GestureZoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureZoom")
    }
}
