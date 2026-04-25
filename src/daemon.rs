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
}

pub async fn run() -> Result<()> {
    let _ = std::fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle(stream).await {
                        eprintln!("[daemon] connection error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("[daemon] accept error: {e}"),
        }
    }
}

async fn handle(stream: UnixStream) -> Result<()> {
    let mut lines = BufReader::new(stream).lines();
    while let Some(line) = lines.next_line().await? {
        match serde_json::from_str::<DaemonCmd>(&line) {
            Ok(cmd) => {
                if let Err(e) = dispatch(cmd).await {
                    eprintln!("[daemon] dispatch error: {e}");
                }
            }
            Err(e) => eprintln!("[daemon] parse error: {e} — line: {line}"),
        }
    }
    Ok(())
}

async fn dispatch(cmd: DaemonCmd) -> Result<()> {
    use crate::items::network::Network;
    match cmd {
        DaemonCmd::UpdateNetworkPopup => Network::update_popup().await,
        DaemonCmd::ToggleWifiPower { device, state } => {
            Network::toggle_wifi_power(&device, &state).await
        }
        DaemonCmd::ConnectNetwork {
            ssid,
            device,
            security,
        } => {
            Network::connect_network(&ssid, &device, &security).await?;
            Network::update_command()?;
            Network::update_popup().await
        }
    }
}

pub async fn send(json: &str) -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    stream.write_all(format!("{json}\n").as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
