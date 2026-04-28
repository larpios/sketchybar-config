use crate::events::{BluetoothData, Event, EventBus, NetworkData};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

pub const SOCKET_PATH: &str = "/tmp/sketchybar-rc.sock";

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "cmd", rename_all = "kebab-case")]
pub enum DaemonCmd {
    UpdateNetworkPopup,
    ToggleWifiPower {
        device: String,
        state: String,
    },
    ConnectNetwork {
        ssid: String,
        device: String,
        security: String,
    },
    UpdateBluetoothPopup {
        scan: bool,
    },
    ToggleBluetoothDevice {
        address: String,
    },
    Shutdown,
}

pub async fn run(bus: EventBus) -> Result<()> {
    let _ = std::fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)?;

    loop {
        let bus = bus.clone();
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle(stream, bus).await {
                        eprintln!("[daemon] connection error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("[daemon] accept error: {e}"),
        }
    }
}

async fn handle(stream: UnixStream, bus: EventBus) -> Result<()> {
    let mut lines = BufReader::new(stream).lines();
    while let Some(line) = lines.next_line().await? {
        match serde_json::from_str::<DaemonCmd>(&line) {
            Ok(cmd) => {
                if let DaemonCmd::Shutdown = cmd {
                    println!("[daemon] shutdown requested");
                    std::process::exit(0);
                }

                let event = match cmd {
                    DaemonCmd::UpdateNetworkPopup => Event::UpdateNetwork,
                    DaemonCmd::ToggleWifiPower { device, state } => {
                        Event::NetworkAction(NetworkData {
                            action: format!("toggle-power:{}", state),
                            ssid: Some(device),
                        })
                    }
                    DaemonCmd::ConnectNetwork {
                        ssid,
                        device,
                        security,
                    } => Event::NetworkAction(NetworkData {
                        action: format!("connect:{}:{}", security, device),
                        ssid: Some(ssid),
                    }),
                    DaemonCmd::UpdateBluetoothPopup { scan } => Event::UpdateBluetooth { scan },
                    DaemonCmd::ToggleBluetoothDevice { address } => {
                        Event::BluetoothAction(BluetoothData {
                            action: "toggle".to_string(),
                            address: Some(address),
                        })
                    }
                    DaemonCmd::Shutdown => unreachable!(),
                };

                if let Err(e) = bus.send(event) {
                    eprintln!("[daemon] broadcast error: {e}");
                }
            }
            Err(e) => eprintln!("[daemon] parse error: {e} — line: {line}"),
        }
    }
    Ok(())
}

pub async fn send(json: &str) -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    stream.write_all(format!("{json}\n").as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn stop() -> Result<()> {
    send(r#"{"cmd": "shutdown"}"#).await
}
