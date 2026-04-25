use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core::ffi::c_void;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(cf: *const c_void);
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> *const c_void;
    fn TISGetInputSourceProperty(
        input_source: *const c_void,
        property_key: *const c_void,
    ) -> *const c_void;
    static kTISPropertyInputSourceID: *const c_void;
}

/// Returns the current input source bundle ID, e.g. "com.apple.keylayout.US"
/// or "com.apple.inputmethod.Kotoeri.RomajiTyping". Always reads live from
/// the system (not from the on-disk plist).
pub fn get_current_source_id() -> Option<String> {
    unsafe {
        let source = TISCopyCurrentKeyboardInputSource();
        if source.is_null() {
            return None;
        }
        let id_ptr = TISGetInputSourceProperty(source, kTISPropertyInputSourceID);
        CFRelease(source);
        if id_ptr.is_null() {
            return None;
        }
        let cf_str: CFString = TCFType::wrap_under_get_rule(id_ptr as _);
        Some(cf_str.to_string())
    }
}
