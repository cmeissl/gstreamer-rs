// This file was generated by gir (531f8d9) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub struct SeekFlags: u32 {
        const SEEK_FLAG_NONE = 0;
        const SEEK_FLAG_FLUSH = 1;
        const SEEK_FLAG_ACCURATE = 2;
        const SEEK_FLAG_KEY_UNIT = 4;
        const SEEK_FLAG_SEGMENT = 8;
        const SEEK_FLAG_TRICKMODE = 16;
        const SEEK_FLAG_SKIP = 16;
        const SEEK_FLAG_SNAP_BEFORE = 32;
        const SEEK_FLAG_SNAP_AFTER = 64;
        const SEEK_FLAG_SNAP_NEAREST = 96;
        const SEEK_FLAG_TRICKMODE_KEY_UNITS = 128;
        const SEEK_FLAG_TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SeekFlags {
    type GlibType = ffi::GstSeekFlags;

    fn to_glib(&self) -> ffi::GstSeekFlags {
        ffi::GstSeekFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSeekFlags> for SeekFlags {
    fn from_glib(value: ffi::GstSeekFlags) -> SeekFlags {
        SeekFlags::from_bits_truncate(value.bits())
    }
}

