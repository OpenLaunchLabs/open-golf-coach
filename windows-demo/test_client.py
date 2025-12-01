#!/usr/bin/env python3
"""
Test client for OpenGolfCoach Windows Demo

Sends sample shot data to the demo application and displays the response.
"""

import socket
import json
import sys
import time

SERVER_HOST = '10.11.27.95'
SERVER_PORT = 10000

# Sample shots to test
SAMPLE_SHOTS = [
    {
        "name": "Pull Fade",
        "data": {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.5,
            "horizontal_launch_angle_degrees": -2.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 6.0
        }
    },
    {
        "name": "Straight Shot",
        "data": {
            "ball_speed_meters_per_second": 75.0,
            "vertical_launch_angle_degrees": 11.0,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 2500.0,
            "spin_axis_degrees": 0.0
        }
    },
    {
        "name": "Push Draw",
        "data": {
            "ball_speed_meters_per_second": 68.0,
            "vertical_launch_angle_degrees": 13.0,
            "horizontal_launch_angle_degrees": 3.0,
            "total_spin_rpm": 3000.0,
            "spin_axis_degrees": -15.0
        }
    },
    {
        "name": "Baby Push Draw",
        "data": {
            "ball_speed_meters_per_second": 65.0,
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": 1.5,
            "total_spin_rpm": 2600.0,
            "spin_axis_degrees": -3.0
        }
    },
    {
        "name": "Baby Pull Draw",
        "data": {
            "ball_speed_meters_per_second": 66.0,
            "vertical_launch_angle_degrees": 13.0,
            "horizontal_launch_angle_degrees": -1.5,
            "total_spin_rpm": 2700.0,
            "spin_axis_degrees": -3.0
        }
    },
    {
        "name": "Pull Slice",
        "data": {
            "ball_speed_meters_per_second": 72.0,
            "vertical_launch_angle_degrees": 14.0,
            "horizontal_launch_angle_degrees": -3.5,
            "total_spin_rpm": 3500.0,
            "spin_axis_degrees": 25.0
        }
    },
    {
        "name": "Right Shank",
        "data": {
            "ball_speed_meters_per_second": 55.0,
            "vertical_launch_angle_degrees": 8.0,
            "horizontal_launch_angle_degrees": 15.0,
            "total_spin_rpm": 4000.0,
            "spin_axis_degrees": 30.0
        }
    },
    {
        "name": "Bladed",
        "data": {
            "ball_speed_meters_per_second": 80.0,
            "vertical_launch_angle_degrees": 3.0,
            "horizontal_launch_angle_degrees": 0.5,
            "total_spin_rpm": 1200.0,
            "spin_axis_degrees": 2.0
        }
    },
]


def send_shot(shot_data, shot_name="Unknown"):
    """Send a shot to the server and return the response."""
    try:
        # Create socket
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5.0)

        # Connect
        print(f"\nConnecting to {SERVER_HOST}:{SERVER_PORT}...")
        sock.connect((SERVER_HOST, SERVER_PORT))
        print("Connected!")

        # Send data
        json_data = json.dumps(shot_data)
        print(f"\nSending shot: {shot_name}")
        print(f"Data: {json_data}")

        # Note: Newline is optional - server can handle both:
        # - With newline (backwards compatible): json_data + '\n'
        # - Without newline (OpenAPI style): json_data
        # Using without newline to match OpenAPI behavior
        sock.sendall(json_data.encode('utf-8'))

        # Close write side to signal we're done sending
        sock.shutdown(socket.SHUT_WR)

        # Receive response
        response = sock.recv(4096).decode('utf-8')
        sock.close()

        # Parse and display
        result = json.loads(response)

        print("\n" + "="*60)
        print("RESPONSE FROM SERVER:")
        print("="*60)

        if "error" in result:
            print(f"ERROR: {result['error']}")
        elif "open_golf_coach" in result:
            ogc = result["open_golf_coach"]
            print(f"Shot Name:  {ogc.get('shot_name', 'N/A')}")
            print(f"Shot Rank:  {ogc.get('shot_rank', 'N/A')}")
            print(f"Carry Dist: {ogc.get('carry_distance_meters', 'N/A'):.1f}m")
            print(f"Offline:    {ogc.get('offline_distance_meters', 'N/A'):.1f}m")
        else:
            print(json.dumps(result, indent=2))

        print("="*60)
        print("\n✓ Check the Windows Demo GUI - it should show the updated shot!")

        return result

    except socket.timeout:
        print("\n✗ Connection timeout - is the server running?")
        return None
    except ConnectionRefusedError:
        print(f"\n✗ Connection refused - is the Windows Demo app running?")
        print(f"   Make sure the app is started and listening on port {SERVER_PORT}")
        return None
    except Exception as e:
        print(f"\n✗ Error: {e}")
        return None


def main():
    print("OpenGolfCoach Windows Demo - Test Client")
    print("="*60)

    if len(sys.argv) > 1 and sys.argv[1] == "--loop":
        # Continuous mode - send shots repeatedly using persistent connection
        print("\nContinuous mode - persistent connection, sending shots every 3 seconds")
        print("Press Ctrl+C to stop\n")

        try:
            # Create persistent connection
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(30.0)  # Longer timeout for persistent connection
            print(f"Connecting to {SERVER_HOST}:{SERVER_PORT}...")
            sock.connect((SERVER_HOST, SERVER_PORT))
            print("Connected! Keeping connection alive...\n")

            shot_index = 0
            while True:
                shot = SAMPLE_SHOTS[shot_index % len(SAMPLE_SHOTS)]
                json_data = json.dumps(shot["data"])

                print(f"Sending shot {shot_index + 1}: {shot['name']}")
                # Send with newline delimiter for persistent connection
                sock.sendall((json_data + '\n').encode('utf-8'))

                # Receive response
                response = sock.recv(4096).decode('utf-8')
                result = json.loads(response)

                if "open_golf_coach" in result:
                    ogc = result["open_golf_coach"]
                    print(f"  → {ogc.get('shot_name', 'N/A')}: {ogc.get('carry_distance_meters', 'N/A'):.1f}m carry")

                shot_index += 1
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
        print("\nAvailable test shots:")
        for i, shot in enumerate(SAMPLE_SHOTS, 1):
            print(f"  {i}. {shot['name']}")
        print(f"  {len(SAMPLE_SHOTS) + 1}. Send all shots")
        print(f"  {len(SAMPLE_SHOTS) + 2}. Continuous loop (--loop)")

        choice = input(f"\nSelect shot (1-{len(SAMPLE_SHOTS) + 2}): ").strip()

        try:
            choice_num = int(choice)
            if 1 <= choice_num <= len(SAMPLE_SHOTS):
                # Send single shot
                shot = SAMPLE_SHOTS[choice_num - 1]
                send_shot(shot["data"], shot["name"])
            elif choice_num == len(SAMPLE_SHOTS) + 1:
                # Send all shots
                for shot in SAMPLE_SHOTS:
                    send_shot(shot["data"], shot["name"])
                    time.sleep(1)
            elif choice_num == len(SAMPLE_SHOTS) + 2:
                # Run in loop mode
                print("\nRestart with --loop flag: python test_client.py --loop")
            else:
                print("Invalid choice")
        except ValueError:
            print("Invalid input")


if __name__ == "__main__":
    main()
