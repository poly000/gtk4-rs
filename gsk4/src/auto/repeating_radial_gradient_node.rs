// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColorStop;
use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskRepeatingRadialGradientNode")]
    pub struct RepeatingRadialGradientNode(Shared<ffi::GskRepeatingRadialGradientNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for RepeatingRadialGradientNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_repeating_radial_gradient_node_get_type()) }
    }
}

impl RepeatingRadialGradientNode {
    #[doc(alias = "gsk_repeating_radial_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        color_stops: &[ColorStop],
    ) -> RepeatingRadialGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as _;
        unsafe {
            from_glib_full(ffi::gsk_repeating_radial_gradient_node_new(
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}

impl fmt::Display for RepeatingRadialGradientNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepeatingRadialGradientNode")
    }
}
