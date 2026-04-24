use crate::api::event::BarEvent;
use crate::api::item::{
    BarItem, ComponentPosition, ImageType, ItemBuilder, PopupAlign, TextAlignment, ToggleState,
};
use crate::api::types::{Argb, Font, FontStyle};
use crate::api::{self};
use crate::media_ffi::{self, MediaRemoteCommand};
use crate::path::data_dir;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use media_remote::{NowPlaying, NowPlayingPerl};
use std::env;

#[derive(Debug, Clone, Default)]
pub struct Media {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub elapsed_time: f64,
    pub playback_rate: f64,
    pub timestamp: Option<f64>,
    pub has_artwork: bool,
    pub artwork_path: String,
}

impl Media {
    pub fn fetch() -> Result<Self> {
        // 1. Try custom FFI
        if let Some(info) = media_ffi::get_now_playing_info()
            && (info.title.is_some() || info.artist.is_some())
        {
            return Self::from_ffi_info(&info);
        }

        // 2. Try media-remote crate (Native)
        let now_playing = NowPlaying::new();
        if let Some(info) = &*now_playing.get_info()
            && (info.title.is_some() || info.artist.is_some())
        {
            return Self::from_crate_info(info);
        }

        // 3. Try media-remote crate (Perl fallback)
        let perl = NowPlayingPerl::new();
        if let Some(info) = &*perl.get_info()
            && (info.title.is_some() || info.artist.is_some())
        {
            return Self::from_crate_info(info);
        }

        Ok(Self::default())
    }

    fn from_ffi_info(info: &media_ffi::NowPlayingInfo) -> Result<Self> {
        let artwork_path = data_dir().join("media.png");
        let mut has_artwork = false;

        if let Some(data) = &info.artwork_data {
            if std::fs::write(&artwork_path, data).is_ok() {
                has_artwork = true;
            }
        } else if artwork_path.exists() {
            has_artwork = true;
        }

        Ok(Self {
            title: info.title.clone().unwrap_or_else(|| "Unknown".into()),
            artist: info.artist.clone().unwrap_or_else(|| "Unknown".into()),
            album: info.album.clone().unwrap_or_default(),
            duration: info.duration.unwrap_or(0.0),
            elapsed_time: info.elapsed_time.unwrap_or(0.0),
            playback_rate: info.playback_rate.unwrap_or(0.0),
            timestamp: info.timestamp,
            has_artwork,
            artwork_path: artwork_path.to_string_lossy().into(),
        })
    }

    fn from_crate_info(info: &media_remote::NowPlayingInfo) -> Result<Self> {
        let artwork_path = data_dir().join("media.png");
        let mut has_artwork = false;

        if let Some(image) = &info.album_cover
            && image.save(&artwork_path).is_ok()
        {
            has_artwork = true;
        }

        let timestamp = info.info_update_time.and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs_f64() - 978_307_200.0)
        });

        Ok(Self {
            title: info.title.clone().unwrap_or_else(|| "Unknown".into()),
            artist: info.artist.clone().unwrap_or_else(|| "Unknown".into()),
            album: info.album.clone().unwrap_or_default(),
            duration: info.duration.unwrap_or(0.0),
            elapsed_time: info.elapsed_time.unwrap_or(0.0),
            playback_rate: info
                .is_playing
                .map(|p| if p { 1.0 } else { 0.0 })
                .unwrap_or(0.0),
            timestamp,
            has_artwork,
            artwork_path: artwork_path.to_string_lossy().into(),
        })
    }

    pub fn formatted_progress(&self) -> String {
        let mut elapsed = self.elapsed_time;
        let is_playing = self.playback_rate > 0.0;

        if is_playing && let Some(timestamp) = self.timestamp {
            let now_unix = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64();
            let apple_reference_date_offset = 978_307_200.0;
            let timestamp_unix = timestamp + apple_reference_date_offset;
            let diff = now_unix - timestamp_unix;
            if diff > 0.0 && diff < 3600.0 {
                elapsed += diff * self.playback_rate;
            }
        }

        format!("{} / {}", format_time(elapsed), format_time(self.duration))
    }
}

pub fn update_command() -> Result<()> {
    update()
}

