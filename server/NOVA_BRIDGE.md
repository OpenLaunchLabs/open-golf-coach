# Nova → OpenGolfCoach Bridge

This utility discovers a Nova launch monitor on the local network, connects to the Nova OpenAPI TCP stream, maps each shot into the OpenGolfCoach JSON schema, runs `calculate_derived_values`, and can rebroadcast enriched shots locally.

## Run

```bash
# SSDP (default) + rebroadcast on 10000 for compatibility with existing listeners
cargo run -p opengolfcoach-server --bin nova_bridge -- --output-port=10000

# mDNS discovery
cargo run -p opengolfcoach-server --bin nova_bridge -- --discovery=mdns --output-port=10000

# Manual host/port (skip discovery)
cargo run -p opengolfcoach-server --bin nova_bridge -- --discovery=manual --nova-host=192.168.1.50 --nova-port=2921
```

## Behavior

- **Discovery:** SSDP by default (`urn:openlaunch:service:openapi:1`) with mDNS fallback (`_openapi-nova._tcp.local.`), or manual `--nova-host/--nova-port`.
- **Mapping:** Nova OpenAPI fields (`BallData.Speed`, `VLA`, `HLA`, `TotalSpin`, `SpinAxis`, optional `BackSpin/SideSpin`) are converted to OpenGolfCoach inputs. Imperial units are detected via Nova's `Units` field and carried through `us_customary_units`.
- **Processing:** Each shot is passed to `calculate_derived_values`; errors are logged but do not stop the stream.
- **Output:** Optional `--output-port` rebroadcasts enriched, line-delimited JSON so existing TCP clients on port 10000 keep working. The original `server` and `openapi` binaries that listen on ports 10000/921 remain unchanged.
- **Resilience:** Logs discovery/connection status and retries on disconnects with `--reconnect-delay-secs`.
