// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct WaylandDevice(Object<ffi::GdkWaylandDevice, ffi::GdkWaylandDeviceClass>) @extends gdk::Device;

    match fn {
        get_type => || ffi::gdk_wayland_device_get_type(),
    }
}

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_get_node_path")]
    pub fn get_node_path(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_wayland_device_get_node_path(self.to_glib_none().0)) }
    }
}

impl fmt::Display for WaylandDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WaylandDevice")
    }
}
