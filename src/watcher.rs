use crate::api;
use crate::media_ffi;
use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopDefaultMode};
use core_foundation::string::CFString;
use std::os::raw::c_void;
use std::ptr;
use std::thread;
use std::time::Duration;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFNotificationCenterGetDistributedCenter() -> *mut c_void;
    fn CFNotificationCenterGetDarwinNotifyCenter() -> *mut c_void;
    fn CFNotificationCenterAddObserver(
        center: *mut c_void,
        observer: *const c_void,
        callback: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_void,
        ),
        name: *mut c_void,
        object: *const c_void,
        suspensionBehavior: i64,
    );
}

const CF_NOTIFICATION_SUSPENSION_BEHAVIOR_DELIVER_IMMEDIATELY: i64 = 4;

unsafe extern "C" fn notification_callback(
    _center: *mut c_void,
    _observer: *mut c_void,
    _name: *mut c_void,
    _object: *mut c_void,
    _user_info: *mut c_void,
) {
    // Trigger the media_update event in SketchyBar
    let _ = api::trigger_evt("media_update");
}

pub fn watch_media() -> anyhow::Result<()> {
    println!("Starting media watcher...");

    // Start the notification listener in a background thread
    thread::spawn(|| {
        unsafe {
            let distributed_center = CFNotificationCenterGetDistributedCenter();
            let darwin_center = CFNotificationCenterGetDarwinNotifyCenter();

            let observe = |center: *mut c_void, name: &str| {
                let cf_name = CFString::new(name);
                CFNotificationCenterAddObserver(
                    center,
                    ptr::null(),
                    notification_callback,
                    cf_name.as_CFTypeRef() as _,
                    ptr::null(),
                    CF_NOTIFICATION_SUSPENSION_BEHAVIOR_DELIVER_IMMEDIATELY,
                );
            };

            // 1. Generic MediaRemote notifications
            observe(
                darwin_center,
                "kMRMediaRemoteNowPlayingInfoDidChangeNotification",
            );
            observe(darwin_center, "kMRPlaybackStateDidChangeNotification");
            observe(
                darwin_center,
                "kMRNowPlayingPlaybackQueueChangedNotification",
            );

            // 2. App-specific Distributed notifications
            observe(distributed_center, "com.apple.Music.playerInfo");
            observe(
                distributed_center,
                "com.spotify.client.PlaybackStateChanged",
            );

            println!("Watcher active. Listening for media events...");
            CFRunLoop::run_in_mode(kCFRunLoopDefaultMode, Duration::from_secs(3600 * 24), false);
        }
    });

    // Heartbeat for progress bar (only triggers when playing)
    loop {
        let is_playing = media_ffi::get_now_playing_info()
            .map(|info| info.playback_rate.unwrap_or(0.0) > 0.0)
            .unwrap_or(false);

        if is_playing {
            let _ = api::trigger_evt("media_update");
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
