use crate::api;
use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopDefaultMode};
use core_foundation::string::CFString;
use media_remote::NowPlayingJXA;
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
    let _ = api::trigger_evt("media_update");
}

unsafe extern "C" fn keyboard_layout_callback(
    _center: *mut c_void,
    _observer: *mut c_void,
    _name: *mut c_void,
    _object: *mut c_void,
    _user_info: *mut c_void,
) {
    // Read source here in the watcher process (which has a TIS context) so we
    // don't rely on Carbon API working correctly inside a short-lived subprocess.
    let source_id = crate::keyboard_ffi::get_current_source_id().unwrap_or_default();
    let _ = api::trigger_evt_with_data("keyboard_layout_change", &source_id);
}

pub fn watch_media() -> anyhow::Result<()> {
    println!("Starting media watcher...");

    // Heartbeat for progress bar (only triggers when playing)
    // Run this in a background thread so the main thread can run the CFRunLoop.
    thread::spawn(|| {
        loop {
            let now_playing = NowPlayingJXA::new(Duration::from_secs(1));
            let guard = now_playing.get_info();
            let info = guard.as_ref();

            if info.is_some_and(|info| info.is_playing.unwrap_or_default()) {
                let _ = api::trigger_evt("media_update");
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });

    // Run the notification listener on the main thread
    // TIS APIs are thread-local and must run on the main thread to get global changes.
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

        // 3. Keyboard layout change
        let cf_kb_name =
            CFString::new("com.apple.Carbon.TISNotifySelectedKeyboardInputSourceChanged");
        CFNotificationCenterAddObserver(
            distributed_center,
            ptr::null(),
            keyboard_layout_callback,
            cf_kb_name.as_CFTypeRef() as _,
            ptr::null(),
            CF_NOTIFICATION_SUSPENSION_BEHAVIOR_DELIVER_IMMEDIATELY,
        );

        println!("Watcher active. Listening for media events...");
        loop {
            CFRunLoop::run_in_mode(kCFRunLoopDefaultMode, Duration::from_secs(3600 * 24), false);
        }
    }
}
