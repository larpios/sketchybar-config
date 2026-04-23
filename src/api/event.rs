use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BarEvent {
    /// Triggered when an update frequency is set for an item
    Routine,
    /// Triggered when the bar is forced to update
    Forced,
    /// Triggered when the mouse enters over a specific item
    MouseEntered,
    /// Triggered when the mouse leaves a specific item
    MouseExited,
    /// Triggered when the mouse enters over any part of the bar
    MouseEnteredGlobal,
    /// Triggered when the mouse leaves all parts of the bar
    MouseExitedGlobal,
    /// Triggered when an item is clicked
    MouseClicked,
    /// Triggered when the mouse is scrolled over an item
    MouseScrolled,
    /// Triggered when the mouse is scrolled over an empty region of the bar
    MouseScrolledGlobal,
    /// Triggered when the front application changes
    FrontAppSwitched,
    /// Triggered when the active mission control space changes
    SpaceChange,
    /// Triggered when a window is created or destroyed on a space
    SpaceWindowsChange,
    /// Triggered when the active display is changed
    DisplayChange,
    /// Triggered when the system audio volume is changed
    VolumeChange,
    /// Triggered when a display's brightness is changed
    BrightnessChange,
    /// Triggered when the device's power source is changed (AC or BATTERY)
    PowerSourceChange,
    /// Triggered when the device connects to or disconnects from Wi-Fi
    WifiChange,
    /// Triggered when a change in "now playing" media is performed
    MediaChange,
    /// Triggered when the system prepares to sleep
    SystemWillSleep,
    /// Triggered when the system has awakened from sleep
    SystemWoke,
    /// A user-defined custom event
    Custom(String),
}

impl Display for BarEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Routine => "routine",
            Self::Forced => "forced",
            Self::MouseEntered => "mouse.entered",
            Self::MouseExited => "mouse.exited",
            Self::MouseEnteredGlobal => "mouse.entered.global",
            Self::MouseExitedGlobal => "mouse.exited.global",
            Self::MouseClicked => "mouse.clicked",
            Self::MouseScrolled => "mouse.scrolled",
            Self::MouseScrolledGlobal => "mouse.scrolled.global",
            Self::FrontAppSwitched => "front_app_switched",
            Self::SpaceChange => "space_change",
            Self::SpaceWindowsChange => "space_windows_change",
            Self::DisplayChange => "display_change",
            Self::VolumeChange => "volume_change",
            Self::BrightnessChange => "brightness_change",
            Self::PowerSourceChange => "power_source_change",
            Self::WifiChange => "wifi_change",
            Self::MediaChange => "media_change",
            Self::SystemWillSleep => "system_will_sleep",
            Self::SystemWoke => "system_woke",
            Self::Custom(name) => name,
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for BarEvent {
    fn from(s: &str) -> Self {
        match s {
            "routine" => Self::Routine,
            "forced" => Self::Forced,
            "mouse.entered" => Self::MouseEntered,
            "mouse.exited" => Self::MouseExited,
            "mouse.entered.global" => Self::MouseEnteredGlobal,
            "mouse.exited.global" => Self::MouseExitedGlobal,
            "mouse.clicked" => Self::MouseClicked,
            "mouse.scrolled" => Self::MouseScrolled,
            "mouse.scrolled.global" => Self::MouseScrolledGlobal,
            "front_app_switched" => Self::FrontAppSwitched,
            "space_change" => Self::SpaceChange,
            "space_windows_change" => Self::SpaceWindowsChange,
            "display_change" => Self::DisplayChange,
            "volume_change" => Self::VolumeChange,
            "brightness_change" => Self::BrightnessChange,
            "power_source_change" => Self::PowerSourceChange,
            "wifi_change" => Self::WifiChange,
            "media_change" => Self::MediaChange,
            "system_will_sleep" => Self::SystemWillSleep,
            "system_woke" => Self::SystemWoke,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl From<String> for BarEvent {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
