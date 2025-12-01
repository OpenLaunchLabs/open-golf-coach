#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use opengolfcoach::bindings::calculate_derived_values;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use serde::Deserialize;

const SERVER_PORT: u16 = 10000;
const OPENAPI_PORT: u16 = 921;

// Tooltip data structures
#[derive(Debug, Deserialize)]
struct ShotTooltip {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Definitions {
    en: DefinitionsEn,
}

#[derive(Debug, Deserialize)]
struct DefinitionsEn {
    us_customary_units: Option<HashMap<String, String>>,
    trajectory_derived_values: Option<HashMap<String, String>>,
    clubhead_values: Option<HashMap<String, String>>,
}

// Load embedded tooltip files
fn load_shot_tooltips() -> HashMap<String, String> {
    let toml_str = include_str!("../../locales/shots/shots_en.toml");

    // The structure is: [shots.en.shot_name]
    #[derive(Deserialize)]
    struct ShotsFile {
        shots: ShotsContainer,
    }

    #[derive(Deserialize)]
    struct ShotsContainer {
        en: HashMap<String, ShotTooltip>,
    }

    let mut tooltips = HashMap::new();
    if let Ok(parsed) = toml::from_str::<ShotsFile>(toml_str) {
        for (key, shot) in parsed.shots.en {
            tooltips.insert(key, shot.description.trim().to_string());
        }
    }
    tooltips
}

fn load_definition_tooltips() -> HashMap<String, String> {
    let toml_str = include_str!("../../locales/definitions/definitions_en.toml");

    // The structure is: [definitions.en.category]
    #[derive(Deserialize)]
    struct DefinitionsFile {
        definitions: Definitions,
    }

    let mut tooltips = HashMap::new();
    if let Ok(parsed) = toml::from_str::<DefinitionsFile>(toml_str) {
        let def_en = &parsed.definitions.en;

        // Load US customary units
        if let Some(us_units) = &def_en.us_customary_units {
            for (key, value) in us_units {
                tooltips.insert(key.clone(), value.trim().to_string());
            }
        }

        // Load trajectory derived values
        if let Some(traj) = &def_en.trajectory_derived_values {
            for (key, value) in traj {
                tooltips.insert(key.clone(), value.trim().to_string());
            }
        }

        // Load clubhead values
        if let Some(club) = &def_en.clubhead_values {
            for (key, value) in club {
                tooltips.insert(key.clone(), value.trim().to_string());
            }
        }
    }
    tooltips
}

#[derive(Clone, Debug)]
struct ShotResult {
    shot_name: String,
    shot_rank: String,
    shot_color_rgb: String,
    timestamp: String,
    // Ball flight data
    carry_distance_yards: Option<f64>,
    offline_distance_yards: Option<f64>,
    total_distance_yards: Option<f64>,
    peak_height_yards: Option<f64>,
    hang_time_seconds: Option<f64>,
    descent_angle_degrees: Option<f64>,
    // Club data
    club_speed_mph: Option<f64>,
    smash_factor: Option<f64>,
    optimal_maximum_distance_meters: Option<f64>,
    distance_efficiency_percent: Option<f64>,
    club_path_degrees: Option<f64>,
    club_face_to_path_degrees: Option<f64>,
    club_face_to_target_degrees: Option<f64>,
}

impl Default for ShotResult {
    fn default() -> Self {
        Self {
            shot_name: "".to_string(),
            shot_rank: "".to_string(),
            shot_color_rgb: "0xFFFFFF".to_string(), // White default
            timestamp: "".to_string(),
            carry_distance_yards: None,
            offline_distance_yards: None,
            total_distance_yards: None,
            peak_height_yards: None,
            hang_time_seconds: None,
            descent_angle_degrees: None,
            club_speed_mph: None,
            smash_factor: None,
            optimal_maximum_distance_meters: None,
            distance_efficiency_percent: None,
            club_path_degrees: None,
            club_face_to_path_degrees: None,
            club_face_to_target_degrees: None,
        }
    }
}

struct OpenGolfCoachApp {
    latest_result: Arc<Mutex<ShotResult>>,
    _runtime: Arc<Runtime>, // Keep runtime alive for background tasks
    server_status: Arc<Mutex<String>>,
    shot_tooltips: HashMap<String, String>,
    definition_tooltips: HashMap<String, String>,
    is_left_handed: Arc<AtomicBool>,
}

impl OpenGolfCoachApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Create Tokio runtime for async TCP operations
        let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
        let latest_result = Arc::new(Mutex::new(ShotResult::default()));
        let server_status = Arc::new(Mutex::new("Starting...".to_string()));
        let is_left_handed = Arc::new(AtomicBool::new(false)); // Default to right-handed

