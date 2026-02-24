# grafana-rs 🦀

A Rust SDK and CLI for the Grafana HTTP API. Built incrementally — one feature at a time.

## Prerequisites

- Rust (stable)
- Grafana instance running (tested with Grafana 8.4.5)
- Grafana API key ([how to create one](#getting-an-api-key))

## Project Structure

```
grafana-rs/
├── Cargo.toml         # Dependencies: reqwest, serde, tokio, clap
├── src/
│   ├── main.rs        # CLI entry point — parses args, calls grafana module
│   └── grafana.rs     # SDK module — client, models, error types
```

## Getting an API Key

1. Open Grafana at `http://localhost:3000`
2. Go to **Settings (gear icon) → API Keys → Add API key**
3. Set role to **Admin**
4. Copy the key — Grafana only shows it once

## Setup

```bash
# Clone the repo
git clone https://github.com/bapuoldtown/grafana-rs.git
cd grafana-rs

# Set your API key (Git Bash / Linux / Mac)
export GRAFANA_API_KEY="your-api-key-here"

# Build
cargo build --release
```

## Usage

```bash
# See all available commands
cargo run -- --help

# Health check — is Grafana alive?
cargo run -- health

# Show current organization
cargo run -- org

# List all datasources
cargo run -- datasources

# Get a specific datasource by name
cargo run -- get-datasource --name "Prometheus"

# Create a new datasource
cargo run -- create-datasource --name "My-Prometheus" --type prometheus --ds-url http://localhost:9090
cargo run -- create-datasource --name "My-Postgres" --type postgres --ds-url <any url>:<port>
cargo run -- create-datasource --name "My-InfluxDB" --type influxdb --ds-url http://localhost:8086
```

### Custom Grafana URL

```bash
cargo run -- --url http://other-grafana:3000 health
```

### Pass API Key Directly

```bash
cargo run -- --key "your-key" health
```

## CLI Reference

| Command | Description | Grafana API |
|---------|-------------|-------------|
| `health` | Check if Grafana is alive | `GET /api/health` |
| `org` | Show current organization | `GET /api/org` |
| `datasources` | List all datasources | `GET /api/datasources` |
| `get-datasource --name <NAME>` | Get datasource by name | `GET /api/datasources/name/:name` |
| `create-datasource --name <NAME> --type <TYPE> --ds-url <URL>` | Create a new datasource | `POST /api/datasources` |

## SDK Module (grafana.rs)

The `grafana.rs` module can be used independently as a library:

```rust
use grafana::GrafanaClient;

let client = GrafanaClient::connect("http://localhost:3000", "your-api-key");

// Health check
let health = client.health().await?;

// List datasources
let datasources = client.list_datasources().await?;

// Get datasource by name
let ds = client.get_datasource_by_name("Prometheus").await?;

// Create a datasource
let request = CreateDatasourceRequest {
    name: "New-Prom".to_string(),
    kind: "prometheus".to_string(),
    access: "proxy".to_string(),
    url: "http://localhost:9090".to_string(),
    database: None,
    user: None,
    is_default: false,
};
let result = client.create_datasource(&request).await?;
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `reqwest` | HTTP client for Grafana API calls |
| `serde` | JSON serialization/deserialization |
| `serde_json` | JSON value handling |
| `tokio` | Async runtime |
| `clap` | CLI argument parsing |

## Roadmap

- [x] API key authentication
- [x] Health check
- [x] Get current organization
- [x] List datasources
- [x] Get datasource by name
- [x] Create datasource (POST)
- [x] CLI with subcommands
- [ ] Delete datasource (DELETE)
- [ ] Search dashboards
- [ ] List folders
- [ ] Create dashboard
- [ ] Annotations
- [ ] Alerting

## Setting Environment Variables

**Git Bash (MINGW64):**
```bash
export GRAFANA_API_KEY="your-key"
cargo run -- health
```

**PowerShell:**
```powershell
$env:GRAFANA_API_KEY="your-key"
cargo run -- health
```

**CMD:**
```cmd
set GRAFANA_API_KEY=your-key
cargo run -- health
```

## License

MIT