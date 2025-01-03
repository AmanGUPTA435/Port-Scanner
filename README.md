# Port Scanner Tool

---

## Description

This is a Rust-based asynchronous port scanner tool designed to scan a single IP address or a range of IPs specified in CIDR format. It checks for open ports within a user-defined range using `tokio` for concurrency and efficient asynchronous operations.

---

## Features

- Scan a single IP address or a range of IPs using CIDR notation.
- Define a custom port range for scanning.
- Display open ports in real time.
- Asynchronous and efficient, leveraging Rust's `tokio` runtime.
- Handles multiple tasks concurrently.

---

## Dependencies

This program uses the following Rust crates:

- **tokio**: For asynchronous networking and runtime.
- **clap**: For parsing command-line arguments.
- **cidr**: To handle CIDR-based IP range parsing.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/AmanGUPTA435/Port-Scanner.git
   cd porter
   ```

## Usage

1. To scan a single IP address:

   ```bash
   cargo run -- 127.0.0.1
   ```

2. To scan a CIDR block:

   ```bash
   cargo run -- --cidr 192.168.1.0/24
   ```

3. To specify a custom port range:
   ```bash
   cargo run -- 127.0.0.1 -s 8000 -e 65535
   ```

## Output

1. ? <IP>:<PORT_RANGE>: Indicates the IP and port range being scanned.
2. = <IP>:<PORT>: Indicates an open port detected during the scan.