        // Start original TCP server in background (port 10000)
        let latest_result_clone = Arc::clone(&latest_result);
        let server_status_clone = Arc::clone(&server_status);
        let is_left_handed_clone = Arc::clone(&is_left_handed);

        runtime.spawn(async move {
            Self::run_tcp_server(latest_result_clone, server_status_clone, is_left_handed_clone).await;
        });

        // Start OpenAPI server in background (port 921)
        let latest_result_clone2 = Arc::clone(&latest_result);
        let server_status_clone2 = Arc::clone(&server_status);
        let is_left_handed_clone2 = Arc::clone(&is_left_handed);

        runtime.spawn(async move {
            Self::run_openapi_server(latest_result_clone2, server_status_clone2, is_left_handed_clone2).await;
        });

        let runtime_clone = Arc::clone(&runtime);

        // Load tooltips
        let shot_tooltips = load_shot_tooltips();
        let definition_tooltips = load_definition_tooltips();

        Self {
            latest_result,
            _runtime: runtime_clone,
            server_status,
            shot_tooltips,
            definition_tooltips,
            is_left_handed,
        }
    }

    async fn run_tcp_server(
        latest_result: Arc<Mutex<ShotResult>>,
        server_status: Arc<Mutex<String>>,
        is_left_handed: Arc<AtomicBool>,
    ) {
        let bind_addr = format!("0.0.0.0:{}", SERVER_PORT);

        let listener = match TcpListener::bind(&bind_addr).await {
            Ok(l) => {
                *server_status.lock().unwrap() = format!("Listening on {}", bind_addr);
                l
            }
            Err(e) => {
                *server_status.lock().unwrap() = format!("Failed to bind: {}", e);
                return;
            }
        };

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let latest_result = Arc::clone(&latest_result);
                    let server_status = Arc::clone(&server_status);
                    let is_left_handed = Arc::clone(&is_left_handed);

                    tokio::spawn(async move {
                        Self::handle_client(stream, addr.to_string(), latest_result, server_status, is_left_handed).await;
                    });
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }
    }

    async fn handle_client(
        mut stream: tokio::net::TcpStream,
        peer_addr: String,
        latest_result: Arc<Mutex<ShotResult>>,
        server_status: Arc<Mutex<String>>,
        is_left_handed: Arc<AtomicBool>,
    ) {
        *server_status.lock().unwrap() = format!("Connected: {}", peer_addr);

        use tokio::io::AsyncReadExt;
        let (mut reader, mut writer) = stream.split();

        // Keep connection alive and process multiple shots
        loop {
            // Read JSON - could be terminated by newline, EOF, or brace count
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 4096];

            let input_json = loop {
                match reader.read(&mut temp_buf).await {
                    Ok(0) => {
                        // Connection closed
                        *server_status.lock().unwrap() = format!("Client {} disconnected", peer_addr);
                        return;
                    }
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buf[0..n]);

                        // Try to parse as JSON - if successful, we have complete message
                        if let Ok(s) = String::from_utf8(buffer.clone()) {
                            let trimmed = s.trim();
                            if !trimmed.is_empty() && serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                                break trimmed.to_string();
                            }
                        }

                        // Also break on newline for backwards compatibility
                        if temp_buf[0..n].contains(&b'\n') {
                            if let Ok(s) = String::from_utf8(buffer.clone()) {
                                let trimmed = s.trim();
                                if !trimmed.is_empty() {
                                    break trimmed.to_string();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        *server_status.lock().unwrap() =
                            format!("Error reading from {}: {}", peer_addr, e);
                        return;
                    }
                }
            };

            *server_status.lock().unwrap() =
                format!("Received {} bytes from {}", input_json.len(), peer_addr);

            // Validate JSON
            if let Err(e) = serde_json::from_str::<serde_json::Value>(&input_json) {
                let error_msg = format!("{{\"error\": \"Invalid JSON: {}\"}}\n", e);
                let _ = writer.write_all(error_msg.as_bytes()).await;
                let _ = writer.flush().await;
                *server_status.lock().unwrap() = format!("Sent error response to {}", peer_addr);
                continue; // Continue to next shot instead of closing connection
            }

            // Apply sign inversion for left-handed golfers before processing
            let processed_json = if is_left_handed.load(Ordering::Relaxed) {
                invert_signs_for_left_handed(input_json.trim())
            } else {
                input_json.trim().to_string()
            };

            // Process the golf shot calculation and immediately convert to Send-safe types
            let (response_msg, status_msg) = {
                match calculate_derived_values(&processed_json) {
                    Ok(result_json) => {
                    // Parse the result to extract all data
                    if let Ok(result) = serde_json::from_str::<serde_json::Value>(&result_json) {
                        if let Some(ogc) = result.get("open_golf_coach") {
                            let shot_name = ogc
                                .get("shot_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let shot_rank = ogc
                                .get("shot_rank")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let shot_color_rgb = ogc
                                .get("shot_color_rgb")
                                .and_then(|v| v.as_str())
                                .unwrap_or("0xFFFFFF")
                                .to_string();

                            // Extract US customary units
                            let us_units = ogc.get("us_customary_units");
                            let carry_distance_yards = us_units
                                .and_then(|u| u.get("carry_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let offline_distance_yards = us_units
                                .and_then(|u| u.get("offline_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let total_distance_yards = us_units
                                .and_then(|u| u.get("total_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let peak_height_yards = us_units
                                .and_then(|u| u.get("peak_height_yards"))
                                .and_then(|v| v.as_f64());
                            let club_speed_mph = us_units
                                .and_then(|u| u.get("club_speed_mph"))
                                .and_then(|v| v.as_f64());

                            // Extract other metrics
                            let hang_time_seconds = ogc
                                .get("hang_time_seconds")
                                .and_then(|v| v.as_f64());
                            let descent_angle_degrees = ogc
                                .get("descent_angle_degrees")
                                .and_then(|v| v.as_f64());
                            let smash_factor = ogc
                                .get("smash_factor")
                                .and_then(|v| v.as_f64());
                            let optimal_maximum_distance_meters = ogc
                                .get("optimal_maximum_distance_meters")
                                .and_then(|v| v.as_f64());
                            let distance_efficiency_percent = ogc
                                .get("distance_efficiency_percent")
                                .and_then(|v| v.as_f64());
                            let club_path_degrees = ogc
                                .get("club_path_degrees")
                                .and_then(|v| v.as_f64());
                            let club_face_to_path_degrees = ogc
                                .get("club_face_to_path_degrees")
                                .and_then(|v| v.as_f64());
                            let club_face_to_target_degrees = ogc
                                .get("club_face_to_target_degrees")
                                .and_then(|v| v.as_f64());

                            // Get current timestamp
                            let now = chrono::Local::now();
                            let timestamp = now.format("%H:%M:%S").to_string();

                            // Update the latest result
                            *latest_result.lock().unwrap() = ShotResult {
                                shot_name,
                                shot_rank,
                                shot_color_rgb,
                                timestamp,
                                carry_distance_yards,
                                offline_distance_yards,
                                total_distance_yards,
                                peak_height_yards,
                                hang_time_seconds,
                                descent_angle_degrees,
                                club_speed_mph,
                                smash_factor,
                                optimal_maximum_distance_meters,
                                distance_efficiency_percent,
                                club_path_degrees,
                                club_face_to_path_degrees,
                                club_face_to_target_degrees,
                            };
                        }
                    }

                    // Compact the JSON and send response
                    let compacted = match serde_json::from_str::<serde_json::Value>(&result_json) {
                        Ok(json) => serde_json::to_string(&json).unwrap_or(result_json),
                        Err(_) => result_json,
                    };

                    let response = format!("{}\n", compacted);
                    let status = format!("Sent response to {} | Ready for next shot", peer_addr);
                    (response, status)
                }
                Err(e) => {
                    // Convert error to string immediately to avoid Send issues
                    let error_str = format!("{:?}", e);
                    let error_msg = format!("{{\"error\": \"Calculation error: {}\"}}\n", error_str);
                    let status = format!("Sent error to {}: {}", peer_addr, error_str);
                    (error_msg, status)
                }
            }
        }; // calculation_result is dropped here, before any await

            // Now do async operations with owned strings
            let _ = writer.write_all(response_msg.as_bytes()).await;
            let _ = writer.flush().await;
            *server_status.lock().unwrap() = status_msg;

            // Continue loop to wait for next shot on same connection
        }
    }

    // OpenAPI Protocol Server (port 921)
    async fn run_openapi_server(
        latest_result: Arc<Mutex<ShotResult>>,
        server_status: Arc<Mutex<String>>,
        is_left_handed: Arc<AtomicBool>,
    ) {
        let bind_addr = format!("0.0.0.0:{}", OPENAPI_PORT);

        let listener = match TcpListener::bind(&bind_addr).await {
            Ok(l) => {
                let current_status = server_status.lock().unwrap().clone();
                *server_status.lock().unwrap() = format!("{} | OpenAPI: {}", current_status, bind_addr);
                l
            }
            Err(e) => {
                eprintln!("Failed to bind OpenAPI server on {}: {}", bind_addr, e);
                return;
            }
        };

        println!("OpenAPI server listening on {}", bind_addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let latest_result = Arc::clone(&latest_result);
                    let server_status = Arc::clone(&server_status);
                    let is_left_handed = Arc::clone(&is_left_handed);

                    tokio::spawn(async move {
                        Self::handle_openapi_client(stream, addr.to_string(), latest_result, server_status, is_left_handed).await;
                    });
                }
                Err(e) => {
                    eprintln!("OpenAPI connection error: {}", e);
                }
            }
        }
    }

    async fn handle_openapi_client(
        mut stream: tokio::net::TcpStream,
        peer_addr: String,
        latest_result: Arc<Mutex<ShotResult>>,
        server_status: Arc<Mutex<String>>,
        is_left_handed: Arc<AtomicBool>,
    ) {
        println!("OpenAPI client connected: {}", peer_addr);

        use tokio::io::AsyncReadExt;
        let (mut reader, mut writer) = stream.split();

        // Send OpenAPI handshake/ack message upon connection
        const OPENAPI_HANDSHAKE: &str = r#"{"Code":201,"GameId":"OpenGolfCoach"}"#;
        if let Err(e) = writer
            .write_all(format!("{}\n", OPENAPI_HANDSHAKE).as_bytes())
            .await
        {
            eprintln!(
                "Failed to send OpenAPI handshake to {}: {}",
                peer_addr, e
            );
            return;
        }

        // Keep connection alive and process multiple shots
        loop {
            // Read JSON - could be terminated by newline, EOF, or brace count
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 4096];

            let input_json = loop {
                match reader.read(&mut temp_buf).await {
                    Ok(0) => {
                        // Connection closed
                        println!("OpenAPI client {} disconnected", peer_addr);
                        return;
                    }
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buf[0..n]);

                        // Try to parse as JSON - if successful, we have complete message
                        if let Ok(s) = String::from_utf8(buffer.clone()) {
                            let trimmed = s.trim();
                            if !trimmed.is_empty() && serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                                break trimmed.to_string();
                            }
                        }

                        // Also break on newline for backwards compatibility
                        if temp_buf[0..n].contains(&b'\n') {
                            if let Ok(s) = String::from_utf8(buffer.clone()) {
                                let trimmed = s.trim();
                                if !trimmed.is_empty() {
                                    break trimmed.to_string();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from OpenAPI client {}: {}", peer_addr, e);
                        return;
                    }
                }
            };

            println!("OpenAPI received from {}: {}", peer_addr, input_json.trim());

            // Parse OpenAPI message
            let openapi_data: serde_json::Value = match serde_json::from_str(&input_json) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Invalid OpenAPI JSON from {}: {}", peer_addr, e);
                    continue; // Continue to next shot instead of closing connection
                }
            };

            // Convert OpenAPI format to OpenGolfCoach format
            let ogc_input = Self::convert_openapi_to_ogc(&openapi_data);

            // Apply sign inversion for left-handed golfers before processing
            let processed_input = if is_left_handed.load(Ordering::Relaxed) {
                invert_signs_for_left_handed(&ogc_input)
            } else {
                ogc_input
            };

            // Process the shot calculation and extract all data
            let shot_result = {
                match calculate_derived_values(&processed_input) {
                    Ok(result_json) => {
                    if let Ok(result) = serde_json::from_str::<serde_json::Value>(&result_json) {
                        if let Some(ogc) = result.get("open_golf_coach") {
                            let shot_name = ogc
                                .get("shot_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let shot_rank = ogc
                                .get("shot_rank")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            let shot_color_rgb = ogc
                                .get("shot_color_rgb")
                                .and_then(|v| v.as_str())
                                .unwrap_or("0xFFFFFF")
                                .to_string();

                            // Extract US customary units
                            let us_units = ogc.get("us_customary_units");
                            let carry_distance_yards = us_units
                                .and_then(|u| u.get("carry_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let offline_distance_yards = us_units
                                .and_then(|u| u.get("offline_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let total_distance_yards = us_units
                                .and_then(|u| u.get("total_distance_yards"))
                                .and_then(|v| v.as_f64());
                            let peak_height_yards = us_units
                                .and_then(|u| u.get("peak_height_yards"))
                                .and_then(|v| v.as_f64());
                            let club_speed_mph = us_units
                                .and_then(|u| u.get("club_speed_mph"))
                                .and_then(|v| v.as_f64());

                            // Extract other metrics
                            let hang_time_seconds = ogc
                                .get("hang_time_seconds")
                                .and_then(|v| v.as_f64());
                            let descent_angle_degrees = ogc
                                .get("descent_angle_degrees")
                                .and_then(|v| v.as_f64());
                            let smash_factor = ogc
                                .get("smash_factor")
                                .and_then(|v| v.as_f64());
                            let optimal_maximum_distance_meters = ogc
                                .get("optimal_maximum_distance_meters")
                                .and_then(|v| v.as_f64());
                            let distance_efficiency_percent = ogc
                                .get("distance_efficiency_percent")
                                .and_then(|v| v.as_f64());
                            let club_path_degrees = ogc
                                .get("club_path_degrees")
                                .and_then(|v| v.as_f64());
                            let club_face_to_path_degrees = ogc
                                .get("club_face_to_path_degrees")
                                .and_then(|v| v.as_f64());
                            let club_face_to_target_degrees = ogc
                                .get("club_face_to_target_degrees")
                                .and_then(|v| v.as_f64());

                            let now = chrono::Local::now();
                            let timestamp = now.format("%H:%M:%S").to_string();

                            Some(ShotResult {
                                shot_name,
                                shot_rank,
                                shot_color_rgb,
                                timestamp,
                                carry_distance_yards,
                                offline_distance_yards,
                                total_distance_yards,
                                peak_height_yards,
                                hang_time_seconds,
                                descent_angle_degrees,
                                club_speed_mph,
                                smash_factor,
                                optimal_maximum_distance_meters,
                                distance_efficiency_percent,
                                club_path_degrees,
                                club_face_to_path_degrees,
                                club_face_to_target_degrees,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("OpenAPI calculation error: {:?}", e);
                    None
                }
            }
        };

        // Update GUI if we got results
        if let Some(result) = shot_result {
            *latest_result.lock().unwrap() = result;

            let base_status = format!("Listening on 0.0.0.0:{} | OpenAPI: 0.0.0.0:{}", SERVER_PORT, OPENAPI_PORT);
            *server_status.lock().unwrap() = format!("{} | Last: OpenAPI", base_status);
        }

            // No additional response is required after the handshake above
            println!("OpenAPI shot processed from {}", peer_addr);

            // Continue loop to wait for next shot on same connection
        }
    }

    fn render_tile(ui: &mut egui::Ui, label: &str, value: Option<String>, unit: Option<&str>, tooltip: Option<&str>) {
        let response = ui.group(|ui| {
            ui.set_width(180.0);
            ui.set_height(80.0);
            ui.vertical_centered(|ui| {
                // Label
                ui.label(
                    egui::RichText::new(label)
                        .size(13.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(3.0);

                // Value
                if let Some(val) = value {
                    let display_text = if let Some(u) = unit {
                        format!("{} {}", val, u)
                    } else {
                        val
                    };
                    ui.label(
                        egui::RichText::new(display_text)
                            .size(20.0)
                            .strong(),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("---")
                            .size(20.0)
                            .color(egui::Color32::DARK_GRAY),
                    );
                }
            });
        });

        // Add tooltip if provided
        if let Some(tip) = tooltip {
            response.response.on_hover_text(tip);
        }
    }

    fn convert_openapi_to_ogc(openapi_data: &serde_json::Value) -> String {
        let mut ogc_input = serde_json::json!({});

        // Extract BallData from OpenAPI message
        if let Some(ball_data) = openapi_data.get("BallData") {
            // Speed -> ball_speed_meters_per_second
            if let Some(speed) = ball_data.get("Speed").and_then(|v| v.as_f64()) {
                // OpenAPI typically uses mph or m/s depending on Units field
                let units = openapi_data.get("Units")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Yards");

                let speed_ms = if units.contains("Yards") || units.contains("MPH") {
                    // Assume mph if using imperial units, convert to m/s
                    speed * 0.44704
                } else {
                    speed
                };
                ogc_input["ball_speed_meters_per_second"] = serde_json::json!(speed_ms);
            }

            // VLA -> vertical_launch_angle_degrees
            if let Some(vla) = ball_data.get("VLA").and_then(|v| v.as_f64()) {
                ogc_input["vertical_launch_angle_degrees"] = serde_json::json!(vla);
            }

            // HLA -> horizontal_launch_angle_degrees
            if let Some(hla) = ball_data.get("HLA").and_then(|v| v.as_f64()) {
                ogc_input["horizontal_launch_angle_degrees"] = serde_json::json!(hla);
            }

            // TotalSpin -> total_spin_rpm
            if let Some(total_spin) = ball_data.get("TotalSpin").and_then(|v| v.as_f64()) {
                ogc_input["total_spin_rpm"] = serde_json::json!(total_spin);
            }

            // SpinAxis -> spin_axis_degrees
            if let Some(spin_axis) = ball_data.get("SpinAxis").and_then(|v| v.as_f64()) {
                ogc_input["spin_axis_degrees"] = serde_json::json!(spin_axis);
            }

            // BackSpin and SideSpin if provided separately
            if let Some(backspin) = ball_data.get("BackSpin").and_then(|v| v.as_f64()) {
                ogc_input["backspin_rpm"] = serde_json::json!(backspin);
            }
            if let Some(sidespin) = ball_data.get("SideSpin").and_then(|v| v.as_f64()) {
                ogc_input["sidespin_rpm"] = serde_json::json!(sidespin);
            }
        }

        serde_json::to_string(&ogc_input).unwrap_or_else(|_| "{}".to_string())
    }
}

// Helper function to parse hex color string (e.g., "0x23C4FF") to egui::Color32
fn parse_hex_color(hex_str: &str) -> egui::Color32 {
    let hex = hex_str.trim_start_matches("0x").trim_start_matches("#");
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return egui::Color32::from_rgb(r, g, b);
        }
    }
    egui::Color32::WHITE // Fallback
}

// Invert horizontal launch angle and spin axis for left-handed golfers
// This transforms the data so the classification system (designed for right-handed)
// produces the correct shot names for left-handed golfers
fn invert_signs_for_left_handed(input_json: &str) -> String {
    if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(input_json) {
        if let Some(obj) = json.as_object_mut() {
            // Invert horizontal_launch_angle_degrees
            if let Some(h_launch) = obj.get("horizontal_launch_angle_degrees").and_then(|v| v.as_f64()) {
                obj.insert("horizontal_launch_angle_degrees".to_string(), serde_json::json!(-h_launch));
            }
            // Invert spin_axis_degrees
            if let Some(spin_axis) = obj.get("spin_axis_degrees").and_then(|v| v.as_f64()) {
                obj.insert("spin_axis_degrees".to_string(), serde_json::json!(-spin_axis));
            }
        }
        serde_json::to_string(&json).unwrap_or_else(|_| input_json.to_string())
    } else {
        input_json.to_string()
    }
}

impl eframe::App for OpenGolfCoachApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints to keep the UI responsive
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OpenGolfCoach - Shot Monitor");
            ui.add_space(10.0);

            // Server status
            ui.horizontal(|ui| {
                ui.label("Server Status:");
                let status = self.server_status.lock().unwrap().clone();
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), status);
            });

            ui.add_space(10.0);

            // Hand toggle
            ui.horizontal(|ui| {
                let mut is_left = self.is_left_handed.load(Ordering::Relaxed);
                let left_clicked = egui::Frame::group(ui.style())
                    .show(ui, |ui| ui.selectable_value(&mut is_left, true, "Left Handed Golfer").clicked())
                    .inner;
                let right_clicked = egui::Frame::group(ui.style())
                    .show(ui, |ui| ui.selectable_value(&mut is_left, false, "Right Handed Golfer").clicked())
                    .inner;
                if left_clicked || right_clicked {
                    self.is_left_handed.store(is_left, Ordering::Relaxed);
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(20.0);

            // Display latest shot result
            let result = self.latest_result.lock().unwrap().clone();
            let shot_color = parse_hex_color(&result.shot_color_rgb);

            ui.vertical_centered(|ui| {
                ui.heading("Latest Shot Analysis");
                ui.add_space(15.0);

                // Shot Name - Large display with shot color
                let shot_group = ui.group(|ui| {
                    ui.set_min_width(400.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Shot Name")
                                .size(16.0)
                                .color(egui::Color32::GRAY),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(&result.shot_name)
                                .size(32.0)
                                .color(shot_color)
                                .strong(),
                        );
                    });
                });

                // Add tooltip for shot name
                let shot_key = result.shot_name.to_lowercase().replace(" ", "_");
                if let Some(tooltip) = self.shot_tooltips.get(&shot_key) {
                    shot_group.response.on_hover_text(tooltip);
                }

                ui.add_space(15.0);

                // Shot Rank - Large display with shot color
                ui.group(|ui| {
                    ui.set_min_width(400.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Shot Rank")
                                .size(16.0)
                                .color(egui::Color32::GRAY),
                        );
                        ui.add_space(5.0);

                        ui.label(
                            egui::RichText::new(&result.shot_rank)
                                .size(32.0)
                                .color(shot_color)
                                .strong(),
                        );
                    });
                });

                ui.add_space(15.0);

                // Timestamp
                ui.label(
                    egui::RichText::new(format!("Last Updated: {}", result.timestamp))
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);

            // Section 1: Derived Ball Flight Numbers
            ui.vertical_centered(|ui| {
                ui.heading("Derived Ball Flight Numbers");
            });
            ui.add_space(10.0);

            // First row of ball flight tiles
            ui.horizontal(|ui| {
                // Carry Distance
                Self::render_tile(ui, "Carry",
                    result.carry_distance_yards.map(|v| format!("{:.0}", v)),
                    None,
                    self.definition_tooltips.get("carry_distance_yards").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Offline Distance
                let offline_display = result.offline_distance_yards.map(|v| {
                    let abs_val = v.abs().round() as i32;
                    if v < 0.0 {
                        format!("{}L", abs_val)
                    } else {
                        format!("{}R", abs_val)
                    }
                });
                Self::render_tile(ui, "Offline", offline_display, None,
                    self.definition_tooltips.get("offline_distance_yards").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Total Distance
                Self::render_tile(ui, "Total",
                    result.total_distance_yards.map(|v| format!("{:.0}", v)),
                    None,
                    self.definition_tooltips.get("total_distance_yards").map(|s| s.as_str()));
            });

            ui.add_space(5.0);

            // Second row of ball flight tiles
            ui.horizontal(|ui| {
                // Peak Height (convert yards to feet)
                let peak_height_feet = result.peak_height_yards.map(|v| format!("{:.0}", v * 3.0));
                Self::render_tile(ui, "Peak Height", peak_height_feet, Some("ft"),
                    self.definition_tooltips.get("peak_height_yards").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Hang Time
                Self::render_tile(ui, "Hang Time",
                    result.hang_time_seconds.map(|v| format!("{:.1}", v)),
                    Some("s"),
                    self.definition_tooltips.get("hang_time_seconds").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Descent Angle
                Self::render_tile(ui, "Descent Angle",
                    result.descent_angle_degrees.map(|v| format!("{:.0}°", v)),
                    None,
                    self.definition_tooltips.get("descent_angle_degrees").map(|s| s.as_str()));
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);

            // Section 2: Derived Club Data
            ui.vertical_centered(|ui| {
                ui.heading("Derived Club Data");
            });
            ui.add_space(10.0);

            // First row of club data tiles
            ui.horizontal(|ui| {
                // Club Speed
                Self::render_tile(ui, "Club Speed",
                    result.club_speed_mph.map(|v| format!("{:.0}", v)),
                    Some("mph"),
                    self.definition_tooltips.get("club_speed_mph").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Smash Factor
                Self::render_tile(ui, "Smash Factor",
                    result.smash_factor.map(|v| format!("{:.1}", v)),
                    None,
                    self.definition_tooltips.get("smash_factor").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Distance Efficiency
                Self::render_tile(ui, "Distance Efficiency",
                    result.distance_efficiency_percent.map(|v| format!("{:.0}", v)),
                    Some("%"),
                    self.definition_tooltips.get("distance_efficiency_percent").map(|s| s.as_str()));
            });

            ui.add_space(5.0);

            // Second row of club data tiles
            ui.horizontal(|ui| {
                // Club Path
                let club_path_display = result.club_path_degrees.map(|v| {
                    let rounded = v.round() as i32;
                    if v < 0.0 {
                        format!("{}\nOut to In", rounded.abs())
                    } else {
                        format!("{}\nIn to Out", rounded)
                    }
                });
                Self::render_tile(ui, "Club Path", club_path_display, None,
                    self.definition_tooltips.get("club_path_degrees").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Face to Path
                let face_to_path_display = result.club_face_to_path_degrees.map(|v| {
                    let rounded = v.round() as i32;
                    if v < 0.0 {
                        format!("{} Closed", rounded.abs())
                    } else {
                        format!("{} Open", rounded)
                    }
                });
                Self::render_tile(ui, "Face to Path", face_to_path_display, None,
                    self.definition_tooltips.get("club_face_to_path_degrees").map(|s| s.as_str()));
                ui.add_space(5.0);

                // Face to Target
                let face_to_target_display = result.club_face_to_target_degrees.map(|v| {
                    let rounded = v.round() as i32;
                    if v < 0.0 {
                        format!("{} Closed", rounded.abs())
                    } else {
                        format!("{} Open", rounded)
                    }
                });
                Self::render_tile(ui, "Face to Target", face_to_target_display, None,
                    self.definition_tooltips.get("club_face_to_target_degrees").map(|s| s.as_str()));
            });

            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);

            // Instructions
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(format!("Listening on TCP port {} (OpenGolfCoach)", SERVER_PORT))
                        .size(14.0)
                        .color(egui::Color32::LIGHT_BLUE),
                );
                ui.label(
                    egui::RichText::new(format!("Listening on TCP port {} (Open API)", OPENAPI_PORT))
                        .size(14.0)
                        .color(egui::Color32::LIGHT_BLUE),
                );
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("Send shot data to either port to see results here")
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .size(10.0)
                        .color(egui::Color32::DARK_GRAY),
                );
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([620.0, 900.0])
            .with_min_inner_size([600.0, 700.0])
            .with_title("OpenGolfCoach - Shot Monitor"),
        ..Default::default()
    };

    eframe::run_native(
        "OpenGolfCoach Shot Monitor",
        options,
        Box::new(|cc| Ok(Box::new(OpenGolfCoachApp::new(cc)))),
    )
}
