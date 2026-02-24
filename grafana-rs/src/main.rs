mod grafana;

use grafana::{GrafanaClient, CreateDatasourceRequest};
use clap::{Parser, Subcommand};

#[derive(Parser)]   //This will tell a type to be like a command which is below here that is grafana-rs
#[command(name = "grafana-rs")] //When we put this name as command because it has been derived above
#[command(about = "A Rust CLI for the Grafana API")] //This will generate a dicumentation if we give command-name --help
struct Cli {
    #[arg(short, long, default_value = "http://localhost:3000")]
    url: String,
    #[arg(short, long, env = "GRAFANA_API_KEY")]
    key: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List datasources
    Datasources,
    //Check if grafana ias alive
    Health,
    Org,
    CreateDatasource {
        /// Datasource name
        #[arg(short, long)]
        name: String,
        /// Datasource type (e.g. prometheus, postgres, influxdb)
        #[arg(short = 't', long = "type")]
        kind: String,
        /// Datasource URL (e.g. http://localhost:9090)
        #[arg(long)]
        ds_url: String,
    },
    GetDatasource {
        /// Datasource name to look up
        #[arg(short, long)]
        name: String,
    },

    DeleteDatasource {
        /// Datasource name to delete
        #[arg(short, long)]
        name: String,
    },

}

/*
#[tokio::main]
async fn main() {
    // Read from environment — never hardcode secrets
    let base_url = std::env::var("GRAFANA_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let api_key = match std::env::var("GRAFANA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: GRAFANA_API_KEY env var not set");
            eprintln!("Usage: GRAFANA_API_KEY=your-key cargo run");
            return;
        }
    };

    // Connect
    let client = GrafanaClient::connect(&base_url, &api_key);
    println!("Connecting to {base_url}...\n");

    // Health check
    match client.health().await {
        Ok(health) => {
            println!("[health]");
            println!("  version  : {}", health.version);
            println!("  database : {}", health.database);
            println!("  commit   : {}", health.commit);
        }
        Err(e) => {
            eprintln!("Health check failed: {e}");
            return;
        }
    }

    // Current org (proves API key auth works)
    match client.get_current_org().await {
        Ok(org) => {
            println!("\n[current org]");
            println!("  id       : {}", org.id);
            println!("  name     : {}", org.name);
        }
        Err(e) => {
            eprintln!("\nAuth failed: {e}");
            eprintln!("Check your API key!");
            return;
        }
    }
    match client.list_datasources().await {
        Ok(datasources) =>{
            println!("\n[datasources]");
            for ds in datasources {
                println!("  - {} ({})", ds.name, ds.kind);
            }
        }
        Err(e) =>{
            eprintln!("\nFailed to list datasources: {e}");
        }
    }

    println!("grafana-rs connected successfully!");
}
*/
#[tokio::main]
async fn main(){
    let cli = Cli::parse();
    let client = GrafanaClient::connect(&cli.url, &cli.key);
    match cli.command {
        Commands::Health => {
            match client.health().await {
                Ok(health) => {
                    println!("[health]");
                    println!("  version  : {}", health.version);
                    println!("  database : {}", health.database);
                    println!("  commit   : {}", health.commit);
                }
                Err(e) => eprintln!("Health check failed: {e}"),
            }
        }
        Commands::Org => {
            match client.get_current_org().await {
                Ok(org) => {
                    println!("[org]");
                    println!("  id       : {}", org.id);
                    println!("  name     : {}", org.name);
                }
                Err(e) => eprintln!("Failed to get org: {e}"),
            }
        }
        Commands::Datasources => {
            match client.list_datasources().await {
                Ok(datasources) => {
                    println!("[datasources] ({})", datasources.len());
                    for ds in &datasources {
                        println!(
                            "  - {} | type={} | url={} | default={}",
                            ds.name, ds.kind, ds.url, ds.is_default
                        );
                    }
                }
                Err(e) => eprintln!("Failed to list datasources: {e}"),
            }
        }
        Commands::CreateDatasource { name, kind, ds_url } => {
            let request = CreateDatasourceRequest {
                name: name.clone(),
                kind,
                access: "proxy".to_string(),
                url: ds_url,
                database: None,
                user: None,
                is_default: false,
            };

            match client.create_datasource(&request).await {
                Ok(resp) => {
                    println!("[created]");
                    println!("  id       : {}", resp.id);
                    println!("  name     : {}", resp.name);
                    println!("  message  : {}", resp.message);
                }
                Err(e) => eprintln!("Failed to create datasource: {e}"),
            }
        }
        Commands::GetDatasource { name } => {
            match client.get_datasource_by_name(&name).await {
                Ok(ds) => {
                    println!("[datasource]");
                    println!("  id       : {}", ds.id);
                    println!("  uid      : {}", ds.uid);
                    println!("  name     : {}", ds.name);
                    println!("  type     : {}", ds.kind);
                    println!("  url      : {}", ds.url);
                    println!("  default  : {}", ds.is_default);
                }
                Err(e) => eprintln!("Failed to get datasource: {e}"),
            }
        }
        Commands::DeleteDatasource { name } => {
            match client.delete_datasource(&name).await {
                Ok(resp) => {
                    println!("[deleted]");
                    println!("  name     : {}", name);
                    println!("  response : {}", resp);
                }
                Err(e) => eprintln!("Failed to delete datasource: {e}"),
            }
        }

    }
}