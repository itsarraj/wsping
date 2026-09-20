use std::time::{Duration, Instant};

use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use wsping::stats::compute_stats;

#[derive(Parser)]
#[command(
    name = "wsping",
    about = "Tests a WebSocket endpoint's handshake and measures real ping/pong round-trip latency"
)]
struct Cli {
    /// A ws:// or wss:// URL.
    url: String,
    /// Number of pings to send.
    #[arg(short = 'n', long, default_value_t = 5)]
    count: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let connect_start = Instant::now();
    let (mut ws, response) = connect_async(&cli.url)
        .await
        .map_err(|e| anyhow::anyhow!("connecting to {}: {e}", cli.url))?;
    println!(
        "connected in {:?} (HTTP {})",
        connect_start.elapsed(),
        response.status()
    );

    let mut samples = Vec::with_capacity(cli.count);
    for i in 0..cli.count {
        let payload = format!("wsping-{i}").into_bytes();
        let sent_at = Instant::now();
        ws.send(Message::Ping(payload.clone())).await?;

        let rtt = loop {
            match ws.next().await {
                Some(Ok(Message::Pong(data))) if data == payload => break sent_at.elapsed(),
                Some(Ok(_)) => continue, // some other frame arrived first — keep waiting for our pong
                Some(Err(e)) => anyhow::bail!("connection error while waiting for pong: {e}"),
                None => anyhow::bail!("connection closed before a pong was received"),
            }
        };
        println!("seq={i} time={rtt:?}");
        samples.push(rtt);
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    if let Some(stats) = compute_stats(&samples) {
        println!(
            "\n{} pings: min={:?} avg={:?} max={:?}",
            stats.count, stats.min, stats.avg, stats.max
        );
    }

    Ok(())
}
