# exkernel

CLI for the [Existential Kernel](https://github.com/existential-kernel/ontology) ontology framework.

Navigate, validate, and visualize ontology term definitions organized in scoping rings.

## Install

```bash
cargo install exkernel
```

Or build from source:

```bash
git clone https://github.com/existential-kernel/exkernel
cd exkernel
cargo install --path .
```

## Usage

All commands auto-detect the ontology directory. If you're inside an ontology repo (one with `exkernel.toml`), it just works. Otherwise, use `--ontology <path>` or run `exkernel fetch` first.

### Lookup a term

```bash
# Print the full markdown definition
exkernel lookup existence

# Output as structured JSON (title, ontology, axiology, epistemology sections)
exkernel lookup existence --json
```

### Navigate scoping rings

```bash
# List all rings and their terms
exkernel scope

# List only Ring 0 (kernel) terms
exkernel scope 0

# List Ring 1 (software) terms
exkernel scope 1
```

### Lint ontology nodes

```bash
# Validate all nodes in src/
exkernel lint

# Validate a specific directory or file
exkernel lint src/existence.md
```

Checks:
- Title (`# Term`) is present
- Required sections: `## [Ontology]`, `## [Axiology]`, `## [Epistemology]`
- Broken links: `[term](./term.md)` references where `src/term.md` doesn't exist

Exit code 0 if clean, 1 if errors found.

### Generate relationship graph

```bash
# DOT format (pipe to graphviz)
exkernel graph | dot -Tsvg -o ontology.svg

# Filter to a specific ring
exkernel graph 0 | dot -Tpng -o kernel.png

# JSON adjacency list
exkernel graph --format json
```

### Fetch an ontology

```bash
# Clone from GitHub
exkernel fetch github:existential-kernel/ontology

# Pull all sources defined in exkernel.toml
exkernel fetch
```

Sources are stored in `~/.exkernel/sources/{org}/{repo}/`.

## Configuration

Ontologies are configured via `exkernel.toml`:

```toml
[meta]
name = "existential-kernel/ontology"
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
upstream = "github:existential-kernel/ontology"
```

## Commands (v0.1.0)

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
