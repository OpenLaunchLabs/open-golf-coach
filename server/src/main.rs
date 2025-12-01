use opengolfcoach::bindings::calculate_derived_values;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const SERVER_PORT: u16 = 10000;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("New connection from: {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut input_json = String::new();

    // Read the JSON input (expecting one line of JSON)
    match reader.read_line(&mut input_json) {
        Ok(0) => {
            println!("Client {} disconnected", peer_addr);
            return Ok(());
        }
        Ok(bytes_read) => {
            println!(
                "Received {} bytes from {}: {}",
                bytes_read,
                peer_addr,
                input_json.trim()
            );
        }
        Err(e) => {
            eprintln!("Error reading from {}: {}", peer_addr, e);
            return Err(e);
        }
    }

    // Validate that it's valid JSON before processing
    if let Err(e) = serde_json::from_str::<serde_json::Value>(&input_json) {
        let error_msg = format!("{{\"error\": \"Invalid JSON: {}\"}}\n", e);
        stream.write_all(error_msg.as_bytes())?;
        stream.flush()?;
        println!("Sent error response to {}", peer_addr);
        return Ok(());
    }

    // Process the golf shot calculation
    match calculate_derived_values(&input_json.trim()) {
        Ok(result_json) => {
            // Compact the JSON (remove pretty-printing newlines) to ensure line-delimited protocol
            let compacted = match serde_json::from_str::<serde_json::Value>(&result_json) {
                Ok(json) => serde_json::to_string(&json).unwrap_or(result_json),
                Err(_) => result_json, // If parsing fails, use original
            };

            // Send the result back to the client with a newline
            let response = format!("{}\n", compacted);
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
            println!("Sent {} bytes response to {}", response.len(), peer_addr);
        }
        Err(e) => {
            // Send error response
            let error_msg = format!("{{\"error\": \"Calculation error: {:?}\"}}\n", e);
            stream.write_all(error_msg.as_bytes())?;
            stream.flush()?;
            println!("Sent error response to {}: {:?}", peer_addr, e);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let bind_addr = format!("127.0.0.1:{}", SERVER_PORT);
    let listener = TcpListener::bind(&bind_addr)?;

    println!("OpenGolfCoach TCP Server");
    println!("========================");
    println!("Listening on {}", bind_addr);
    println!("Waiting for connections...\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each client connection
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}
