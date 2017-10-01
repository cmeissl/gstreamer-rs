// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

mod stream_volume;
pub use self::stream_volume::StreamVolume;
pub use self::stream_volume::StreamVolumeExt;

mod enums;
pub use self::enums::AudioChannelPosition;
pub use self::enums::AudioFormat;
pub use self::enums::AudioLayout;
pub use self::enums::StreamVolumeFormat;

mod flags;
pub use self::flags::AudioFlags;
pub use self::flags::AudioFormatFlags;
pub use self::flags::AudioPackFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::StreamVolumeExt;
}
