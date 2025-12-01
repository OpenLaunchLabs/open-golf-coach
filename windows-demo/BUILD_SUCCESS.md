# Build Complete!

## Windows Executable Ready

Your portable Windows demo application has been successfully built!

### Location

```
target/x86_64-pc-windows-gnu/release/opengolfcoach-windows-demo.exe
```

**Size**: 4.3 MB

### What It Does

1. **Opens a GUI window** with shot monitoring interface
2. **Listens on TWO TCP ports**:
   - Port 10000: OpenGolfCoach native JSON format
   - Port 921: Open API protocol
3. **Processes** the shot data using OpenGolfCoach
4. **Displays** `shot_name` and `shot_rank` in large, color-coded text
5. **Waits** for the next shot (continuously running on both ports)

### Next Steps

#### To Deploy

1. Copy the `.exe` file to a USB drive or cloud storage
2. Transfer to your Windows machine
3. Double-click to run - no installation needed!

#### To Test

On Windows, run the application, then use the test clients:

```bash
# Test OpenGolfCoach format (port 10000)
python test_client.py

# Test OpenAPI format (port 921)
python test_openapi_client.py
```

Or send data manually (OpenGolfCoach format):

```python
import socket, json
s = socket.socket()
s.connect(('127.0.0.1', 10000))
shot = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "horizontal_launch_angle_degrees": -2.0,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 6.0
}
s.sendall((json.dumps(shot) + '\n').encode())
print(s.recv(4096).decode())
s.close()
```

### Features

- ✅ **Portable** - Single executable, no dependencies
- ✅ **Dual TCP Servers**:
  - Port 10000: OpenGolfCoach native format
  - Port 921: Open API protocol
- ✅ **Real-time GUI** - Updates instantly when data received
- ✅ **Color-coded** - Shot name and rank displayed in rank-specific colors (blue for S-rank, green for A/B, yellow/orange for C, red for D/E)
- ✅ **Multi-Protocol** - Works with any OpenAPI-compatible launch monitor
- ✅ **No console window** - Clean GUI-only experience on Windows
- ✅ **Optimized** - Stripped binary, ~4MB total size

### Rebuilding

From Mac:
```bash
./build-windows.sh
```

From Windows:
```cmd
build-windows.bat
```

---

See [README.md](README.md) for full documentation and [QUICKSTART.md](QUICKSTART.md) for quick start guide.
