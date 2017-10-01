// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

use ClockTime;
use Object;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Clock(Object<ffi::GstClock>): Object;

    match fn {
        get_type => || ffi::gst_clock_get_type(),
    }
}

impl Clock {
    //pub fn id_compare_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(id1: P, id2: Q) -> i32 {
    //    unsafe { TODO: call ffi::gst_clock_id_compare_func() }
    //}

    //pub fn id_get_time(id: /*Unimplemented*/ClockID) -> ClockTime {
    //    unsafe { TODO: call ffi::gst_clock_id_get_time() }
    //}

    //pub fn id_ref(id: /*Unimplemented*/ClockID) -> /*Unimplemented*/Option<ClockID> {
    //    unsafe { TODO: call ffi::gst_clock_id_ref() }
    //}

    //pub fn id_unref(id: /*Unimplemented*/ClockID) {
    //    unsafe { TODO: call ffi::gst_clock_id_unref() }
    //}

    //pub fn id_unschedule(id: /*Unimplemented*/ClockID) {
    //    unsafe { TODO: call ffi::gst_clock_id_unschedule() }
    //}

    //pub fn id_wait(id: /*Unimplemented*/ClockID) -> (ClockReturn, ClockTimeDiff) {
    //    unsafe { TODO: call ffi::gst_clock_id_wait() }
    //}

    //pub fn id_wait_async<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(id: /*Unimplemented*/ClockID, func: /*Unknown conversion*//*Unimplemented*/ClockCallback, user_data: P, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> ClockReturn {
    //    unsafe { TODO: call ffi::gst_clock_id_wait_async() }
    //}
}

unsafe impl Send for Clock {}
unsafe impl Sync for Clock {}

pub trait ClockExt {
    fn add_observation(&self, slave: ClockTime, master: ClockTime) -> Option<f64>;

    fn add_observation_unapplied(&self, slave: ClockTime, master: ClockTime) -> Option<(f64, ClockTime, ClockTime, ClockTime, ClockTime)>;

    fn adjust_unlocked(&self, internal: ClockTime) -> ClockTime;

    fn adjust_with_calibration(&self, internal_target: ClockTime, cinternal: ClockTime, cexternal: ClockTime, cnum: ClockTime, cdenom: ClockTime) -> ClockTime;

    fn get_calibration(&self) -> (ClockTime, ClockTime, ClockTime, ClockTime);

    fn get_internal_time(&self) -> ClockTime;

    fn get_master(&self) -> Option<Clock>;

    fn get_resolution(&self) -> ClockTime;

    fn get_time(&self) -> ClockTime;

    fn get_timeout(&self) -> ClockTime;

    fn is_synced(&self) -> bool;

    //fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> /*Unimplemented*/Option<ClockID>;

    //fn new_single_shot_id(&self, time: ClockTime) -> /*Unimplemented*/Option<ClockID>;

    //fn periodic_id_reinit(&self, id: /*Unimplemented*/ClockID, start_time: ClockTime, interval: ClockTime) -> bool;

    fn set_calibration(&self, internal: ClockTime, external: ClockTime, rate_num: ClockTime, rate_denom: ClockTime);

