use crate::markdown;
use std::path::Path;

/// Lint result for a single node file.
struct LintResult {
    file: String,
    errors: Vec<String>,
}

/// Validate ontology nodes against SPEC.md rules.
///
/// Checks:
/// - Title (# Term) is required
/// - Ontology section is required
/// - Axiology section is required
/// - Epistemology section is required
/// - Broken links: references to `./term.md` where `src/term.md` doesn't exist
pub fn run(ontology_dir: &Path, path: Option<&str>) -> Result<(), String> {
    let src_dir = match path {
        Some(p) => {
            let p = Path::new(p);
            if p.is_dir() {
                p.to_path_buf()
            } else {
                // Lint a single file
                return lint_single_file(p, ontology_dir);
            }
        }
        None => ontology_dir.join("src"),
    };

    if !src_dir.is_dir() {
        return Err(format!("Source directory {} not found", src_dir.display()));
    }

    let existing_terms = markdown::list_terms(&src_dir)?;
    let mut results = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(&src_dir)
        .map_err(|e| format!("Cannot read {}: {e}", src_dir.display()))?
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext == "md")
        })
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
        let filename = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown")
            .to_string();

        let errors = lint_content(&content, &filename, &existing_terms);
        if !errors.is_empty() {
            results.push(LintResult {
                file: filename,
                errors,
            });
        }
    }

    if results.is_empty() {
        println!("All nodes pass lint checks.");
        Ok(())
    } else {
        let mut total_errors = 0;
        for result in &results {
            println!("{}:", result.file);
            for err in &result.errors {
                println!("  - {err}");
                total_errors += 1;
            }
            println!();
        }
        println!("{total_errors} error(s) in {} file(s).", results.len());
        // Return Err so the process exits with code 1
        Err(format!("{total_errors} lint error(s) found"))
    }
}

fn lint_single_file(path: &Path, ontology_dir: &Path) -> Result<(), String> {
    let src_dir = ontology_dir.join("src");
    let existing_terms = if src_dir.is_dir() {
        markdown::list_terms(&src_dir)?
    } else {
        Vec::new()
    };

    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown")
        .to_string();

    let errors = lint_content(&content, &filename, &existing_terms);
    if errors.is_empty() {
        println!("{filename}: OK");
        Ok(())
    } else {
        println!("{filename}:");
        for err in &errors {
            println!("  - {err}");
        }
        Err(format!("{} lint error(s) found", errors.len()))
    }
}

fn lint_content(content: &str, filename: &str, existing_terms: &[String]) -> Vec<String> {
    let mut errors = Vec::new();

    // Check title
    let has_title = content
        .lines()
        .any(|l| l.trim().starts_with("# ") && !l.trim().starts_with("## "));
    if !has_title {
        errors.push("Missing title (# Term)".to_string());
    }

    // Check required sections
    let has_section = |name: &str| -> bool {
        content.lines().any(|l| {
            let t = l.trim();
            t.starts_with("## ") && !t.starts_with("### ") && t.contains(name)
        })
    };

    if !has_section("Ontology") {
        errors.push("Missing required section: ## [Ontology]".to_string());
    }
    if !has_section("Axiology") {
        errors.push("Missing required section: ## [Axiology]".to_string());
    }
    if !has_section("Epistemology") {
        errors.push("Missing required section: ## [Epistemology]".to_string());
    }

    // Check broken links
    let links = markdown::extract_unique_links(content);
    for link in links {
        if !existing_terms.contains(&link) {
            errors.push(format!(
                "Broken link: [{link}](./{link}.md) — file src/{link}.md not found (referenced in {filename})"
            ));
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_valid_content() {
        let content = r#"# Test

## [Ontology](./ontology.md)

Definition here.

## [Axiology](./axiology.md)

Value here.

## [Epistemology](./epistemology.md)

Knowledge here.
"#;
        let errors = lint_content(content, "test.md", &[]);
        // No structural errors (broken links are expected since no terms exist)
        let structural: Vec<_> = errors
            .iter()
            .filter(|e| !e.starts_with("Broken link"))
            .collect();
        assert!(structural.is_empty());
    }

    #[test]
    fn test_lint_missing_sections() {
        let content = "# Test\n\nSome content.\n";
        let errors = lint_content(content, "test.md", &[]);
        assert!(errors.iter().any(|e| e.contains("Ontology")));
        assert!(errors.iter().any(|e| e.contains("Axiology")));
        assert!(errors.iter().any(|e| e.contains("Epistemology")));
    }

    #[test]
    fn test_lint_missing_title() {
        let content = "## [Ontology](./ontology.md)\n## [Axiology](./axiology.md)\n## [Epistemology](./epistemology.md)\n";
        let errors = lint_content(content, "test.md", &[]);
        assert!(errors.iter().any(|e| e.contains("Missing title")));
    }

    #[test]
    fn test_lint_broken_links() {
        let content = r#"# Test

## [Ontology](./ontology.md)

Links to [foo](./foo.md) and [bar](./bar.md).

## [Axiology](./axiology.md)

Value.

## [Epistemology](./epistemology.md)

Knowledge.
"#;
        let existing = vec!["foo".to_string()];
        let errors = lint_content(content, "test.md", &existing);
        // "bar" and "ontology" are broken, "foo" is OK
        assert!(errors.iter().any(|e| e.contains("bar")));
        assert!(!errors.iter().any(|e| e.contains("[foo]")));
    }
}
