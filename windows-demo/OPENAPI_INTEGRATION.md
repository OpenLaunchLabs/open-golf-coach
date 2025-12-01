# Open API Integration

This document describes the Open API protocol integration in the Windows demo application.

> **Note**: The golf simulator community commonly refers to the protocol specified at [gsprogolf.com/GSProConnectV1.html](https://gsprogolf.com/GSProConnectV1.html) as "Open API". This is the protocol we've implemented on port 921.

## Overview

The Windows demo now supports **two simultaneous TCP protocols**:

1. **Port 10000**: OpenGolfCoach native JSON format
2. **Port 921**: Open API protocol

Both protocols process incoming shot data and display the same results in the GUI.

## Open API Protocol

### Protocol Details

- **Port**: 921
- **IP**: 0.0.0.0 (accepts connections from any interface)
- **Format**: Line-delimited JSON
- **Authentication**: None
- **Response**: None (one-way communication, as per requirements)

### Data Flow

```
Launch Monitor → Port 921 → Windows Demo App
                              ↓
                    Convert OpenAPI → OGC Format
                              ↓
                    Calculate Derived Values
                              ↓
                    Update GUI Display
```

## Message Format

### OpenAPI Input Format

```json
{
  "DeviceID": "LaunchMonitorName",
  "Units": "Yards",
  "ShotNumber": 1,
  "APIversion": "1",
  "BallData": {
    "Speed": 156.5,
    "SpinAxis": 6.0,
    "TotalSpin": 2800.0,
    "HLA": -2.0,
    "VLA": 12.5
  },
  "ShotDataOptions": {
    "ContainsBallData": true,
    "ContainsClubData": false
  }
}
```

### Field Mapping

The application automatically converts OpenAPI format to OpenGolfCoach format:

| OpenAPI Field | OpenGolfCoach Field | Notes |
|-------------|---------------------|-------|
| `BallData.Speed` | `ball_speed_meters_per_second` | Converted from mph if Units="Yards" |
| `BallData.VLA` | `vertical_launch_angle_degrees` | Direct mapping |
| `BallData.HLA` | `horizontal_launch_angle_degrees` | Direct mapping |
| `BallData.TotalSpin` | `total_spin_rpm` | Direct mapping |
| `BallData.SpinAxis` | `spin_axis_degrees` | Direct mapping |
| `BallData.BackSpin` | `backspin_rpm` | Optional, if provided |
| `BallData.SideSpin` | `sidespin_rpm` | Optional, if provided |

### Unit Conversion

- **Imperial (Units="Yards")**: Speed is assumed to be in mph, converted to m/s (× 0.44704)
- **Metric**: Speed is assumed to already be in m/s

## Implementation Details

### Code Structure

The OpenAPI server is implemented in [src/main.rs](src/main.rs):

1. **`run_openapi_server()`**: Main server loop, accepts connections on port 921
2. **`handle_openapi_client()`**: Processes individual OpenAPI messages
3. **`convert_openapi_to_ogc()`**: Converts OpenAPI JSON to OpenGolfCoach JSON

### Key Features

- ✅ **Non-blocking**: Runs concurrently with OpenGolfCoach server
- ✅ **No response**: Complies with OpenAPI protocol (no response sent back)
- ✅ **Automatic conversion**: Transparently converts formats
- ✅ **Unit handling**: Detects and converts imperial units
- ✅ **Shared GUI**: Updates same display as OpenGolfCoach format

## Testing

### Using the Test Client

```bash
python test_openapi_client.py
```

Options:
- Send single test shot
- Send all test shots sequentially
- Run in continuous loop mode (`--loop`)

### Manual Testing

```python
import socket
import json

openapi_shot = {
    "DeviceID": "TestDevice",
    "Units": "Yards",
    "ShotNumber": 1,
    "APIversion": "1",
    "BallData": {
        "Speed": 156.5,      # mph
        "SpinAxis": 6.0,
        "TotalSpin": 2800.0,
        "HLA": -2.0,
        "VLA": 12.5
    },
    "ShotDataOptions": {
        "ContainsBallData": True,
        "ContainsClubData": False
    }
}

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(('127.0.0.1', 921))
sock.sendall(json.dumps(openapi_shot).encode() + b'\n')
sock.close()

print("Shot sent! Check the GUI for results.")
```

## Compatibility

### Launch Monitors

Any launch monitor that supports Open API can send data to this application:

- Configure the launch monitor to connect to:
  - **IP**: `127.0.0.1` (or the Windows PC's IP if on same network)
  - **Port**: `921`

The application will receive the data, process it, and display the shot analysis.

### OpenAPI Golf Simulator

While this application doesn't replace OpenAPI, it can receive the same data that would normally be sent to OpenAPI. This allows you to:

1. See shot analysis in real-time
2. Classify shots independently
3. Use OpenGolfCoach's shot ranking system
4. Run alongside OpenAPI (if needed, on different ports)

## Protocol Reference

Full protocol specification: [GSPro Connect V1](https://gsprogolf.com/GSProConnectV1.html) (community calls this "Open API")

### Supported Features

- ✅ Ball speed, spin, and launch angles
- ✅ Shot number tracking
- ✅ Units detection (Yards/Meters)
- ✅ BackSpin/SideSpin (optional)
- ✅ Heartbeat messages (ignored)

### Unsupported Features

- ❌ Club data (not used for shot classification)
- ❌ Response codes (no response sent)
- ❌ Player information (not needed)

## Future Enhancements

Potential additions:
- Support for ClubData processing
- Shot history/statistics
- Multiple simultaneous protocols (e.g., TruGolf, E6)
- Protocol selection via command-line flags

## Troubleshooting

**Q: Launch monitor can't connect**
- Ensure Windows Firewall allows port 921
- Check the app is running and shows "Listening on 0.0.0.0:921" in status

**Q: Units seem wrong**
- OpenAPI protocol uses "Yards" to indicate imperial units
- Speed is converted from mph to m/s automatically
- If using metric launch monitor, set Units="Meters" in the message

**Q: No data appears in GUI**
- Check the conversion is working (console output shows received data)
- Ensure BallData contains all required fields
- Verify the JSON is properly formatted

## Summary

The Open API integration allows the Windows demo to work seamlessly with any OpenAPI-compatible launch monitor, providing real-time shot analysis and classification without needing the full OpenAPI software.
