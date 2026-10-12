//! End-to-end integration tests for `podcli list`.
//!
//! These spin up a tiny local HTTP server that serves a minimal RSS 2.0 feed
//! and drive the real `podcli` binary against it via `CARGO_BIN_EXE_podcli`.
//!
//! The point of these tests (regression for a real bug) is that `--json`
//! output on **stdout** must be valid JSON and free of diagnostic pollution,
//! even when `RUST_LOG=debug` is set.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::thread;

/// A minimal RSS 2.0 document with three `<item>`s, each carrying a title,
/// description, enclosure, link and duration.
const RSS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Test Feed</title>
    <description>A tiny test feed</description>
    <link>http://example.com/</link>
    <item>
      <title>Episode One</title>
      <description>First episode</description>
      <enclosure url="http://example.com/1.mp3" length="1" type="audio/mpeg"/>
      <link>http://example.com/1</link>
      <duration>10:00</duration>
    </item>
    <item>
      <title>Episode Two</title>
      <description>Second episode</description>
      <enclosure url="http://example.com/2.mp3" length="1" type="audio/mpeg"/>
      <link>http://example.com/2</link>
      <duration>20:00</duration>
    </item>
    <item>
      <title>Episode Three</title>
      <description>Third episode</description>
      <enclosure url="http://example.com/3.mp3" length="1" type="audio/mpeg"/>
      <link>http://example.com/3</link>
      <duration>30:00</duration>
    </item>
  </channel>
</rss>
"#;

/// Bind an ephemeral local port, serve [`RSS`] on it from a detached background
/// thread and return the URL to reach it.
///
/// `Connection: close` is used on every response so the client never keeps a
/// connection alive; the loop serves several sequential connections and then
/// returns, so accepting cannot hang the test suite.
fn spawn_rss_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().expect("local_addr").port();

    thread::spawn(move || {
        let body = RSS.as_bytes();
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/rss+xml\r\n\
             Content-Length: {len}\r\n\
             Connection: close\r\n\
             \r\n",
            len = body.len()
        );

        // Serve a handful of connections so a stray keep-alive request cannot
        // block the client, then let the thread end.
        for stream in listener.incoming().take(8) {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            // Drain the request head (we don't care about its contents).
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);

            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(body);
            let _ = stream.flush();
        }
    });

    format!("http://127.0.0.1:{port}/feed.xml")
}

/// Run the binary with the given args against the local feed.
fn run_podcli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_podcli"))
        .args(args)
        .env("RUST_LOG", "debug")
        .output()
        .expect("failed to run podcli binary")
}

/// Test A: `--json` stdout is a single, clean, parseable JSON array even with
/// `RUST_LOG=debug` set. This fails today because the tracing subscriber writes
/// DEBUG lines to stdout.
#[test]
fn json_stdout_is_clean_parseable_array() {
    let url = spawn_rss_server();
    let output = run_podcli(&["list", "--url", &url, "--json"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "--json stdout must be valid JSON: {e}\n--- stdout ---\n{stdout}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    assert!(
        value.is_array(),
        "top-level --json value must be an array, got: {stdout}"
    );
    assert!(
        !stdout.contains("DEBUG"),
        "stdout must not contain debug logs:\n{stdout}"
    );
    assert!(
        !stdout.contains("====="),
        "stdout must not contain human-readable headers:\n{stdout}"
    );
}

/// Test B: `--json --first 2` yields an array of exactly two episodes.
#[test]
fn json_with_first_is_filtered_array() {
    let url = spawn_rss_server();
    let output = run_podcli(&["list", "--url", &url, "--json", "--first", "2"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("--json stdout must be valid JSON: {e}\n--- stdout ---\n{stdout}")
    });
    let array = value
        .as_array()
        .expect("top-level --json value must be an array");
    assert_eq!(array.len(), 2, "--json --first 2 must yield two episodes");
}

/// Test C: the human-readable (non-JSON) listing keeps the `===== <id> =====`
/// headers on stdout, and stdout is not polluted by DEBUG logs.
#[test]
fn human_readable_listing_keeps_headers_and_no_debug() {
    let url = spawn_rss_server();
    let output = run_podcli(&["list", "--url", &url]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("====="),
        "human-readable output must contain ===== headers:\n{stdout}"
    );
    assert!(
        !stdout.contains("DEBUG"),
        "stdout must not contain debug logs:\n{stdout}"
    );
}
