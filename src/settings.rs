pub struct Settings {
    icons: Icons,
}

struct Icons {
    apple: &'static str,
    workspace: &'static str,
    workspace_active: &'static str,
    clock: &'static str,
    cpu: &'static str,
    wifi: &'static str,
    wifi_off: &'static str,
    bluetooth: &'static str,
    weather: &'static str,
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            apple: "",
            workspace: "",
            workspace_active: "",
            clock: "",
            cpu: "",
            wifi: "",
            wifi_off: "󰖪",
            bluetooth: "",
            weather: "",
        }
    }
}
