// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColorStop;
use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskRepeatingLinearGradientNode")]
    pub struct RepeatingLinearGradientNode(Shared<ffi::GskRepeatingLinearGradientNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for RepeatingLinearGradientNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_repeating_linear_gradient_node_get_type()) }
    }
}

impl RepeatingLinearGradientNode {
    #[doc(alias = "gsk_repeating_linear_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        start: &graphene::Point,
        end: &graphene::Point,
        color_stops: &[ColorStop],
    ) -> RepeatingLinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as _;
        unsafe {
            from_glib_full(ffi::gsk_repeating_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}

impl fmt::Display for RepeatingLinearGradientNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepeatingLinearGradientNode")
    }
}
