use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

const SERVER_ADDR: &str = "127.0.0.1:10000";

fn main() -> std::io::Result<()> {
    println!("OpenGolfCoach TCP Client Example");
    println!("=================================\n");

    // Connect to the server
    println!("Connecting to {}...", SERVER_ADDR);
    let mut stream = TcpStream::connect(SERVER_ADDR)?;
    println!("Connected!\n");

    // Example golf shot data
    let shot_data = r#"{"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.5, "horizontal_launch_angle_degrees": -2.0, "total_spin_rpm": 2800.0, "spin_axis_degrees": 6.0}"#;

    println!("Sending golf shot data:");
    println!("{}\n", shot_data);

    // Send the request (must end with newline)
    let request = format!("{}\n", shot_data);
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    // Read the response
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    println!("Received response:");

    // Try to pretty-print the JSON
    match serde_json::from_str::<serde_json::Value>(&response) {
        Ok(json) => {
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(_) => {
            // If JSON parsing fails, just print the raw response
            println!("{}", response);
        }
    }

    Ok(())
}