pub fn update() -> Result<()> {
    let name = env::var("NAME").unwrap_or_else(|_| "".to_string());
    let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());

    if sender == "mouse.clicked" {
        match name.as_str() {
            "media.prev" => {
                media_ffi::send_command(MediaRemoteCommand::PreviousTrack);
            }
            "media.next" => {
                media_ffi::send_command(MediaRemoteCommand::NextTrack);
            }
            "media.play" => {
                media_ffi::send_command(MediaRemoteCommand::TogglePlayPause);
            }
            _ => {}
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let data = Media::fetch()?;

    // Main item
    if data.title.is_empty() && data.artist.is_empty() {
        BarItem::new("media").drawing(ToggleState::Off).set()?;
        return Ok(());
    }

    let track_info = if data.artist.is_empty() {
        data.title.clone()
    } else if data.title.is_empty() {
        data.artist.clone()
    } else {
        format!("{} - {}", data.artist, data.title)
    };

    let mut display_text = track_info;
    if display_text.len() > 25 {
        display_text.truncate(22);
        display_text.push_str("...");
    }

    let mut media_item = BarItem::new("media")
        .drawing(ToggleState::On)
        .label(&display_text)
        .label_drawing(ToggleState::On)
        .icon_drawing(ToggleState::On);

    if data.has_artwork {
        media_item = media_item
            .background_image(ImageType::Path(data.artwork_path.clone()))
            .background_image_drawing(ToggleState::On)
            .background_image_blur_radius(20)
            .background_image_scale(0.15);
    } else {
        media_item = media_item.background_image_drawing(ToggleState::Off);
    }
    media_item.set()?;

    // Popups
    if data.has_artwork {
        BarItem::new("media")
            .popup_background_image(ImageType::Path(data.artwork_path.clone()))
            .popup_background_image_blur_radius(50)
            .popup_background_image_drawing(ToggleState::On)
            .set()?;

        BarItem::new("media.cover")
            .background_image(ImageType::Path(data.artwork_path.clone()))
            .background_image_drawing(ToggleState::On)
            .drawing(ToggleState::On)
            .set()?;
    } else {
        BarItem::new("media.cover")
            .drawing(ToggleState::Off)
            .set()?;
        BarItem::new("media")
            .popup_background_image_drawing(ToggleState::Off)
            .set()?;
    }

    BarItem::new("media.title").label(&data.title).set()?;
    BarItem::new("media.artist").label(&data.artist).set()?;
    BarItem::new("media.progress")
        .label(&data.formatted_progress())
        .set()?;

    let play_icon = if data.playback_rate > 0.0 {
        "󰏤"
    } else {
        "󰐎"
    };
    BarItem::new("media.play").icon(play_icon).set()?;

    Ok(())
}

fn format_time(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn setup(exe_path: &str) -> Result<()> {
    let item = BarItem::new("media")
        .position(ComponentPosition::Left)
        .update_freq(0)
        .script(&format!("{} --update-media", exe_path))
        .click_script("sketchybar --set media popup.drawing=toggle")
        .icon("󰎆")
        .icon_color(CATPUCCIN_MOCHA.green)
        .background_color(CATPUCCIN_MOCHA.surface0)
        .background_drawing(ToggleState::On)
        .background_image(ImageType::Path(
            data_dir().join("media.png").to_str().unwrap().to_string(),
        ))
        .background_image_drawing(ToggleState::On)
        .background_image_scale(0.15)
        .background_image_corner_radius(4)
        .padding_left(10)
        .padding_right(10)
        .popup_align(PopupAlign::Center)
        .popup_background_color(CATPUCCIN_MOCHA.base)
        .popup_background_corner_radius(12)
        .popup_background_border_width(2)
        .popup_background_border_color(CATPUCCIN_MOCHA.surface1)
        .add_item(
            BarItem::new("media.cover")
                .background_image(ImageType::MediaArtwork)
                .background_image_drawing(ToggleState::On)
                .background_image_scale(0.5)
                .background_image_corner_radius(12)
                .background_height(140)
                .padding_left(10)
                .padding_right(10)
                .width(240)
                .text_align(TextAlignment::Center),
        )
        .add_item(
            BarItem::new("media.title")
                .label_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 16.0,
                    style: FontStyle::Bold,
                })
                .width(240)
                .text_align(TextAlignment::Center),
        )
        .add_item(
            BarItem::new("media.artist")
                .label_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 13.0,
                    style: FontStyle::Regular,
                })
                .label_color(Argb::from_u32(0xffbac2de))
                .width(240)
                .text_align(TextAlignment::Center),
        )
        .add_item(
            BarItem::new("media.progress")
                .label_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 12.0,
                    style: FontStyle::Regular,
                })
                .width(240)
                .text_align(TextAlignment::Center),
        )
        .add_item(
            BarItem::new("media.prev")
                .icon("󰒮")
                .icon_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 20.0,
                    style: FontStyle::Regular,
                })
                .width(80)
                .text_align(TextAlignment::Center)
                .click_script(&format!("{} --update-media", exe_path)),
        )
        .add_item(
            BarItem::new("media.play")
                .icon("󰐎")
                .icon_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 24.0,
                    style: FontStyle::Regular,
                })
                .width(80)
                .text_align(TextAlignment::Center)
                .y_offset(-32)
                .click_script(&format!("{} --update-media", exe_path)),
        )
        .add_item(
            BarItem::new("media.next")
                .icon("󰒭")
                .icon_font(Font {
                    family: "JetBrainsMono Nerd Font".to_string(),
                    size: 20.0,
                    style: FontStyle::Regular,
                })
                .width(80)
                .text_align(TextAlignment::Center)
                .y_offset(-32)
                .click_script(&format!("{} --update-media", exe_path)),
        );

    api::add_event("media_update")?;
    item.add()?;
    item.subscribe([BarEvent::from("media_update"), BarEvent::MouseClicked])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(0.0), "00:00");
        assert_eq!(format_time(60.0), "01:00");
        assert_eq!(format_time(65.0), "01:05");
        assert_eq!(format_time(3600.0), "60:00");
    }
}
