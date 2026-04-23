use block2::{Block, RcBlock};
use core_foundation::base::TCFType;
use core_foundation::data::CFData;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use std::os::raw::c_void;
use std::ptr;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

#[repr(u32)]
pub enum MediaRemoteCommand {
    Play = 0,
    Pause = 1,
    TogglePlayPause = 2,
    Stop = 3,
    NextTrack = 4,
    PreviousTrack = 5,
}

#[link(name = "MediaRemote", kind = "framework")]
unsafe extern "C" {
    fn MRMediaRemoteGetNowPlayingInfo(
        queue: *const c_void,
        callback: &Block<dyn Fn(*const c_void)>,
    );

    fn MRMediaRemoteSendCommand(command: u32, arg: *const c_void) -> bool;
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C" {
    fn dispatch_get_global_queue(identifier: i64, flags: u64) -> *const c_void;
}

const QOS_CLASS_DEFAULT: i64 = 0x15;

pub struct NowPlayingInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<f64>,
    pub elapsed_time: Option<f64>,
    pub playback_rate: Option<f64>,
    pub timestamp: Option<f64>,
    pub artwork_data: Option<Vec<u8>>,
}

pub fn get_now_playing_info() -> Option<NowPlayingInfo> {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    let block = RcBlock::new({
        let tx = tx.clone();
        move |info_ptr: *const c_void| {
            if info_ptr.is_null() {
                let _ = tx.lock().unwrap().send(None);
                return;
            }

            let dict: CFDictionary<CFString, *const c_void> =
                unsafe { TCFType::wrap_under_get_rule(info_ptr as _) };

            let get_string = |key: &str| {
                let cf_key = CFString::new(key);
                dict.find(&cf_key).map(|val| {
                    let cf_string: CFString = unsafe { TCFType::wrap_under_get_rule(*val as _) };
                    cf_string.to_string()
                })
            };

            let get_f64 = |key: &str| {
                let cf_key = CFString::new(key);
                dict.find(&cf_key).and_then(|val| {
                    let cf_num: CFNumber = unsafe { TCFType::wrap_under_get_rule(*val as _) };
                    cf_num.to_f64()
                })
            };

            let get_data = |key: &str| {
                let cf_key = CFString::new(key);
                dict.find(&cf_key).map(|val| {
                    let cf_data: CFData = unsafe { TCFType::wrap_under_get_rule(*val as _) };
                    cf_data.to_vec()
                })
            };

            let info = NowPlayingInfo {
                title: get_string("kMRMediaRemoteNowPlayingInfoTitle"),
                artist: get_string("kMRMediaRemoteNowPlayingInfoArtist"),
                album: get_string("kMRMediaRemoteNowPlayingInfoAlbum"),
                duration: get_f64("kMRMediaRemoteNowPlayingInfoDuration"),
                elapsed_time: get_f64("kMRMediaRemoteNowPlayingInfoElapsedTime"),
                playback_rate: get_f64("kMRMediaRemoteNowPlayingInfoPlaybackRate"),
                timestamp: get_f64("kMRMediaRemoteNowPlayingInfoTimestamp"),
                artwork_data: get_data("kMRMediaRemoteNowPlayingInfoArtworkData"),
            };

            let _ = tx.lock().unwrap().send(Some(info));
        }
    });

    unsafe {
        let queue = dispatch_get_global_queue(QOS_CLASS_DEFAULT, 0);
        MRMediaRemoteGetNowPlayingInfo(queue, &block);
    }

    rx.recv_timeout(Duration::from_millis(2000)).ok().flatten()
}

pub fn send_command(command: MediaRemoteCommand) -> bool {
    unsafe { MRMediaRemoteSendCommand(command as u32, ptr::null()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_playing_info_struct() {
        let info = NowPlayingInfo {
            title: Some("Test Title".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            duration: Some(300.0),
            elapsed_time: Some(150.0),
            playback_rate: Some(1.0),
            timestamp: Some(0.0),
            artwork_data: None,
        };

        assert_eq!(info.title.unwrap(), "Test Title");
        assert_eq!(info.artist.unwrap(), "Test Artist");
        assert_eq!(info.duration.unwrap(), 300.0);
    }

    #[test]
    fn test_media_remote_command_enum() {
        assert_eq!(MediaRemoteCommand::Play as u32, 0);
        assert_eq!(MediaRemoteCommand::Pause as u32, 1);
        assert_eq!(MediaRemoteCommand::TogglePlayPause as u32, 2);
    }

    #[test]
    #[ignore] // This test requires a real media session to be active
    fn test_get_now_playing_info_real() {
        let info = get_now_playing_info();
        println!("Real Info: {:?}", info.is_some());
        if let Some(i) = info {
            println!("Title: {:?}", i.title);
            println!("Artist: {:?}", i.artist);
        }
    }
}
