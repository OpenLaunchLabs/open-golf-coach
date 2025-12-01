# OpenGolfCoach TCP Server

A simple TCP socket server that accepts JSON input for golf shot calculations and returns the computed results.

## Overview

This server provides a network interface to the OpenGolfCoach library, allowing clients to send golf shot data over TCP and receive calculated derived values.

## Features

- **TCP Socket Server** on port 1000
- **JSON-based protocol** for easy integration
- **Line-delimited messages** (one JSON object per line)
- **Error handling** with JSON error responses
- **Connection logging** for debugging

## Building

Build the server and client using Cargo:

```bash
cd server
cargo build --release
```

The compiled binaries will be available at:
- **Server**: `target/release/server`
- **Client**: `target/release/client`

## Running

### Server

Start the server:

```bash
cargo run --release --bin server
```

Or run the compiled binary directly:

```bash
./target/release/server
```

The server will start listening on `127.0.0.1:1000`.

**Note**: Port 1000 requires elevated privileges on Unix-based systems (macOS, Linux). If you get a "Permission denied" error, you can either:

1. Run with sudo: `sudo ./target/release/server`
2. Change the port in [src/main.rs:5](src/main.rs) to a port ≥1024 (e.g., 8000) which doesn't require special permissions

### Client Example

Once the server is running, test it with the included Rust client:

```bash
# In a separate terminal
cargo run --release --bin client
```

Or run the compiled binary:

```bash
./target/release/client
```

The client will connect to the server, send a sample golf shot, and print the results.

## Usage

### Protocol

The server uses a simple line-delimited JSON protocol:

1. **Connect** to the server on port 1000
2. **Send** a JSON object representing golf shot data (terminated with newline `\n`)
3. **Receive** a JSON response with calculated values (terminated with newline `\n`)

### Input Format

Send golf shot data as a JSON object. Example:

```json
{"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.5, "horizontal_launch_angle_degrees": -2.0, "total_spin_rpm": 2800.0, "spin_axis_degrees": 6.0}
```

See the [API documentation](../API.md) for all available input fields.

### Output Format

The server responds with a JSON object containing the original input plus calculated derived values:

```json
{
  "ball_speed_meters_per_second": 70.0,
  "vertical_launch_angle_degrees": 12.5,
  "horizontal_launch_angle_degrees": -2.0,
  "total_spin_rpm": 2800.0,
  "spin_axis_degrees": 6.0,
  "open_golf_coach": {
    "carry_distance_meters": 182.5,
    "total_distance_meters": 195.3,
    "offline_distance_meters": -6.4,
    "peak_height_meters": 28.7,
    "hang_time_seconds": 5.2,
    "descent_angle_degrees": 42.3,
    "shot_name": "Pull Fade",
    "shot_rank": "good",
    "shot_color_rgb": "#4CAF50"
  }
}
```

### Error Responses

If an error occurs (invalid JSON, calculation error), the server responds with:

```json
{"error": "Error description"}
```

## Example Clients

### Rust Client (Included)

The server includes a simple Rust client example at [src/client.rs](src/client.rs). To run it:

```bash
# Start the server in one terminal
cargo run --release --bin server

# In another terminal, run the client
cargo run --release --bin client
```

The client will send a sample golf shot and print the pretty-printed JSON response.

### Netcat

You can also test the server using `netcat` or `nc`:

```bash
# Start the server in one terminal
cargo run --release --bin server

# In another terminal, send a test request
echo '{"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.5, "horizontal_launch_angle_degrees": -2.0, "total_spin_rpm": 2800.0, "spin_axis_degrees": 6.0}' | nc localhost 1000
```

## Example Client (Python)

```python
import socket
import json

# Connect to server
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(('127.0.0.1', 1000))

# Prepare input
shot_data = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "horizontal_launch_angle_degrees": -2.0,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 6.0
}

# Send request (must end with newline)
request = json.dumps(shot_data) + '\n'
sock.sendall(request.encode())

# Receive response
response = sock.recv(4096).decode()
result = json.loads(response)

print(json.dumps(result, indent=2))

sock.close()
```

## Example Client (Node.js)

```javascript
const net = require('net');

const client = net.createConnection({ port: 1000, host: '127.0.0.1' }, () => {
  const shotData = {
    ball_speed_meters_per_second: 70.0,
    vertical_launch_angle_degrees: 12.5,
    horizontal_launch_angle_degrees: -2.0,
    total_spin_rpm: 2800.0,
    spin_axis_degrees: 6.0
  };

  // Send request (must end with newline)
  client.write(JSON.stringify(shotData) + '\n');
});

client.on('data', (data) => {
  const result = JSON.parse(data.toString());
  console.log(JSON.stringify(result, null, 2));
  client.end();
});
```

## Server Configuration

- **Port**: 1000 (defined in `src/main.rs` as `SERVER_PORT`)
- **Bind Address**: `127.0.0.1` (localhost only by default)

To change the port or bind address, modify the constants in [src/main.rs](src/main.rs).

## Limitations

- **Single connection at a time**: The server processes one connection synchronously before accepting the next
- **No authentication**: The server does not implement any authentication mechanism
- **Localhost only**: Binds to 127.0.0.1 by default for security
- **Line-delimited protocol**: Each message must be a single line (no multi-line JSON)

## Future Enhancements

Potential improvements for production use:

- Multi-threaded or async connection handling
- Authentication and authorization
- TLS/SSL encryption
- Configuration file support
- Persistent connections with multiple requests
- Binary protocol option for better performance
- Connection pooling and rate limiting

## License

Apache 2.0 - See [LICENSE](../LICENSE)
