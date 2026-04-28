use crate::api::event::BarEvent;
use crate::api::item::{
    BarItem, ComponentPosition, ImageType, ItemBuilder, PopupAlign, TextAlignment, ToggleState,
};
use crate::api::types::{Argb, Font, FontStyle};
use crate::api::{self};
use crate::children;
use crate::events::Event;
use crate::items::SketchybarItem;
use crate::path::data_dir;
use crate::themes::CATPUCCIN_MOCHA;
use anyhow::Result;
use async_trait::async_trait;
use image::DynamicImage;
use lazy_static::lazy_static;
use media_remote::{Controller, NowPlayingJXA};
use std::env;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const MINIMIZED_WIDTH: u32 = 25;
const PAUSE_TIMEOUT: Duration = Duration::from_mins(5);

lazy_static! {
    static ref PAUSE_INSTANT: Mutex<Option<Instant>> = Mutex::new(None);
}

#[derive(Debug, Clone, Default)]
pub struct MediaData {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub elapsed_time: f64,
    pub is_playing: bool,
    pub artwork: DynamicImage,
}

impl MediaData {
    pub fn formatted_progress(&self) -> String {
        format!(
            "{} / {}",
            format_time(self.elapsed_time),
            format_time(self.duration)
        )
    }
}

pub struct Media;

impl Media {
    pub fn fetch() -> Result<MediaData> {
        let now_playing = NowPlayingJXA::new(Duration::from_secs(1));

        let guard = now_playing.get_info();
        let info = guard.as_ref();

        if let Some(info) = info {
            return Ok(MediaData {
                title: info.title.clone().unwrap_or_default(),
                artist: info.artist.clone().unwrap_or_default(),
                album: info.album.clone().unwrap_or_default(),
                duration: info.duration.unwrap_or_default(),
                elapsed_time: info.elapsed_time.unwrap_or_default(),
                is_playing: info.is_playing.unwrap_or_default(),
                artwork: info.album_cover.clone().unwrap_or_default(),
            });
        }

        Ok(MediaData::default())
    }

    pub fn update_command() -> Result<()> {
        Self::update()
    }

