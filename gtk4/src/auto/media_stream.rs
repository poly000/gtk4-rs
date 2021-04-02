// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct MediaStream(Object<ffi::GtkMediaStream, ffi::GtkMediaStreamClass>) @implements gdk::Paintable;

    match fn {
        get_type => || ffi::gtk_media_stream_get_type(),
    }
}

pub const NONE_MEDIA_STREAM: Option<&MediaStream> = None;

pub trait MediaStreamExt: 'static {
    #[doc(alias = "gtk_media_stream_ended")]
    fn ended(&self);

    #[doc(alias = "gtk_media_stream_get_duration")]
    fn get_duration(&self) -> i64;

    #[doc(alias = "gtk_media_stream_get_ended")]
    fn get_ended(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_error")]
    fn get_error(&self) -> Option<glib::Error>;

    #[doc(alias = "gtk_media_stream_get_loop")]
    fn get_loop(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_muted")]
    fn get_muted(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_playing")]
    fn get_playing(&self) -> bool;

    #[doc(alias = "gtk_media_stream_get_timestamp")]
    fn get_timestamp(&self) -> i64;

    #[doc(alias = "gtk_media_stream_get_volume")]
    fn get_volume(&self) -> f64;

    #[doc(alias = "gtk_media_stream_has_audio")]
    fn has_audio(&self) -> bool;

    #[doc(alias = "gtk_media_stream_has_video")]
    fn has_video(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_prepared")]
    fn is_prepared(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_seekable")]
    fn is_seekable(&self) -> bool;

    #[doc(alias = "gtk_media_stream_is_seeking")]
    fn is_seeking(&self) -> bool;

    #[doc(alias = "gtk_media_stream_pause")]
    fn pause(&self);

    #[doc(alias = "gtk_media_stream_play")]
    fn play(&self);

    #[doc(alias = "gtk_media_stream_prepared")]
    fn prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64);

    #[doc(alias = "gtk_media_stream_realize")]
    fn realize(&self, surface: &gdk::Surface);

    #[doc(alias = "gtk_media_stream_seek")]
    fn seek(&self, timestamp: i64);

    #[doc(alias = "gtk_media_stream_seek_failed")]
    fn seek_failed(&self);

    #[doc(alias = "gtk_media_stream_seek_success")]
    fn seek_success(&self);

    #[doc(alias = "gtk_media_stream_set_loop")]
    fn set_loop(&self, loop_: bool);

    #[doc(alias = "gtk_media_stream_set_muted")]
    fn set_muted(&self, muted: bool);

    #[doc(alias = "gtk_media_stream_set_playing")]
    fn set_playing(&self, playing: bool);

    #[doc(alias = "gtk_media_stream_set_volume")]
    fn set_volume(&self, volume: f64);

    #[doc(alias = "gtk_media_stream_unprepared")]
    fn unprepared(&self);

    #[doc(alias = "gtk_media_stream_unrealize")]
    fn unrealize(&self, surface: &gdk::Surface);

    #[doc(alias = "gtk_media_stream_update")]
    fn update(&self, timestamp: i64);

    fn get_property_has_audio(&self) -> bool;

    fn get_property_has_video(&self) -> bool;

    fn get_property_prepared(&self) -> bool;

    fn set_property_prepared(&self, prepared: bool);

    fn get_property_seekable(&self) -> bool;

    fn get_property_seeking(&self) -> bool;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_audio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_video_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_muted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_playing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_prepared_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_seekable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_seeking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MediaStream>> MediaStreamExt for O {
    fn ended(&self) {
        unsafe {
            ffi::gtk_media_stream_ended(self.as_ref().to_glib_none().0);
        }
    }

    fn get_duration(&self) -> i64 {
        unsafe { ffi::gtk_media_stream_get_duration(self.as_ref().to_glib_none().0) }
    }

    fn get_ended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_ended(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_none(ffi::gtk_media_stream_get_error(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_loop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_loop(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_muted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_muted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_playing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_get_playing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timestamp(&self) -> i64 {
        unsafe { ffi::gtk_media_stream_get_timestamp(self.as_ref().to_glib_none().0) }
    }

    fn get_volume(&self) -> f64 {
        unsafe { ffi::gtk_media_stream_get_volume(self.as_ref().to_glib_none().0) }
    }

    fn has_audio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_has_audio(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_video(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_has_video(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_prepared(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_prepared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_seekable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_seeking(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_media_stream_is_seeking(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pause(&self) {
        unsafe {
            ffi::gtk_media_stream_pause(self.as_ref().to_glib_none().0);
        }
    }

    fn play(&self) {
        unsafe {
            ffi::gtk_media_stream_play(self.as_ref().to_glib_none().0);
        }
    }

    fn prepared(&self, has_audio: bool, has_video: bool, seekable: bool, duration: i64) {
        unsafe {
            ffi::gtk_media_stream_prepared(
                self.as_ref().to_glib_none().0,
                has_audio.to_glib(),
                has_video.to_glib(),
                seekable.to_glib(),
                duration,
            );
        }
    }

    fn realize(&self, surface: &gdk::Surface) {
        unsafe {
            ffi::gtk_media_stream_realize(self.as_ref().to_glib_none().0, surface.to_glib_none().0);
        }
    }

    fn seek(&self, timestamp: i64) {
        unsafe {
            ffi::gtk_media_stream_seek(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn seek_failed(&self) {
        unsafe {
            ffi::gtk_media_stream_seek_failed(self.as_ref().to_glib_none().0);
        }
    }

    fn seek_success(&self) {
        unsafe {
            ffi::gtk_media_stream_seek_success(self.as_ref().to_glib_none().0);
        }
    }

    fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gtk_media_stream_set_loop(self.as_ref().to_glib_none().0, loop_.to_glib());
        }
    }

    fn set_muted(&self, muted: bool) {
        unsafe {
            ffi::gtk_media_stream_set_muted(self.as_ref().to_glib_none().0, muted.to_glib());
        }
    }

    fn set_playing(&self, playing: bool) {
        unsafe {
            ffi::gtk_media_stream_set_playing(self.as_ref().to_glib_none().0, playing.to_glib());
        }
    }

    fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::gtk_media_stream_set_volume(self.as_ref().to_glib_none().0, volume);
        }
    }

    fn unprepared(&self) {
        unsafe {
            ffi::gtk_media_stream_unprepared(self.as_ref().to_glib_none().0);
        }
    }

    fn unrealize(&self, surface: &gdk::Surface) {
        unsafe {
            ffi::gtk_media_stream_unrealize(
                self.as_ref().to_glib_none().0,
                surface.to_glib_none().0,
            );
        }
    }

    fn update(&self, timestamp: i64) {
        unsafe {
            ffi::gtk_media_stream_update(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn get_property_has_audio(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"has-audio\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-audio` getter")
                .unwrap()
        }
    }

    fn get_property_has_video(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"has-video\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-video` getter")
                .unwrap()
        }
    }

    fn get_property_prepared(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"prepared\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `prepared` getter")
                .unwrap()
        }
    }

    fn set_property_prepared(&self, prepared: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"prepared\0".as_ptr() as *const _,
                glib::Value::from(&prepared).to_glib_none().0,
            );
        }
    }

    fn get_property_seekable(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"seekable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `seekable` getter")
                .unwrap()
        }
    }

    fn get_property_seeking(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"seeking\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `seeking` getter")
                .unwrap()
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ended_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ended\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ended_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_error_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_error_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_audio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_audio_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-audio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_audio_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_video_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_video_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-video\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_video_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loop_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::loop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_muted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_muted_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::muted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_muted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_playing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_playing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::playing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_playing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_prepared_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_prepared_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::prepared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_prepared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_seekable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seekable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seekable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seekable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_seeking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seeking_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seeking\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seeking_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timestamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMediaStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MediaStream>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MediaStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MediaStream")
    }
}
