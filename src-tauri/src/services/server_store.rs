use crate::models::server::{ServerBookmark, ServerStatus};
use anyhow::Result;
use std::fs;

pub struct ServerStore {
    file: std::path::PathBuf,
}

impl ServerStore {
    pub fn new() -> Self {
        let dir = crate::utils::get_app_data_dir();
        let _ = fs::create_dir_all(&dir);
        Self {
            file: dir.join("servers.json"),
        }
    }

    pub fn list(&self) -> Result<Vec<ServerBookmark>> {
        if self.file.exists() {
            let raw = fs::read_to_string(&self.file)?;
            Ok(serde_json::from_str(&raw).unwrap_or_default())
        } else {
            Ok(vec![])
        }
    }

    pub fn add(&self, bookmark: ServerBookmark) -> Result<ServerBookmark> {
        let mut servers = self.list()?;
        servers.push(bookmark.clone());
        self.persist(&servers)?;
        Ok(bookmark)
    }

    pub fn remove(&self, id: &str) -> Result<()> {
        let mut servers = self.list()?;
        servers.retain(|s| s.id != id);
        self.persist(&servers)?;
        Ok(())
    }

    pub fn toggle_favorite(&self, id: &str) -> Result<Option<ServerBookmark>> {
        let mut servers = self.list()?;
        if let Some(s) = servers.iter_mut().find(|s| s.id == id) {
            s.is_favorite = !s.is_favorite;
            let updated = s.clone();
            self.persist(&servers)?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    fn persist(&self, servers: &[ServerBookmark]) -> Result<()> {
        let raw = serde_json::to_string_pretty(servers)?;
        fs::write(&self.file, raw)?;
        Ok(())
    }
}

/// Query a Minecraft server status using the modern 1.19+ ping protocol
/// (handshake + status request + read JSON response).
pub fn query_status(address: &str) -> ServerStatus {
    let parts: Vec<&str> = address.splitn(2, ':').collect();
    let host = parts[0].to_string();
    let port: u16 = parts
        .get(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(25565);

    let mut stream = match std::net::TcpStream::connect((host.as_str(), port)) {
        Ok(s) => s,
        Err(e) => {
            return ServerStatus {
                reachable: false,
                motd: None,
                online: None,
                max: None,
                version: None,
                error: Some(e.to_string()),
            }
        }
    };
    use std::io::{Read, Write};
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(8)));

    // Handshake packet: [varint(len)][0x00][47][host][port_be][0x01]
    let mut inner: Vec<u8> = Vec::new();
    push_varint(&mut inner, 0x00);
    push_varint(&mut inner, 47);
    inner.extend_from_slice(host.as_bytes());
    let port_bytes = port.to_be_bytes();
    inner.extend_from_slice(&port_bytes);
    inner.push(0x01);
    let mut payload: Vec<u8> = Vec::new();
    push_varint(&mut payload, inner.len() as u32);
    payload.extend_from_slice(&inner);
    if stream.write_all(&payload).is_err() {
        return ServerStatus {
            reachable: false,
            motd: None,
            online: None,
            max: None,
            version: None,
            error: Some("handshake write failed".into()),
        };
    }

    // Status request: [varint(1)][0x00]
    let mut req: Vec<u8> = Vec::new();
    push_varint(&mut req, 1);
    req.push(0x00);
    if stream.write_all(&req).is_err() {
        return ServerStatus {
            reachable: false,
            motd: None,
            online: None,
            max: None,
            version: None,
            error: Some("status request write failed".into()),
        };
    }

    // Read response
    let mut buf = vec![0u8; 8192];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => 0,
    };
    if n == 0 {
        return ServerStatus {
            reachable: false,
            motd: None,
            online: None,
            max: None,
            version: None,
            error: Some("no response".into()),
        };
    }
    buf.truncate(n);
    // The handshake/status packets are sent without a length prefix (the client
    // only receives them). Response layout: [0x00][varint(json_len)][json].
    let mut off = 0usize;
    // read the packet id varint (0x00 = response to status request)
    if let Some((consumed, _)) = read_varint(&buf[off..]) {
        off += consumed;
    }
    // read the json string length varint
    let json_len = if let Some((consumed, len)) = read_varint(&buf[off..]) {
        off += consumed;
        len
    } else {
        0
    };
    let json_str: String = buf
        .get(off..(off + json_len as usize))
        .and_then(|b| std::str::from_utf8(b).ok())
        .unwrap_or("")
        .trim()
        .to_string();

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
        let motd = json["description"].as_str().map(|s| s.replace('\n', " ").replace("\u{a7}", ""));
        let online = json["players"]["online"].as_u64().map(|v| v as u32);
        let max = json["players"]["max"].as_u64().map(|v| v as u32);
        let version = json["version"]["name"].as_str().map(String::from);
        return ServerStatus {
            reachable: true,
            motd,
            online,
            max,
            version,
            error: None,
        };
    }
    ServerStatus {
        reachable: true,
        motd: None,
        online: None,
        max: None,
        version: None,
        error: Some("unparseable response".into()),
    }
}

fn push_varint(buf: &mut Vec<u8>, mut value: u32) {
    loop {
        let byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            buf.push(byte | 0x80);
        } else {
            buf.push(byte);
            return;
        }
    }
}

/// Returns (bytes_consumed, value)
fn read_varint(data: &[u8]) -> Option<(usize, u32)> {
    let mut result: u32 = 0;
    let mut shift = 0;
    for (i, &b) in data.iter().enumerate() {
        result |= ((b & 0x7F) as u32) << shift;
        if b & 0x80 == 0 {
            return Some((i + 1, result));
        }
        shift += 7;
        if shift > 28 {
            return None;
        }
    }
    None
}
