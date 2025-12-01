# Quick Start Guide - Windows Demo

## For Windows Users

### Running the Pre-built Application

1. Get the `opengolfcoach-windows-demo.exe` file
2. Double-click to run
3. A window will open showing the shot monitor
4. Send shot data via TCP:
   - Port 10000: OpenGolfCoach JSON format
   - Port 921: Open API protocol
5. Watch the results appear in real-time!

### Building from Source

1. Install Rust: Download from https://rustup.rs/ and run the installer
2. Open Command Prompt
3. Navigate to this folder:
   ```cmd
   cd path\to\OpenGolfCoach\windows-demo
   ```
4. Run the build script:
   ```cmd
   build-windows.bat
   ```
5. Find your executable at: `target\release\opengolfcoach-windows-demo.exe`

## For Mac/Linux Users (Cross-Compiling)

### Building for Windows from Mac

```bash
cd windows-demo
./build-windows.sh
```

The Windows executable will be at:
```
target/x86_64-pc-windows-gnu/release/opengolfcoach-windows-demo.exe
```

Transfer this file to a Windows machine and run it!

## Testing the Application

### Method 1: Using Included Test Scripts

The easiest way to test both protocols:

```bash
# Test OpenGolfCoach format (port 10000)
python test_client.py

# Test Open API format (port 921)
python test_openapi_client.py
```

Both scripts offer:
- Single shot testing
- Send all sample shots
- Continuous loop mode

### Method 2: Quick Python Test

**OpenGolfCoach format:**
```python
import socket, json
shot = {"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.5, "horizontal_launch_angle_degrees": -2.0, "total_spin_rpm": 2800.0, "spin_axis_degrees": 6.0}
s = socket.socket()
s.connect(('127.0.0.1', 10000))
s.sendall((json.dumps(shot) + '\n').encode())
print(s.recv(4096).decode())
s.close()
```

**OpenAPI format:**
```python
import socket, json
shot = {"DeviceID": "Test", "Units": "Yards", "ShotNumber": 1, "APIversion": "1", "BallData": {"Speed": 156.5, "SpinAxis": 6.0, "TotalSpin": 2800.0, "HLA": -2.0, "VLA": 12.5}, "ShotDataOptions": {"ContainsBallData": True, "ContainsClubData": False}}
s = socket.socket()
s.connect(('127.0.0.1', 921))
s.sendall((json.dumps(shot) + '\n').encode())
s.close()  # No response expected from Open API
```

## What You'll See

The GUI window displays:

```
┌─────────────────────────────────────┐
│  OpenGolfCoach - Shot Monitor       │
├─────────────────────────────────────┤
│ Server Status: Listening on ...     │
│                                      │
│     Latest Shot Analysis             │
│  ┌────────────────────────────┐    │
│  │      Shot Name              │    │
│  │                             │    │
│  │     Pull Fade               │    │
│  └────────────────────────────┘    │
│  ┌────────────────────────────┐    │
│  │      Shot Rank              │    │
│  │                             │    │
│  │     Good                    │    │
│  └────────────────────────────┘    │
│                                      │
│  Last Updated: 14:23:45             │
│                                      │
│  Listening on TCP port 10000        │
│    (OpenGolfCoach)                  │
│  Listening on TCP port 921          │
│    (Open API)               │
│  Send shot data to see results      │
└─────────────────────────────────────┘
```

## Troubleshooting

**Q: The window won't open**
- Make sure you're on Windows 10 or later
- Try running as administrator

**Q: Says "Failed to bind"**
- Port 10000 is already in use
- Close other applications or modify the port in src/main.rs

**Q: Windows Defender blocks it**
- This is normal for unsigned .exe files
- Click "More info" → "Run anyway"

**Q: No response when sending data**
- Check the server status in the GUI
- Ensure you're sending to 127.0.0.1:10000
- Make sure to include a newline (`\n`) at the end of your JSON

## Next Steps

- See [README.md](README.md) for detailed documentation
- Check [../examples/python/example.py](../examples/python/example.py) for data format examples
- Modify the GUI in [src/main.rs](src/main.rs) to add more features
