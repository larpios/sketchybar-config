use crate::events::{Event, EventBus};
use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopDefaultMode};
use core_foundation::string::CFString;
use lazy_static::lazy_static;
use media_remote::NowPlayingJXA;
use std::os::raw::c_void;
use std::ptr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref GLOBAL_BUS: Mutex<Option<EventBus>> = Mutex::new(None);
}

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
    if let Some(bus) = &*GLOBAL_BUS.lock().unwrap() {
        let _ = bus.send(Event::UpdateMedia);
    }
}

unsafe extern "C" fn keyboard_layout_callback(
    _center: *mut c_void,
    _observer: *mut c_void,
    _name: *mut c_void,
    _object: *mut c_void,
    _user_info: *mut c_void,
) {
    let source_id = crate::keyboard_ffi::get_current_source_id().unwrap_or_default();
    if let Some(bus) = &*GLOBAL_BUS.lock().unwrap() {
        let _ = bus.send(Event::UpdateKeyboardLayout {
            source_id: Some(source_id),
        });
    }
}

pub fn watch(bus: EventBus) -> anyhow::Result<()> {
    println!("Starting system events watcher...");

    {
        let mut global_bus = GLOBAL_BUS.lock().unwrap();
        *global_bus = Some(bus.clone());
    }

    // Heartbeat for progress bar (only triggers when playing)
    let bus_heartbeat = bus.clone();
    thread::spawn(move || {
        loop {
            let now_playing = NowPlayingJXA::new(Duration::from_secs(1));
            let guard = now_playing.get_info();
            let info = guard.as_ref();

            if info.is_some_and(|info| info.is_playing.unwrap_or_default()) {
                let _ = bus_heartbeat.send(Event::UpdateMedia);
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });

    // Run the notification listener on the main thread
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

        println!("Watcher active. Listening for system events...");
        loop {
            CFRunLoop::run_in_mode(kCFRunLoopDefaultMode, Duration::from_secs(3600 * 24), false);
        }
    }
}
