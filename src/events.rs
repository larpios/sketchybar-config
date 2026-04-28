use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum Event {
    UpdateClock,
    UpdateWeather,
    UpdateBattery,
    UpdateCpu,
    UpdateKeyboardLayout,
    UpdateNetwork,
    UpdateVolume,
    UpdateMedia,
    UpdateWorkspaces,
    UpdateBluetooth { scan: bool },
    // Add specific data for complex events if needed
    NetworkAction(NetworkData),
    BluetoothAction(BluetoothData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkData {
    pub action: String,
    pub ssid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothData {
    pub action: String,
    pub address: Option<String>,
}

pub type EventBus = tokio::sync::broadcast::Sender<Event>;
