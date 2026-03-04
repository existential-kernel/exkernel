use crate::config::Config;
use std::path::Path;

/// List terms at a given ring level, or all rings if no level specified.
pub fn run(ontology_dir: &Path, ring: Option<u32>) -> Result<(), String> {
    let config_path = ontology_dir.join("exkernel.toml");
    let config = Config::load(&config_path)?;

    match ring {
        Some(level) => {
            if let Some(r) = config.get_ring(level) {
                println!("Ring {level} — {} ({})", r.name, r.description);
                println!();
                for term in &r.terms {
                    println!("  {term}");
                }
            } else {
                return Err(format!("Ring {level} not defined in exkernel.toml"));
            }
        }
        None => {
            for (level, r) in config.rings_sorted() {
                println!("Ring {level} — {} ({})", r.name, r.description);
                for term in &r.terms {
                    println!("  {term}");
                }
                println!();
            }
        }
    }

    Ok(())
}
