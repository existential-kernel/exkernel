use crate::config::Config;
use crate::markdown;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
struct AdjacencyList {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

/// Generate a term relationship graph.
///
/// Parses all nodes, extracts `[term](./term.md)` links, and outputs
/// either DOT format or a JSON adjacency list.
pub fn run(ontology_dir: &Path, ring: Option<u32>, format: &str) -> Result<(), String> {
    let src_dir = ontology_dir.join("src");
    if !src_dir.is_dir() {
        return Err(format!("Source directory {} not found", src_dir.display()));
    }

    let existing_terms = markdown::list_terms(&src_dir)?;

    // If a ring filter is specified, load config and get ring terms
    let ring_filter: Option<Vec<String>> = if let Some(level) = ring {
        let config_path = ontology_dir.join("existence.toml");
        let config = Config::load(&config_path)?;
        let r = config
            .get_ring(level)
            .ok_or(format!("Ring {level} not defined in existence.toml"))?;
        Some(r.terms.clone())
    } else {
        None
    };

    // Build adjacency map
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut graph_nodes: Vec<String> = Vec::new();

    for term in &existing_terms {
        // Skip terms not in the ring filter
        if let Some(ref filter) = ring_filter
            && !filter.contains(term)
        {
            continue;
        }

        let file = src_dir.join(format!("{term}.md"));
        if !file.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&file)
            .map_err(|e| format!("Failed to read {}: {e}", file.display()))?;

        let links = markdown::extract_unique_links(&content);
        graph_nodes.push(term.clone());

        for link in links {
            // Only include edges to terms that exist
            if existing_terms.contains(&link) {
                // If ring-filtered, only include edges within the ring
                if let Some(ref filter) = ring_filter
                    && !filter.contains(&link)
                {
                    continue;
                }
                edges.push((term.clone(), link));
            }
        }
    }

    graph_nodes.sort();
    graph_nodes.dedup();
    edges.sort();
    edges.dedup();

    match format {
        "json" => {
            let adj = AdjacencyList {
                nodes: graph_nodes,
                edges,
            };
            let json = serde_json::to_string_pretty(&adj)
                .map_err(|e| format!("JSON serialization error: {e}"))?;
            println!("{json}");
        }
        _ => {
            println!("digraph ontology {{");
            println!("  rankdir=LR;");
            println!("  node [shape=box, style=rounded];");
            println!();
            for (from, to) in &edges {
                println!("  {from} -> {to};");
            }
            println!("}}");
        }
    }

    Ok(())
}
