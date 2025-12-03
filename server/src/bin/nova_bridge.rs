use clap::{Parser, ValueEnum};
use opengolfcoach::bindings::calculate_derived_values;
use opengolfcoach_server::nova::{
    discover_nova_openapi, map_nova_shot_to_ogc, DiscoveryMethod, NovaEndpoint,
};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, ValueEnum, Debug)]
enum CliDiscovery {
    Ssdp,
    Mdns,
    Manual,
}

#[derive(Parser, Debug)]
#[command(name = "nova-bridge")]
#[command(about = "Discover Nova and feed shots into OpenGolfCoach", long_about = None)]
struct Cli {
    /// Discovery method (SSDP by default)
    #[arg(long, value_enum, default_value_t = CliDiscovery::Ssdp)]
    discovery: CliDiscovery,

    /// Nova host (required when --discovery=manual)
    #[arg(long)]
    nova_host: Option<String>,

    /// Nova port (default 2921 for manual mode)
    #[arg(long, default_value_t = 2921)]
    nova_port: u16,

    /// Discovery timeout in seconds
    #[arg(long, default_value_t = 5)]
    discovery_timeout_secs: u64,

    /// Optional local output port to rebroadcast enriched shots (OpenGolfCoach format)
    #[arg(long)]
    output_port: Option<u16>,

    /// Seconds to wait before reconnecting if Nova disconnects
    #[arg(long, default_value_t = 3)]
    reconnect_delay_secs: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let output_clients = if let Some(port) = cli.output_port {
        println!("Starting local output server on 0.0.0.0:{port}");
        Some(start_output_server(port)?)
    } else {
        None
    };

    loop {
        let endpoint = match resolve_endpoint(&cli) {
            Ok(ep) => ep,
            Err(e) => {
                eprintln!("{e}");
                thread::sleep(Duration::from_secs(cli.reconnect_delay_secs));
                continue;
            }
        };

        println!(
            "Connecting to Nova OpenAPI at {}:{}",
            endpoint.host, endpoint.port
        );

        match TcpStream::connect_timeout(
            &format!("{}:{}", endpoint.host, endpoint.port).parse()?,
            Duration::from_secs(5),
        ) {
            Ok(stream) => {
                println!("Connected. Waiting for shots...");
                if let Err(e) = process_stream(stream, output_clients.clone()) {
                    eprintln!("Connection dropped: {e}");
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to Nova at {}:{} -> {e}", endpoint.host, endpoint.port);
            }
        }

        println!("Retrying in {}s...", cli.reconnect_delay_secs);
        thread::sleep(Duration::from_secs(cli.reconnect_delay_secs));
    }
}

fn resolve_endpoint(cli: &Cli) -> Result<NovaEndpoint, String> {
    match cli.discovery {
        CliDiscovery::Manual => {
            let host = cli
                .nova_host
                .clone()
                .ok_or_else(|| "Missing --nova-host when discovery=manual".to_string())?;
            Ok(NovaEndpoint {
                host,
                port: cli.nova_port,
            })
        }
        CliDiscovery::Ssdp => {
            println!("Discovering Nova via SSDP...");
            match discover_nova_openapi(
                DiscoveryMethod::Ssdp,
                Duration::from_secs(cli.discovery_timeout_secs),
            ) {
                Ok(ep) => Ok(ep),
                Err(e) => {
                    eprintln!("SSDP discovery failed ({e}), trying mDNS...");
                    discover_nova_openapi(
                        DiscoveryMethod::Mdns,
                        Duration::from_secs(cli.discovery_timeout_secs),
                    )
                }
            }
        }
        CliDiscovery::Mdns => {
            println!("Discovering Nova via mDNS...");
            discover_nova_openapi(
                DiscoveryMethod::Mdns,
                Duration::from_secs(cli.discovery_timeout_secs),
            )
        }
    }
}

fn process_stream(
    stream: TcpStream,
    output_clients: Option<Arc<Mutex<Vec<TcpStream>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Err("Nova closed the connection".into());
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match handle_shot(trimmed) {
            Ok(result) => {
                println!("Processed shot -> {}", result);
                if let Some(clients) = &output_clients {
                    broadcast(clients, &result);
                }
            }
            Err(e) => eprintln!("Failed to process shot: {e} | raw={trimmed}"),
        }
    }
}

fn handle_shot(raw_line: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_json: Value = serde_json::from_str(raw_line)?;
    let ogc_input = map_nova_shot_to_ogc(&raw_json)
        .ok_or_else(|| "Unable to map Nova shot into OpenGolfCoach schema".to_string())?;

    let ogc_input_str = serde_json::to_string(&ogc_input)?;
    let enriched = calculate_derived_values(&ogc_input_str)
        .map_err(|e| format!("OpenGolfCoach calculation error: {:?}", e))?;
    Ok(enriched)
}

fn start_output_server(
    port: u16,
) -> Result<Arc<Mutex<Vec<TcpStream>>>, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let clients_clone = Arc::clone(&clients);

    thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => {
                    let peer = stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    println!("Local client connected: {peer}");
                    clients_clone.lock().unwrap().push(stream);
                }
                Err(e) => eprintln!("Local output client connection error: {e}"),
            }
        }
    });

    Ok(clients)
}

fn broadcast(clients: &Arc<Mutex<Vec<TcpStream>>>, message: &str) {
    let mut clients_guard = clients.lock().unwrap();
    let mut to_remove = Vec::new();

    for (idx, client) in clients_guard.iter_mut().enumerate() {
        if let Err(e) = writeln!(client, "{message}") {
            eprintln!("Failed to send to client: {e}");
            to_remove.push(idx);
        }
    }

    for idx in to_remove.into_iter().rev() {
        clients_guard.remove(idx);
    }
}