    fn set_master<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, master: Q) -> Result<(), glib::error::BoolError>;

    fn set_resolution(&self, resolution: ClockTime) -> ClockTime;

    fn set_synced(&self, synced: bool);

    fn set_timeout(&self, timeout: ClockTime);

    //fn single_shot_id_reinit(&self, id: /*Unimplemented*/ClockID, time: ClockTime) -> bool;

    fn unadjust_unlocked(&self, external: ClockTime) -> ClockTime;

    fn unadjust_with_calibration(&self, external_target: ClockTime, cinternal: ClockTime, cexternal: ClockTime, cnum: ClockTime, cdenom: ClockTime) -> ClockTime;

    fn wait_for_sync(&self, timeout: ClockTime) -> Result<(), glib::error::BoolError>;

    fn get_property_window_size(&self) -> i32;

    fn set_property_window_size(&self, window_size: i32);

    fn get_property_window_threshold(&self) -> i32;

    fn set_property_window_threshold(&self, window_threshold: i32);

    fn connect_synced<F: Fn(&Self, bool) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Clock> + IsA<glib::object::Object>> ClockExt for O {
    fn add_observation(&self, slave: ClockTime, master: ClockTime) -> Option<f64> {
        unsafe {
            let mut r_squared = mem::uninitialized();
            let ret = from_glib(ffi::gst_clock_add_observation(self.to_glib_none().0, slave, master, &mut r_squared));
            if ret { Some(r_squared) } else { None }
        }
    }

    fn add_observation_unapplied(&self, slave: ClockTime, master: ClockTime) -> Option<(f64, ClockTime, ClockTime, ClockTime, ClockTime)> {
        unsafe {
            let mut r_squared = mem::uninitialized();
            let mut internal = mem::uninitialized();
            let mut external = mem::uninitialized();
            let mut rate_num = mem::uninitialized();
            let mut rate_denom = mem::uninitialized();
            let ret = from_glib(ffi::gst_clock_add_observation_unapplied(self.to_glib_none().0, slave, master, &mut r_squared, &mut internal, &mut external, &mut rate_num, &mut rate_denom));
            if ret { Some((r_squared, internal, external, rate_num, rate_denom)) } else { None }
        }
    }

    fn adjust_unlocked(&self, internal: ClockTime) -> ClockTime {
        unsafe {
            ffi::gst_clock_adjust_unlocked(self.to_glib_none().0, internal)
        }
    }

    fn adjust_with_calibration(&self, internal_target: ClockTime, cinternal: ClockTime, cexternal: ClockTime, cnum: ClockTime, cdenom: ClockTime) -> ClockTime {
        unsafe {
            ffi::gst_clock_adjust_with_calibration(self.to_glib_none().0, internal_target, cinternal, cexternal, cnum, cdenom)
        }
    }

    fn get_calibration(&self) -> (ClockTime, ClockTime, ClockTime, ClockTime) {
        unsafe {
            let mut internal = mem::uninitialized();
            let mut external = mem::uninitialized();
            let mut rate_num = mem::uninitialized();
            let mut rate_denom = mem::uninitialized();
            ffi::gst_clock_get_calibration(self.to_glib_none().0, &mut internal, &mut external, &mut rate_num, &mut rate_denom);
            (internal, external, rate_num, rate_denom)
        }
    }

    fn get_internal_time(&self) -> ClockTime {
        unsafe {
            ffi::gst_clock_get_internal_time(self.to_glib_none().0)
        }
    }

    fn get_master(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_clock_get_master(self.to_glib_none().0))
        }
    }

    fn get_resolution(&self) -> ClockTime {
        unsafe {
            ffi::gst_clock_get_resolution(self.to_glib_none().0)
        }
    }

    fn get_time(&self) -> ClockTime {
        unsafe {
            ffi::gst_clock_get_time(self.to_glib_none().0)
        }
    }

    fn get_timeout(&self) -> ClockTime {
        unsafe {
            ffi::gst_clock_get_timeout(self.to_glib_none().0)
        }
    }

    fn is_synced(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_clock_is_synced(self.to_glib_none().0))
        }
    }

    //fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> /*Unimplemented*/Option<ClockID> {
    //    unsafe { TODO: call ffi::gst_clock_new_periodic_id() }
    //}

    //fn new_single_shot_id(&self, time: ClockTime) -> /*Unimplemented*/Option<ClockID> {
    //    unsafe { TODO: call ffi::gst_clock_new_single_shot_id() }
    //}

    //fn periodic_id_reinit(&self, id: /*Unimplemented*/ClockID, start_time: ClockTime, interval: ClockTime) -> bool {
    //    unsafe { TODO: call ffi::gst_clock_periodic_id_reinit() }
    //}

    fn set_calibration(&self, internal: ClockTime, external: ClockTime, rate_num: ClockTime, rate_denom: ClockTime) {
        unsafe {
            ffi::gst_clock_set_calibration(self.to_glib_none().0, internal, external, rate_num, rate_denom);
        }
    }

    fn set_master<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, master: Q) -> Result<(), glib::error::BoolError> {
        let master = master.into();
        let master = master.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_clock_set_master(self.to_glib_none().0, master.0), "Failed to set master clock")
        }
    }

    fn set_resolution(&self, resolution: ClockTime) -> ClockTime {
        unsafe {
            ffi::gst_clock_set_resolution(self.to_glib_none().0, resolution)
        }
    }

    fn set_synced(&self, synced: bool) {
        unsafe {
            ffi::gst_clock_set_synced(self.to_glib_none().0, synced.to_glib());
        }
    }

    fn set_timeout(&self, timeout: ClockTime) {
        unsafe {
            ffi::gst_clock_set_timeout(self.to_glib_none().0, timeout);
        }
    }

    //fn single_shot_id_reinit(&self, id: /*Unimplemented*/ClockID, time: ClockTime) -> bool {
    //    unsafe { TODO: call ffi::gst_clock_single_shot_id_reinit() }
    //}

    fn unadjust_unlocked(&self, external: ClockTime) -> ClockTime {
        unsafe {
            ffi::gst_clock_unadjust_unlocked(self.to_glib_none().0, external)
        }
    }

    fn unadjust_with_calibration(&self, external_target: ClockTime, cinternal: ClockTime, cexternal: ClockTime, cnum: ClockTime, cdenom: ClockTime) -> ClockTime {
        unsafe {
            ffi::gst_clock_unadjust_with_calibration(self.to_glib_none().0, external_target, cinternal, cexternal, cnum, cdenom)
        }
    }

    fn wait_for_sync(&self, timeout: ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_clock_wait_for_sync(self.to_glib_none().0, timeout), "Timed out waiting for sync")
        }
    }

    fn get_property_window_size(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_window_size(&self, window_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "window-size".to_glib_none().0, Value::from(&window_size).to_glib_none().0);
        }
    }

    fn get_property_window_threshold(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-threshold".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_window_threshold(&self, window_threshold: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "window-threshold".to_glib_none().0, Value::from(&window_threshold).to_glib_none().0);
        }
    }

    fn connect_synced<F: Fn(&Self, bool) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "synced",
                transmute(synced_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::timeout",
                transmute(notify_timeout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-size",
                transmute(notify_window_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-threshold",
                transmute(notify_window_threshold_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn synced_trampoline<P>(this: *mut ffi::GstClock, synced: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<Clock> {
    callback_guard!();
    let f: &&(Fn(&P, bool) + Send + Sync + 'static) = transmute(f);
    f(&Clock::from_glib_borrow(this).downcast_unchecked(), from_glib(synced))
}

unsafe extern "C" fn notify_timeout_trampoline<P>(this: *mut ffi::GstClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Clock> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Clock::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_size_trampoline<P>(this: *mut ffi::GstClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Clock> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Clock::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_threshold_trampoline<P>(this: *mut ffi::GstClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Clock> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Clock::from_glib_borrow(this).downcast_unchecked())
}