    pub fn update() -> Result<()> {
        let name = env::var("NAME").unwrap_or_else(|_| "".to_string());
        let sender = env::var("SENDER").unwrap_or_else(|_| "".to_string());

        if sender == "mouse.clicked" {
            let now_playing = NowPlayingJXA::new(Duration::from_secs(1));

            match name.as_str() {
                "media.prev" => {
                    now_playing.previous();
                }
                "media.next" => {
                    now_playing.next();
                }
                "media.play" => {
                    now_playing.toggle();
                }
                _ => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        let data = Self::fetch()?;

        let mut pause_guard = PAUSE_INSTANT.lock().unwrap();
        let should_show = if data.is_playing {
            *pause_guard = None;
            true
        } else if !data.title.is_empty() {
            let pause_at = pause_guard.get_or_insert_with(Instant::now);
            pause_at.elapsed() < PAUSE_TIMEOUT
        } else {
            *pause_guard = None;
            false
        };

        // Main item
        if !should_show {
            BarItem::new_with_pos("media", ComponentPosition::Left)
                .width(MINIMIZED_WIDTH)
                .label_props(|p| p.drawing(ToggleState::Off))
                .animate_set("sin", 15)?;

            return Ok(());
        }

        let media_item = BarItem::new_with_pos("media", ComponentPosition::Left)
            .drawing(ToggleState::On)
            .width(240)
            .label(&data.title)
            .label_props(|p| p.max_chars(28).drawing(ToggleState::On))
            .scroll_texts(ToggleState::On)
            .icon_props(|p| p.drawing(ToggleState::On));

        media_item.animate_set("sin", 15)?;

        let artwork_path = data_dir().join("artwork.jpeg");
        let _ = std::fs::remove_file(&artwork_path);

        let _ = data
            .artwork
            .save_with_format(&artwork_path, image::ImageFormat::Jpeg);

        // Popups
        BarItem::new("media.cover")
            .drawing(ToggleState::On)
            .background(|b| {
                b.image(
                    ImageType::Path(artwork_path.display().to_string())
                        .into_props()
                        .drawing(ToggleState::On),
                )
            })
            .set()?;

        BarItem::new("media.title")
            .label(&data.title)
            .label_props(|p| p.drawing(ToggleState::On))
            .set()?;
        BarItem::new("media.artist")
            .label(&data.artist)
            .label_props(|p| p.drawing(ToggleState::On))
            .set()?;
        BarItem::new("media.progress")
            .label(&data.formatted_progress())
            .label_props(|p| p.drawing(ToggleState::On))
            .set()?;

        let play_icon = if data.is_playing { "󰏤" } else { "󰐎" };
        BarItem::new("media.play")
            .icon(play_icon)
            .icon_props(|p| p.drawing(ToggleState::On))
            .set()?;

        Ok(())
    }
}

#[async_trait]
impl SketchybarItem for Media {
    async fn setup(&self, exe_path: &str) -> Result<()> {
        let item = BarItem::new_with_pos("media", ComponentPosition::Left)
            .update_freq(0)
            .script(&format!("{} --update-media", exe_path))
            .click_script("sketchybar --animate sin 15 --set media popup.drawing=toggle")
            .icon("󰎆")
            .icon_props(|p| p.color(CATPUCCIN_MOCHA.green))
            .width(MINIMIZED_WIDTH)
            .background(|b| {
                b.color(CATPUCCIN_MOCHA.surface0)
                    .drawing(ToggleState::On)
                    .image(
                        ImageType::MediaArtwork
                            .into_props()
                            .drawing(ToggleState::On)
                            .scale(0.15)
                            .corner_radius(4),
                    )
            })
            .padding_left(10)
            .padding_right(10)
            .popup(|p| {
                p.align(PopupAlign::Center)
                    .background(|b| b.color(CATPUCCIN_MOCHA.base))
                    .background(|b| {
                        b.image(
                            ImageType::MediaArtwork
                                .into_props()
                                .blur_radius(50)
                                .drawing(ToggleState::On),
                        )
                        .corner_radius(12)
                        .border_width(2)
                        .border_color(CATPUCCIN_MOCHA.surface1)
                    })
            })
            .with_children(children![
                BarItem::new("media.cover")
                    .background(|b| {
                        b.image(
                            ImageType::MediaArtwork
                                .into_props()
                                .drawing(ToggleState::On)
                                .scale(0.5)
                                .corner_radius(12),
                        )
                        .height(140)
                    })
                    .padding_left(10)
                    .padding_right(10)
                    .width(240)
                    .text(|t| t.align(TextAlignment::Center)),
                BarItem::new("media.title")
                    .label_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 14.0,
                            style: FontStyle::Bold,
                        })
                        .max_chars(25)
                    })
                    .scroll_texts(ToggleState::On)
                    .width(240)
                    .text(|t| t.align(TextAlignment::Center)),
                BarItem::new("media.artist")
                    .label_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 13.0,
                            style: FontStyle::Regular,
                        })
                        .color(Argb::from_u32(0xffbac2de))
                    })
                    .width(240)
                    .text(|t| t.align(TextAlignment::Center)),
                BarItem::new("media.progress")
                    .label_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 12.0,
                            style: FontStyle::Regular,
                        })
                    })
                    .width(240)
                    .text(|t| t.align(TextAlignment::Center)),
                BarItem::new("media.prev")
                    .icon("󰒮")
                    .icon_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 20.0,
                            style: FontStyle::Regular,
                        })
                    })
                    .width(80)
                    .text(|t| t.align(TextAlignment::Center))
                    .click_script(&format!("{} --update-media", exe_path)),
                BarItem::new("media.play")
                    .icon("󰐎")
                    .icon_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 24.0,
                            style: FontStyle::Regular,
                        })
                    })
                    .width(80)
                    .text(|t| t.align(TextAlignment::Center))
                    .click_script(&format!("{} --update-media", exe_path)),
                BarItem::new("media.next")
                    .icon("󰒭")
                    .icon_props(|p| {
                        p.font(Font {
                            family: "JetBrainsMono Nerd Font".to_string(),
                            size: 20.0,
                            style: FontStyle::Regular,
                        })
                    })
                    .width(80)
                    .text(|t| t.align(TextAlignment::Center))
                    .click_script(&format!("{} --update-media", exe_path)),
            ]);

        api::add_event("media_update")?;
        item.add()?;
        item.subscribe([
            BarEvent::from("media_change"),
            BarEvent::from("media_update"),
            BarEvent::MouseClicked,
        ])?;

        Ok(())
    }

    async fn spawn_background_task(&self, mut bus: tokio::sync::broadcast::Receiver<Event>) {
        tokio::spawn(async move {
            while let Ok(event) = bus.recv().await {
                if matches!(event, Event::UpdateMedia)
                    && let Err(e) = Self::update_command()
                {
                    eprintln!("[media] update error: {e}");
                }
            }
        });
    }
}

fn format_time(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
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
