# exkernel

CLI tool for the Existential Kernel ontology framework. Single Rust binary using clap (derive API).

## Architecture

```
src/
  main.rs          — CLI entry point, clap arg parsing, command dispatch
  config.rs        — exkernel.toml parsing (serde + toml), ontology dir resolution
  markdown.rs      — Markdown node parsing, section extraction, link extraction
  commands/
    mod.rs         — Command module declarations
    lookup.rs      — Read and display a term definition (raw or JSON)
    scope.rs       — List terms by ring level from exkernel.toml
    lint.rs        — Validate nodes against SPEC.md rules
    graph.rs       — Generate DOT or JSON relationship graphs
    fetch.rs       — Clone/pull ontology repos via git
```

## Key patterns

- **Ontology resolution**: `config::resolve_ontology_dir()` checks (1) `--ontology` flag, (2) cwd for `exkernel.toml`, (3) `~/.exkernel/sources/`
- **Ring keys**: TOML table keys are strings (`[rings.0]`), stored as `BTreeMap<String, Ring>`, accessed via `Config::get_ring(u32)` and `Config::rings_sorted()`
- **Markdown parsing**: Section extraction matches `## [SectionName]` or `## SectionName` headings; link extraction uses regex for `[term](./term.md)` patterns
- **Error handling**: All commands return `Result<(), String>` — main prints errors to stderr and exits with code 1
- **Stub commands**: `install`, `serve`, `build-site`, `context` are defined in the CLI enum but print "not yet implemented" messages — no command files for these yet

## Development

```bash
cargo clippy -- -D warnings  # Lint
cargo test                    # Run tests
cargo fmt -- --check          # Format check
```

## Testing against the real ontology

```bash
# Point at the ontology repo
exkernel --ontology /path/to/existential-kernel/ontology lookup existence
exkernel --ontology /path/to/existential-kernel/ontology lint
exkernel --ontology /path/to/existential-kernel/ontology graph 0
```
