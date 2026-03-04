use crate::markdown::Node;
use std::path::Path;

/// Read a node's full definition from `src/{term}.md`.
///
/// If `--json` is set, output parsed sections as JSON.
/// Otherwise, print the raw markdown to stdout.
pub fn run(ontology_dir: &Path, term: &str, json: bool) -> Result<(), String> {
    let file = ontology_dir.join("src").join(format!("{term}.md"));
    if !file.exists() {
        return Err(format!("Term '{term}' not found at {}", file.display()));
    }

    let content = std::fs::read_to_string(&file)
        .map_err(|e| format!("Failed to read {}: {e}", file.display()))?;

    if json {
        let node = Node::parse(&content)?;
        let json_output = serde_json::to_string_pretty(&node)
            .map_err(|e| format!("JSON serialization error: {e}"))?;
        println!("{json_output}");
    } else {
        print!("{content}");
    }

    Ok(())
}
