# wsping

Tests a WebSocket endpoint's real handshake and measures real ping/pong
round-trip latency over N pings — a `websocat`-with-timing alternative.
`websocat` (Rust, already excellent) can send raw frames but has no
built-in latency measurement mode; `wscat` (Node) is interactive only.
This is the narrow "just tell me the RTT" tool for either.

## Usage

```bash
wsping ws://localhost:8080/socket
wsping wss://example.com/ws -n 20
```

Reports the real HTTP status of the upgrade response, then each ping's
individual round-trip time, then a min/avg/max summary.

## How it measures

Each ping sends a real WebSocket `Ping` frame with a unique payload
(`wsping-<seq>`) and waits for the matching `Pong` — matched by payload,
not just "the next frame that arrives," so an unrelated data frame
arriving in between doesn't get mistaken for the pong and doesn't
corrupt the timing. 200ms between pings, matching plain ICMP `ping`'s
own default cadence.

## Status: built and verified against a real local WebSocket server, real handshake, real ping/pong frames

- **4 unit tests** (`cargo test --lib`): the pure min/avg/max statistics
  function — empty input, a single sample (min/avg/max all equal),
  multiple samples with a correct average, and that sample order
  doesn't affect the result.
- **Live-verified against a real local WebSocket server**: built a
  second, independent real WebSocket server (using `tokio-tungstenite`'s
  own accept-side API) that does nothing but accept connections and read
  frames — `tokio-tungstenite` handles Ping→Pong automatically on both
  sides per the WebSocket spec, so this genuinely exercises the real
  protocol-level ping/pong exchange, not an application-level echo the
  server had to implement by hand. The actual compiled binary connected
  to this real server, completed a real HTTP/1.1 Upgrade handshake
  (reported `HTTP 101 Switching Protocols`), sent 3 real pings, and
  received all 3 real pongs back with genuine sub-millisecond
  round-trip times on localhost. Separately confirmed a real
  connection-refused case (nothing listening on the target port) fails
  cleanly with a clear error instead of hanging.

**Not done / deliberately deferred**: TLS certificate validation options
(no `--insecure`/custom CA flag — `wss://` uses whatever the underlying
TLS stack's default trust store is); no continuous/streaming mode
(`ping -t`'s "run forever" equivalent) — this always sends a fixed count
and exits; and message-level (as opposed to protocol-level ping/pong)
round-trip timing isn't measured, only real WebSocket Ping/Pong control
frames.
