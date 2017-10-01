// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

use Error;
use PlayerAudioInfo;
use PlayerColorBalanceType;
use PlayerMediaInfo;
use PlayerSnapshotFormat;
use PlayerState;
use PlayerSubtitleInfo;
use PlayerVideoInfo;
use PlayerVisualization;
use ffi;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Player(Object<ffi::GstPlayer>);

    match fn {
        get_type => || ffi::gst_player_get_type(),
    }
}

impl Player {
    pub fn config_set_seek_accurate(&self, accurate: bool) {
        unsafe {
            ffi::gst_player_config_set_seek_accurate(self.to_glib_none().0, accurate.to_glib());
        }
    }

    pub fn get_audio_video_offset(&self) -> i64 {
        unsafe {
            ffi::gst_player_get_audio_video_offset(self.to_glib_none().0)
        }
    }

    pub fn get_color_balance(&self, type_: PlayerColorBalanceType) -> f64 {
        unsafe {
            ffi::gst_player_get_color_balance(self.to_glib_none().0, type_.to_glib())
        }
    }

    pub fn get_config(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_full(ffi::gst_player_get_config(self.to_glib_none().0))
        }
    }

    pub fn get_current_audio_track(&self) -> Option<PlayerAudioInfo> {
        unsafe {
            from_glib_full(ffi::gst_player_get_current_audio_track(self.to_glib_none().0))
        }
    }

    pub fn get_current_subtitle_track(&self) -> Option<PlayerSubtitleInfo> {
        unsafe {
            from_glib_none(ffi::gst_player_get_current_subtitle_track(self.to_glib_none().0))
        }
    }

    pub fn get_current_video_track(&self) -> Option<PlayerVideoInfo> {
        unsafe {
            from_glib_full(ffi::gst_player_get_current_video_track(self.to_glib_none().0))
        }
    }

    pub fn get_current_visualization(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_player_get_current_visualization(self.to_glib_none().0))
        }
    }

    pub fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            ffi::gst_player_get_duration(self.to_glib_none().0)
        }
    }

    pub fn get_media_info(&self) -> Option<PlayerMediaInfo> {
        unsafe {
            from_glib_full(ffi::gst_player_get_media_info(self.to_glib_none().0))
        }
    }

    //pub fn get_multiview_flags(&self) -> /*Ignored*/gst_video::VideoMultiviewFlags {
    //    unsafe { TODO: call ffi::gst_player_get_multiview_flags() }
    //}

    //pub fn get_multiview_mode(&self) -> /*Ignored*/gst_video::VideoMultiviewMode {
    //    unsafe { TODO: call ffi::gst_player_get_multiview_mode() }
    //}

    pub fn get_mute(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_player_get_mute(self.to_glib_none().0))
        }
    }

    pub fn get_pipeline(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_player_get_pipeline(self.to_glib_none().0))
        }
    }

    pub fn get_position(&self) -> gst::ClockTime {
        unsafe {
            ffi::gst_player_get_position(self.to_glib_none().0)
        }
    }

    pub fn get_rate(&self) -> f64 {
        unsafe {
            ffi::gst_player_get_rate(self.to_glib_none().0)
        }
    }

    pub fn get_subtitle_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_player_get_subtitle_uri(self.to_glib_none().0))
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_player_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_video_snapshot<'a, P: Into<Option<&'a gst::Structure>>>(&self, format: PlayerSnapshotFormat, config: P) -> Option<gst::Sample> {
        let config = config.into();
        let config = config.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_player_get_video_snapshot(self.to_glib_none().0, format.to_glib(), config.0))
        }
    }

    pub fn get_volume(&self) -> f64 {
        unsafe {
            ffi::gst_player_get_volume(self.to_glib_none().0)
        }
    }

    pub fn has_color_balance(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_player_has_color_balance(self.to_glib_none().0))
        }
    }

    pub fn pause(&self) {
        unsafe {
            ffi::gst_player_pause(self.to_glib_none().0);
        }
    }

    pub fn play(&self) {
        unsafe {
            ffi::gst_player_play(self.to_glib_none().0);
        }
    }

    pub fn seek(&self, position: gst::ClockTime) {
        unsafe {
            ffi::gst_player_seek(self.to_glib_none().0, position);
        }
    }

    pub fn set_audio_track(&self, stream_index: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_player_set_audio_track(self.to_glib_none().0, stream_index))
        }
    }

    pub fn set_audio_track_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_player_set_audio_track_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_audio_video_offset(&self, offset: i64) {
        unsafe {
            ffi::gst_player_set_audio_video_offset(self.to_glib_none().0, offset);
        }
    }

    pub fn set_color_balance(&self, type_: PlayerColorBalanceType, value: f64) {
        unsafe {
            ffi::gst_player_set_color_balance(self.to_glib_none().0, type_.to_glib(), value);
        }
    }

    //pub fn set_multiview_flags(&self, flags: /*Ignored*/gst_video::VideoMultiviewFlags) {
    //    unsafe { TODO: call ffi::gst_player_set_multiview_flags() }
    //}

    //pub fn set_multiview_mode(&self, mode: /*Ignored*/gst_video::VideoMultiviewMode) {
    //    unsafe { TODO: call ffi::gst_player_set_multiview_mode() }
    //}

    pub fn set_mute(&self, val: bool) {
        unsafe {
            ffi::gst_player_set_mute(self.to_glib_none().0, val.to_glib());
        }
    }

    pub fn set_rate(&self, rate: f64) {
        unsafe {
            ffi::gst_player_set_rate(self.to_glib_none().0, rate);
        }
    }

    pub fn set_subtitle_track(&self, stream_index: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_player_set_subtitle_track(self.to_glib_none().0, stream_index))
        }
    }

    pub fn set_subtitle_track_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_player_set_subtitle_track_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_subtitle_uri(&self, uri: &str) {
        unsafe {
            ffi::gst_player_set_subtitle_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    pub fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gst_player_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    pub fn set_video_track(&self, stream_index: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_player_set_video_track(self.to_glib_none().0, stream_index))
        }
    }

    pub fn set_video_track_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_player_set_video_track_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_visualization(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_player_set_visualization(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn set_visualization_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_player_set_visualization_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_volume(&self, val: f64) {
        unsafe {
            ffi::gst_player_set_volume(self.to_glib_none().0, val);
        }
    }

    pub fn stop(&self) {
        unsafe {
            ffi::gst_player_stop(self.to_glib_none().0);
        }
    }

    pub fn get_property_suburi(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "suburi".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_suburi(&self, suburi: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "suburi".to_glib_none().0, Value::from(suburi).to_glib_none().0);
        }
    }

    //pub fn get_property_video_multiview_flags(&self) -> /*Ignored*/gst_video::VideoMultiviewFlags {
    //    let mut value = Value::from(&0u32);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "video-multiview-flags".to_glib_none().0, value.to_glib_none_mut().0);
    //        from_glib(transmute(value.get::<u32>().unwrap()))
    //    }
    //}

    //pub fn set_property_video_multiview_flags(&self, video_multiview_flags: /*Ignored*/gst_video::VideoMultiviewFlags) {
    //    let video_multiview_flags = video_multiview_flags.to_glib().bits() as u32;
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "video-multiview-flags".to_glib_none().0, Value::from(&video_multiview_flags).to_glib_none().0);
    //    }
    //}

    //pub fn get_property_video_multiview_mode(&self) -> /*Ignored*/gst_video::VideoMultiviewFramePacking {
    //    let mut value = Value::from(&0);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "video-multiview-mode".to_glib_none().0, value.to_glib_none_mut().0);
    //        from_glib(transmute(value.get::<i32>().unwrap()))
    //    }
    //}

    //pub fn set_property_video_multiview_mode(&self, video_multiview_mode: /*Ignored*/gst_video::VideoMultiviewFramePacking) {
    //    let video_multiview_mode = video_multiview_mode.to_glib() as i32;
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "video-multiview-mode".to_glib_none().0, Value::from(&video_multiview_mode).to_glib_none().0);
    //    }
    //}

    pub fn config_get_position_update_interval(config: &gst::Structure) -> u32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_player_config_get_position_update_interval(config.to_glib_none().0)
        }
    }

    pub fn config_get_seek_accurate(config: &gst::Structure) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_player_config_get_seek_accurate(config.to_glib_none().0))
        }
    }

    pub fn config_get_user_agent(config: &gst::Structure) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_player_config_get_user_agent(config.to_glib_none().0))
        }
    }

    pub fn config_set_position_update_interval(config: &mut gst::Structure, interval: u32) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_player_config_set_position_update_interval(config.to_glib_none_mut().0, interval);
        }
    }

    pub fn config_set_user_agent(config: &mut gst::Structure, agent: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_player_config_set_user_agent(config.to_glib_none_mut().0, agent.to_glib_none().0);
        }
    }

    pub fn get_audio_streams(info: &PlayerMediaInfo) -> Vec<PlayerAudioInfo> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_get_audio_streams(info.to_glib_none().0))
        }
    }

    pub fn get_subtitle_streams(info: &PlayerMediaInfo) -> Vec<PlayerSubtitleInfo> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_get_subtitle_streams(info.to_glib_none().0))
        }
    }

    pub fn get_video_streams(info: &PlayerMediaInfo) -> Vec<PlayerVideoInfo> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_player_get_video_streams(info.to_glib_none().0))
        }
    }

    pub fn visualizations_get() -> Vec<PlayerVisualization> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_player_visualizations_get())
        }
    }

    pub fn connect_buffering<F: Fn(&Player, i32) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, i32) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "buffering",
                transmute(buffering_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_duration_changed<F: Fn(&Player, u64) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, u64) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "duration-changed",
                transmute(duration_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_end_of_stream<F: Fn(&Player) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "end-of-stream",
                transmute(end_of_stream_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_error<F: Fn(&Player, &Error) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, &Error) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "error",
                transmute(error_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_media_info_updated<F: Fn(&Player, &PlayerMediaInfo) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, &PlayerMediaInfo) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "media-info-updated",
                transmute(media_info_updated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_mute_changed<F: Fn(&Player) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mute-changed",
                transmute(mute_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_position_updated<F: Fn(&Player, u64) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, u64) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "position-updated",
                transmute(position_updated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_seek_done<F: Fn(&Player, u64) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, u64) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "seek-done",
                transmute(seek_done_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_state_changed<F: Fn(&Player, PlayerState) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, PlayerState) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "state-changed",
                transmute(state_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_uri_loaded<F: Fn(&Player, &str) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, &str) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "uri-loaded",
                transmute(uri_loaded_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_video_dimensions_changed<F: Fn(&Player, i32, i32) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, i32, i32) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "video-dimensions-changed",
                transmute(video_dimensions_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_volume_changed<F: Fn(&Player) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "volume-changed",
                transmute(volume_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_warning<F: Fn(&Player, &Error) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, &Error) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "warning",
                transmute(warning_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_audio_video_offset_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::audio-video-offset",
                transmute(notify_audio_video_offset_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_current_audio_track_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-audio-track",
                transmute(notify_current_audio_track_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_current_subtitle_track_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-subtitle-track",
                transmute(notify_current_subtitle_track_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_current_video_track_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-video-track",
                transmute(notify_current_video_track_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_duration_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::duration",
                transmute(notify_duration_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_media_info_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::media-info",
                transmute(notify_media_info_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_mute_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mute",
                transmute(notify_mute_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_pipeline_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pipeline",
                transmute(notify_pipeline_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_position_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::position",
                transmute(notify_position_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_rate_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rate",
                transmute(notify_rate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_signal_dispatcher_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::signal-dispatcher",
                transmute(notify_signal_dispatcher_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_suburi_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::suburi",
                transmute(notify_suburi_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_uri_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_video_multiview_flags_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::video-multiview-flags",
                transmute(notify_video_multiview_flags_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_video_multiview_mode_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::video-multiview-mode",
                transmute(notify_video_multiview_mode_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_video_renderer_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::video-renderer",
                transmute(notify_video_renderer_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_volume_notify<F: Fn(&Player) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::volume",
                transmute(notify_volume_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}

unsafe extern "C" fn buffering_trampoline(this: *mut ffi::GstPlayer, object: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, i32) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), object)
}

unsafe extern "C" fn duration_changed_trampoline(this: *mut ffi::GstPlayer, object: u64, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, u64) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), object)
}

unsafe extern "C" fn end_of_stream_trampoline(this: *mut ffi::GstPlayer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn error_trampoline(this: *mut ffi::GstPlayer, object: *mut glib_ffi::GError, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, &Error) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(object))
}

unsafe extern "C" fn media_info_updated_trampoline(this: *mut ffi::GstPlayer, object: *mut ffi::GstPlayerMediaInfo, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, &PlayerMediaInfo) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(object))
}

unsafe extern "C" fn mute_changed_trampoline(this: *mut ffi::GstPlayer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn position_updated_trampoline(this: *mut ffi::GstPlayer, object: u64, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, u64) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), object)
}

unsafe extern "C" fn seek_done_trampoline(this: *mut ffi::GstPlayer, object: u64, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, u64) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), object)
}

unsafe extern "C" fn state_changed_trampoline(this: *mut ffi::GstPlayer, object: ffi::GstPlayerState, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, PlayerState) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), from_glib(object))
}

unsafe extern "C" fn uri_loaded_trampoline(this: *mut ffi::GstPlayer, object: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, &str) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), &String::from_glib_none(object))
}

unsafe extern "C" fn video_dimensions_changed_trampoline(this: *mut ffi::GstPlayer, object: libc::c_int, p0: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, i32, i32) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), object, p0)
}

unsafe extern "C" fn volume_changed_trampoline(this: *mut ffi::GstPlayer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn warning_trampoline(this: *mut ffi::GstPlayer, object: *mut glib_ffi::GError, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player, &Error) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(object))
}

unsafe extern "C" fn notify_audio_video_offset_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_current_audio_track_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_current_subtitle_track_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_current_video_track_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_duration_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_media_info_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_mute_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_pipeline_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_position_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_rate_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_signal_dispatcher_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_suburi_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_uri_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_video_multiview_flags_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_video_multiview_mode_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_video_renderer_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_volume_trampoline(this: *mut ffi::GstPlayer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Player) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
