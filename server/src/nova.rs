use serde_json::Value;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct NovaEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryMethod {
    Ssdp,
    Mdns,
}

pub fn discover_nova_openapi(
    preferred: DiscoveryMethod,
    timeout: Duration,
) -> Result<NovaEndpoint, String> {
    match preferred {
        DiscoveryMethod::Ssdp => discover_via_ssdp(timeout)
            .or_else(|e| Err(format!("SSDP discovery failed: {e}"))),
        DiscoveryMethod::Mdns => discover_via_mdns(timeout)
            .or_else(|e| Err(format!("mDNS discovery failed: {e}"))),
    }
}

fn discover_via_ssdp(timeout: Duration) -> Result<NovaEndpoint, String> {
    const SSDP_MULTICAST_ADDR: &str = "239.255.255.250";
    const SSDP_PORT: u16 = 1900;
    const SERVICE_URN: &str = "urn:openlaunch:service:openapi:1";

    let sock = UdpSocket::bind(("0.0.0.0", 0)).map_err(|e| e.to_string())?;
    sock.set_read_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;

    let search_request = format!(
        "M-SEARCH * HTTP/1.1\r\n\
         HOST: {SSDP_MULTICAST_ADDR}:{SSDP_PORT}\r\n\
         MAN: \"ssdp:discover\"\r\n\
         MX: 3\r\n\
         ST: {SERVICE_URN}\r\n\
         \r\n"
    );

    sock.send_to(
        search_request.as_bytes(),
        (SSDP_MULTICAST_ADDR, SSDP_PORT),
    )
    .map_err(|e| e.to_string())?;

    let start = Instant::now();
    let mut buf = [0u8; 2048];
    while start.elapsed() < timeout {
        match sock.recv_from(&mut buf) {
            Ok((n, _addr)) => {
                let response = String::from_utf8_lossy(&buf[..n]);
                if !response.contains(SERVICE_URN) {
                    continue;
                }

                let mut location = None;
                for line in response.lines() {
                    if let Some(rest) = line.strip_prefix("LOCATION:") {
                        location = Some(rest.trim().to_string());
                    }
                }

                if let Some(loc) = location {
                    if let Some(endpoint) = parse_location(&loc) {
                        return Ok(endpoint);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.to_string()),
        }
    }

    Err("No SSDP response received".into())
}

fn discover_via_mdns(timeout: Duration) -> Result<NovaEndpoint, String> {
    use mdns_sd::{ServiceDaemon, ServiceEvent};

    let mdns = ServiceDaemon::new().map_err(|e| e.to_string())?;
    let receiver = mdns
        .browse("_openapi-nova._tcp.local.")
        .map_err(|e| e.to_string())?;

    let start = Instant::now();
    while start.elapsed() < timeout {
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                if let Some(endpoint) = info
                    .get_addresses()
                    .iter()
                    .next()
                    .map(|addr| (addr.to_string(), info.get_port()))
                {
                    return Ok(NovaEndpoint {
                        host: endpoint.0,
                        port: endpoint.1,
                    });
                }
            }
            Ok(_) => continue,
            Err(flume::RecvTimeoutError::Timeout) => continue,
            Err(e) => return Err(e.to_string()),
        }
    }

    Err("No mDNS response received".into())
}

fn parse_location(location: &str) -> Option<NovaEndpoint> {
    let trimmed = location
        .trim_start_matches("http://")
        .trim_start_matches("ws://")
        .trim_end_matches('/');

    let host_and_port = trimmed.split('/').next().unwrap_or(trimmed);

    let mut parts = host_and_port.splitn(2, ':');
    let host = parts.next()?.to_string();
    let port_str = parts.next()?;
    let port: u16 = port_str.parse().ok()?;

    Some(NovaEndpoint { host, port })
}

/// Map Nova OpenAPI shot JSON into OpenGolfCoach input schema.
pub fn map_nova_shot_to_ogc(input: &Value) -> Option<Value> {
    // If the input already looks like OpenGolfCoach format, pass it through.
    if input.get("ball_speed_meters_per_second").is_some()
        && input.get("vertical_launch_angle_degrees").is_some()
    {
        return Some(input.clone());
    }

    let mut ogc = serde_json::Map::new();
    let mut us_customary = serde_json::Map::new();

    let units = input
        .get("Units")
        .and_then(|v| v.as_str())
        .unwrap_or("Yards");
    let uses_imperial = units.to_ascii_lowercase().contains("yard")
        || units.to_ascii_lowercase().contains("mph");

    if let Some(ball) = input.get("BallData") {
        if let Some(speed) = ball.get("Speed").and_then(|v| v.as_f64()) {
            let speed_ms = if uses_imperial { speed * 0.44704 } else { speed };
            ogc.insert(
                "ball_speed_meters_per_second".to_string(),
                serde_json::json!(speed_ms),
            );
            if uses_imperial {
                us_customary.insert("ball_speed_mph".to_string(), serde_json::json!(speed));
            }
        }

        if let Some(vla) = ball.get("VLA").and_then(|v| v.as_f64()) {
            ogc.insert(
                "vertical_launch_angle_degrees".to_string(),
                serde_json::json!(vla),
            );
        }

        if let Some(hla) = ball.get("HLA").and_then(|v| v.as_f64()) {
            ogc.insert(
                "horizontal_launch_angle_degrees".to_string(),
                serde_json::json!(hla),
            );
        }

        if let Some(total_spin) = ball.get("TotalSpin").and_then(|v| v.as_f64()) {
            ogc.insert("total_spin_rpm".to_string(), serde_json::json!(total_spin));
        }

        if let Some(spin_axis) = ball.get("SpinAxis").and_then(|v| v.as_f64()) {
            ogc.insert(
                "spin_axis_degrees".to_string(),
                serde_json::json!(spin_axis),
            );
        }

        if let Some(backspin) = ball.get("BackSpin").and_then(|v| v.as_f64()) {
            ogc.insert("backspin_rpm".to_string(), serde_json::json!(backspin));
        }

        if let Some(side_spin) = ball.get("SideSpin").and_then(|v| v.as_f64()) {
            ogc.insert("sidespin_rpm".to_string(), serde_json::json!(side_spin));
        }
    }

    if let Some(club) = input.get("ClubData") {
        if let Some(speed) = club.get("Speed").and_then(|v| v.as_f64()) {
            if uses_imperial {
                us_customary.insert("club_speed_mph".to_string(), serde_json::json!(speed));
            } else {
                // If metric is supplied, convert to mph for completeness
                us_customary.insert(
                    "club_speed_mph".to_string(),
                    serde_json::json!(speed / 0.44704),
                );
            }
        }
    }

    if !us_customary.is_empty() {
        ogc.insert(
            "us_customary_units".to_string(),
            Value::Object(us_customary),
        );
    }

    if ogc.is_empty() {
        None
    } else {
        Some(Value::Object(ogc))
    }
}
