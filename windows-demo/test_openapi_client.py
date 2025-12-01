#!/usr/bin/env python3
"""
Test client for OpenAPI Connect V1 Protocol

Sends sample shot data in OpenAPI format to test the protocol integration.
"""

import socket
import json
import sys
import time

SERVER_HOST = '10.11.27.95'
OPENAPI_PORT = 921

# Sample OpenAPI Connect V1 shots to test
OPENAPI_SAMPLE_SHOTS = [
    {
        "name": "Pull Fade (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 1,
            "APIversion": "1",
            "BallData": {
                "Speed": 156.5,  # mph
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
    },
    {
        "name": "Straight Shot (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 2,
            "APIversion": "1",
            "BallData": {
                "Speed": 167.7,  # mph (~75 m/s)
                "SpinAxis": 0.0,
                "TotalSpin": 2500.0,
                "HLA": 0.0,
                "VLA": 11.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Push Draw (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 3,
            "APIversion": "1",
            "BallData": {
                "Speed": 152.0,  # mph (~68 m/s)
                "SpinAxis": -15.0,
                "TotalSpin": 3000.0,
                "HLA": 3.0,
                "VLA": 13.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Baby Push Draw (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 4,
            "APIversion": "1",
            "BallData": {
                "Speed": 145.4,  # mph (~65 m/s)
                "SpinAxis": -3.0,
                "TotalSpin": 2600.0,
                "HLA": 1.5,
                "VLA": 12.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Baby Pull Draw (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 5,
            "APIversion": "1",
            "BallData": {
                "Speed": 149.1,  # mph (~66.7 m/s)
                "SpinAxis": -3.0,
                "TotalSpin": 2700.0,
                "HLA": -1.0,
                "VLA": 11.5
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Pull Slice (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 6,
            "APIversion": "1",
            "BallData": {
                "Speed": 154.3,  # mph (~69 m/s)
                "SpinAxis": 25.0,
                "TotalSpin": 3500.0,
                "HLA": -4.0,
                "VLA": 10.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Right Shank (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 7,
            "APIversion": "1",
            "BallData": {
                "Speed": 112.5,  # mph (~50 m/s)
                "SpinAxis": 35.0,
                "TotalSpin": 4500.0,
                "HLA": 20.0,
                "VLA": 5.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
    {
        "name": "Bladed (OpenAPI Format)",
        "data": {
            "DeviceID": "OpenGolfCoach-Test",
            "Units": "Yards",
            "ShotNumber": 8,
            "APIversion": "1",
            "BallData": {
                "Speed": 161.6,  # mph (~72.2 m/s)
                "SpinAxis": 0.0,
                "TotalSpin": 800.0,
                "HLA": 0.0,
                "VLA": 3.0
            },
            "ShotDataOptions": {
                "ContainsBallData": True,
                "ContainsClubData": False
            }
        }
    },
]


def send_openapi_shot(shot_data, shot_name="Unknown"):
    """Send a OpenAPI shot to the server. No response expected."""
    try:
        # Create socket
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5.0)

        # Connect
        print(f"\nConnecting to {SERVER_HOST}:{OPENAPI_PORT} (OpenAPI Connect V1)...")
        sock.connect((SERVER_HOST, OPENAPI_PORT))
        print("Connected!")

        # Try to receive handshake (OpenGolfCoach demo sends one, GSPro doesn't)
        # Set non-blocking to check if handshake is available
        sock.setblocking(False)
        try:
            handshake = sock.recv(1024, socket.MSG_DONTWAIT).decode('utf-8')
            print(f"Received handshake: {handshake.strip()}")
        except (BlockingIOError, socket.error):
            # No handshake received - this is normal for GSPro
            print("No handshake received (normal for GSPro)")
        sock.setblocking(True)

        # Send data
        json_data = json.dumps(shot_data)
        print(f"\nSending shot: {shot_name}")
        print(f"Data: {json_data}")

        # Send with newline - OpenAPI standard
        sock.sendall((json_data + '\n').encode('utf-8'))

        print("\n" + "="*60)
        print("SHOT SENT TO OPENAPI SERVER")
        print("="*60)
        print("✓ OpenAPI protocol: No response expected (one-way communication)")
        print("✓ Check the Windows Demo GUI - it should show the updated shot!")
        print("="*60)

        sock.close()
        return True

    except socket.timeout:
        print("\n✗ Connection timeout - is the server running?")
        return False
    except ConnectionRefusedError:
        print(f"\n✗ Connection refused - is the Windows Demo app running?")
        print(f"   Make sure the app is started and listening on port {OPENAPI_PORT}")
        return False
    except Exception as e:
        print(f"\n✗ Error: {e}")
        return False


def main():
    print("OpenGolfCoach Windows Demo - OpenAPI Connect V1 Test Client")
    print("="*60)

    if len(sys.argv) > 1 and sys.argv[1] == "--loop":
        # Continuous mode - send shots repeatedly using persistent connection
        print("\nContinuous mode - persistent connection, sending shots every 3 seconds")
        print("Press Ctrl+C to stop\n")

        try:
            # Create persistent connection
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(30.0)  # Longer timeout for persistent connection
            print(f"Connecting to {SERVER_HOST}:{OPENAPI_PORT} (OpenAPI Connect V1)...")
            sock.connect((SERVER_HOST, OPENAPI_PORT))

            # Try to receive handshake (OpenGolfCoach demo sends one, GSPro doesn't)
            sock.setblocking(False)
            try:
                handshake = sock.recv(1024, socket.MSG_DONTWAIT).decode('utf-8')
                print(f"Received handshake: {handshake.strip()}")
            except (BlockingIOError, socket.error):
                print("No handshake received (normal for GSPro)")
            sock.setblocking(True)
            sock.settimeout(30.0)  # Reset timeout after handshake check
            print("Connected! Keeping connection alive...\n")

            shot_index = 0
            shot_number = 1
            while True:
                shot = OPENAPI_SAMPLE_SHOTS[shot_index % len(OPENAPI_SAMPLE_SHOTS)]
                shot_data = shot["data"].copy()
                shot_data["ShotNumber"] = shot_number

                json_data = json.dumps(shot_data)
                print(f"Sending shot {shot_number}: {shot['name']}")

                # Send with newline delimiter for persistent connection
                sock.sendall((json_data + '\n').encode('utf-8'))

                print(f"  → Sent (OpenAPI protocol: no response expected)")

                shot_index += 1
                shot_number += 1
                print("  Waiting 3 seconds...")
                time.sleep(3)

        except KeyboardInterrupt:
            print("\n\nStopping...")
            if 'sock' in locals():
                sock.close()
            print("Connection closed. Stopped by user")
        except Exception as e:
            print(f"\n✗ Error: {e}")
            if 'sock' in locals():
                sock.close()
    else:
        # Single mode - ask which shot to send
        print("\nAvailable OpenAPI test shots:")
        for i, shot in enumerate(OPENAPI_SAMPLE_SHOTS, 1):
            print(f"  {i}. {shot['name']}")
        print(f"  {len(OPENAPI_SAMPLE_SHOTS) + 1}. Send all shots")
        print(f"  {len(OPENAPI_SAMPLE_SHOTS) + 2}. Continuous loop (--loop)")

        choice = input(f"\nSelect shot (1-{len(OPENAPI_SAMPLE_SHOTS) + 2}): ").strip()

        try:
            choice_num = int(choice)
            if 1 <= choice_num <= len(OPENAPI_SAMPLE_SHOTS):
                # Send single shot
                shot = OPENAPI_SAMPLE_SHOTS[choice_num - 1]
                send_openapi_shot(shot["data"], shot["name"])
            elif choice_num == len(OPENAPI_SAMPLE_SHOTS) + 1:
                # Send all shots
                shot_number = 1
                for shot in OPENAPI_SAMPLE_SHOTS:
                    shot_data = shot["data"].copy()
                    shot_data["ShotNumber"] = shot_number
                    send_openapi_shot(shot_data, shot["name"])
                    shot_number += 1
                    time.sleep(1)
            elif choice_num == len(OPENAPI_SAMPLE_SHOTS) + 2:
                # Run in loop mode
                print("\nRestart with --loop flag: python test_openapi_client.py --loop")
            else:
                print("Invalid choice")
        except ValueError:
            print("Invalid input")


if __name__ == "__main__":
    main()
