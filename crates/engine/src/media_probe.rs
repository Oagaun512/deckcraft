//! Media probing for the commands that insert and describe audio and video.
//!
//! With the `media` feature (the default) this is `deckcraft-media`. Without it, the engine
//! builds with no audio or video decoders at all (no codecs, no symphonia): probing reports that
//! media support isn't built in, inserted media is embedded unplayed (it still round-trips
//! through PPTX), and there are no poster frames. Hosts that only view and edit decks, such as a
//! browser build, use this to leave the decoders out.

#[cfg(feature = "media")]
pub use deckcraft_media::{Bytes, MediaInfo, poster_png, probe};

#[cfg(not(feature = "media"))]
pub use off::*;

#[cfg(not(feature = "media"))]
mod off {
    use std::sync::Arc;

    /// Shared media bytes (the model's `MediaItem::data`).
    pub type Bytes = Arc<Vec<u8>>;

    #[derive(Debug, Clone, PartialEq)]
    pub struct NotBuilt;

    impl std::fmt::Display for NotBuilt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("media support isn't built in")
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AudioInfo {
        pub codec: String,
        pub sample_rate: u32,
        pub channels: u16,
        pub decodable: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct VideoInfo {
        pub codec: String,
        pub width: u32,
        pub height: u32,
        pub fps: f64,
        pub frames: u64,
        pub decodable: bool,
    }

    /// The same shape as `deckcraft_media::MediaInfo`; never produced without the feature.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct MediaInfo {
        pub container: &'static str,
        pub duration_ms: u64,
        pub audio: Option<AudioInfo>,
        pub video: Option<VideoInfo>,
    }

    impl MediaInfo {
        pub fn duration(&self) -> f64 {
            self.duration_ms as f64 / 1000.0
        }
        pub fn playable(&self) -> bool {
            self.audio.as_ref().is_some_and(|a| a.decodable) || self.video.as_ref().is_some_and(|v| v.decodable)
        }
    }

    pub fn probe(_bytes: &Bytes) -> Result<MediaInfo, NotBuilt> {
        Err(NotBuilt)
    }

    pub fn poster_png(_bytes: &Bytes, _t: f64) -> Result<Vec<u8>, NotBuilt> {
        Err(NotBuilt)
    }
}
