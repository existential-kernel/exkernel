mod commands;
mod config;
mod markdown;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "exkernel",
    version,
    about = "CLI for the Existential Kernel ontology framework"
)]
struct Cli {
    /// Path to the ontology directory (overrides auto-detection)
    #[arg(long, global = true)]
    ontology: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read a node's full definition
    Lookup {
        /// Term name (e.g. "existence", "entity")
        term: String,

        /// Output parsed sections as JSON
        #[arg(long)]
        json: bool,
    },

    /// Navigate the scoping chain — list terms at a ring level
    Scope {
        /// Ring level (0, 1, ...). Omit to list all rings.
        ring: Option<u32>,
    },

    /// Validate ontology nodes against SPEC.md rules
    Lint {
        /// Path to lint (directory or single file). Defaults to src/
        path: Option<String>,
    },

    /// Generate term relationship graph
    Graph {
        /// Filter to a specific ring level
        ring: Option<u32>,

        /// Output format: "dot" (default) or "json"
        #[arg(long, default_value = "dot")]
        format: String,
    },

    /// Clone or pull an ontology from a GitHub repo
    Fetch {
        /// Source in format github:org/repo. If omitted, reads exkernel.toml
        source: Option<String>,
    },

    /// Set up ~/.claude integration (not yet implemented)
    Install,

    /// Start local API server (not yet implemented)
    Serve,

    /// Generate static site + JSON API (not yet implemented)
    BuildSite,

    /// Suggest relevant terms for a domain (not yet implemented)
    Context {
        /// Domain name
        domain: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Lookup { ref term, json } => {
            let ontology_dir = resolve_or_exit(cli.ontology.as_deref());
            commands::lookup::run(&ontology_dir, term, json)
        }
        Commands::Scope { ring } => {
            let ontology_dir = resolve_or_exit(cli.ontology.as_deref());
            commands::scope::run(&ontology_dir, ring)
        }
        Commands::Lint { ref path } => {
            let ontology_dir = resolve_or_exit(cli.ontology.as_deref());
            commands::lint::run(&ontology_dir, path.as_deref())
        }
        Commands::Graph { ring, ref format } => {
            let ontology_dir = resolve_or_exit(cli.ontology.as_deref());
            commands::graph::run(&ontology_dir, ring, format)
        }
        Commands::Fetch { ref source } => {
            let ontology_dir = config::resolve_ontology_dir(cli.ontology.as_deref())
                .unwrap_or_else(|_| PathBuf::from("."));
            commands::fetch::run(&ontology_dir, source.as_deref())
        }
        Commands::Install => {
            println!("exkernel install: not yet implemented");
            println!("Will set up ~/.claude integration with ontology terms.");
            Ok(())
        }
        Commands::Serve => {
            println!("exkernel serve: not yet implemented");
            println!("Will start a local API server for ontology queries.");
            Ok(())
        }
        Commands::BuildSite => {
            println!("exkernel build-site: not yet implemented");
            println!("Will generate a static site + JSON API from the ontology.");
            Ok(())
        }
        Commands::Context { ref domain } => {
            println!("exkernel context: not yet implemented");
            println!("Will suggest relevant ontology terms for the '{domain}' domain.");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn resolve_or_exit(explicit: Option<&std::path::Path>) -> PathBuf {
    config::resolve_ontology_dir(explicit).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    })
}
