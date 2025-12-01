# OpenGolfCoach Windows Demo

A portable Windows application that displays golf shot analysis in real-time via a graphical interface.

## Features

- **Dual TCP Servers**:
  - Port 10000: OpenGolfCoach native JSON format
  - Port 921: Open API protocol
- **Real-time Analysis**: Processes shot data and displays results instantly
- **GUI Display**: Shows `shot_name` and `shot_rank` in a clean, easy-to-read interface
- **Portable**: Single executable that runs without installation on Windows
- **Multi-Protocol**: Works with any launch monitor supporting either protocol

## How It Works

1. Application starts and opens two TCP listeners:
   - `0.0.0.0:10000` - OpenGolfCoach native format
   - `0.0.0.0:921` - Open API protocol
2. GUI window displays current server status
3. When shot data is received via either TCP port:
   - Converts OpenAPI format to OpenGolfCoach format (if needed)
   - Calculates derived values using OpenGolfCoach library
   - Extracts `shot_name` and `shot_rank` from results
   - Updates the GUI display with color-coded information
4. Application continues listening for the next shot on both ports

## Building for Windows

### Option 1: Cross-compile from Mac/Linux

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cd windows-demo
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be at: `target/x86_64-pc-windows-gnu/release/opengolfcoach-windows-demo.exe`

### Option 2: Build on Windows

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Open Command Prompt or PowerShell
3. Navigate to the `windows-demo` directory
4. Run:
   ```cmd
   cargo build --release
   ```

The executable will be at: `target/release/opengolfcoach-windows-demo.exe`

## Creating a Portable Distribution

The release build is already optimized and stripped. To create a portable package:

1. Build the release version (see above)
2. Copy `opengolfcoach-windows-demo.exe` to a distribution folder
3. The executable has no external dependencies and can run standalone

**Note**: The first time the app runs on a Windows system, Windows Defender may scan it. This is normal for unsigned executables.

## Usage

### Running the Application

Simply double-click `opengolfcoach-windows-demo.exe` or run from command line:

**Note**: The application runs as a GUI-only app with no console window in release mode. For debugging, build in debug mode to see console output.

```cmd
opengolfcoach-windows-demo.exe
```

The GUI window will open showing:
- Server status (listening on ports 10000 and 921)
- Latest shot name (large display)
- Latest shot rank (color-coded by quality)
- Last update timestamp

### Sending Shot Data

#### OpenGolfCoach Format (Port 10000)

Use any TCP client to send JSON data. Example using Python:

```python
import socket
import json

shot_data = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "horizontal_launch_angle_degrees": -2.0,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 6.0
}

# Connect and send
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(('127.0.0.1', 10000))
sock.sendall(json.dumps(shot_data).encode() + b'\n')

# Receive response
response = sock.recv(4096)
print(json.loads(response))

sock.close()
```

#### Open API Format (Port 921)

Send data using the Open API protocol:

```python
import socket
import json

openapi_shot = {
    "DeviceID": "MyLaunchMonitor",
    "Units": "Yards",
    "ShotNumber": 1,
    "APIversion": "1",
    "BallData": {
        "Speed": 156.5,      # mph
        "SpinAxis": 6.0,
        "TotalSpin": 2800.0,
        "HLA": -2.0,         # Horizontal Launch Angle
        "VLA": 12.5          # Vertical Launch Angle
    },
    "ShotDataOptions": {
        "ContainsBallData": True,
        "ContainsClubData": False
    }
}

# Connect and send (no response expected from OpenAPI protocol)
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(('127.0.0.1', 921))
sock.sendall(json.dumps(openapi_shot).encode() + b'\n')
sock.close()
```

#### Using Test Scripts

Use the included test clients:

```bash
# Test OpenGolfCoach format
python test_client.py

# Test OpenAPI format
python test_openapi_client.py
```

## GUI Display

The interface shows:

- **Shot Name**: Large text display of the classified shot type (e.g., "Pull Fade", "Straight", etc.), color-coded by shot rank
- **Shot Rank**: Large text display (S+, S, A, B, C, D, E), color-coded by quality:
  - S+/S (Blue): Excellent shots
  - A/B (Green): Good shots
  - C (Yellow/Orange): Average shots
  - D/E (Red/Orange): Poor shots
- **Timestamp**: When the last shot was received
- **Server Status**: Connection and processing status

## Data Format

The application expects JSON input matching the OpenGolfCoach format:

```json
{
  "ball_speed_meters_per_second": 70.0,
  "vertical_launch_angle_degrees": 12.5,
  "horizontal_launch_angle_degrees": -2.0,
  "total_spin_rpm": 2800.0,
  "spin_axis_degrees": 6.0
}
```

See the [examples/python/example.py](../examples/python/example.py) for more input examples.

## Troubleshooting

### Port Already in Use

If port 10000 or 921 is already taken, the app will start but show an error status. Close other applications using those ports or modify the port constants in [src/main.rs](src/main.rs) and rebuild.

### Firewall Blocking

Windows Firewall may prompt for network access the first time. Allow access for private networks.

### Application Won't Start

Ensure you're running on Windows 10 or later. The application uses native Windows GUI APIs that require modern Windows versions.

### Debugging Console Output

The release build hides the console window. To see console output for debugging:

```bash
# Build in debug mode (shows console)
cargo build --target x86_64-pc-windows-gnu

# The debug executable will show console output
./target/x86_64-pc-windows-gnu/debug/opengolfcoach-windows-demo.exe
```

Console messages include:
- Server startup messages
- Incoming connection notifications
- Shot data received
- Calculation errors (if any)

## Technical Details

- **Language**: Rust
- **GUI Framework**: egui/eframe (native Windows GUI)
- **Async Runtime**: Tokio (for TCP operations)
- **Core Library**: OpenGolfCoach (../core)
- **Build Target**: x86_64-pc-windows-gnu or x86_64-pc-windows-msvc

The application is fully self-contained with no external dependencies beyond the Windows system libraries.
