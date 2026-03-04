# existence

CLI for the [Existence](https://github.com/existence-lang/ontology) ontology framework.

Navigate, validate, and visualize ontology term definitions organized in scoping rings.

## Install

```bash
cargo install existence
```

Or build from source:

```bash
git clone https://github.com/existence-lang/existence
cd existence
cargo install --path .
```

## Usage

All commands auto-detect the ontology directory. If you're inside an ontology repo (one with `existence.toml`), it just works. Otherwise, use `--ontology <path>` or run `existence fetch` first.

### Lookup a term

```bash
# Print the full markdown definition
existence lookup existence

# Output as structured JSON (title, ontology, axiology, epistemology sections)
existence lookup existence --json
```

### Navigate scoping rings

```bash
# List all rings and their terms
existence scope

# List only Ring 0 (kernel) terms
existence scope 0

# List Ring 1 (software) terms
existence scope 1
```

### Lint ontology nodes

```bash
# Validate all nodes in src/
existence lint

# Validate a specific directory or file
existence lint src/existence.md
```

Checks:
- Title (`# Term`) is present
- Required sections: `## [Ontology]`, `## [Axiology]`, `## [Epistemology]`
- Broken links: `[term](./term.md)` references where `src/term.md` doesn't exist

Exit code 0 if clean, 1 if errors found.

### Generate relationship graph

```bash
# DOT format (pipe to graphviz)
existence graph | dot -Tsvg -o ontology.svg

# Filter to a specific ring
existence graph 0 | dot -Tpng -o kernel.png

# JSON adjacency list
existence graph --format json
```

### Fetch an ontology

```bash
# Clone from GitHub
existence fetch github:existence-lang/ontology

# Pull all sources defined in existence.toml
existence fetch
```

Sources are stored in `~/.existence/sources/{org}/{repo}/`.

## Configuration

Ontologies are configured via `existence.toml`:

```toml
[meta]
name = "existence-lang/ontology"
description = "Reference existential ontology"

[rings.0]
name = "kernel"
description = "14 universal terms, always loaded"
terms = ["existence", "entity", "abstraction", "scope", "context", ...]

[rings.1]
name = "software"
description = "The DDD bridge"
terms = ["project", "model", "algorithm", ...]

[sources]
upstream = "github:existence-lang/ontology"
```

## Commands (v0.3.0)

| Command | Description | Status |
|---------|-------------|--------|
| `lookup <term>` | Read a node's full definition | Implemented |
| `scope [ring]` | List terms at a ring level | Implemented |
| `lint [path]` | Validate nodes against SPEC.md rules | Implemented |
| `graph [ring]` | Generate term relationship graph (DOT/JSON) | Implemented |
| `fetch [source]` | Clone or pull ontology from GitHub | Implemented |
| `install` | Set up ~/.claude integration | Planned |
| `serve` | Start local API server | Planned |
| `build-site` | Generate static site + JSON API | Planned |
| `context <domain>` | Suggest relevant terms for a domain | Planned |

## License

Apache-2.0
