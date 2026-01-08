# Stratus ATC - Session Handoff

**Last Updated:** January 7, 2026

---

## Session Summary (January 7, 2026)

### Major Changes This Session

#### 1. IP Clarity: SimAPI → Telemetry Rename ✅

Complete renaming across entire codebase to avoid SayIntentions.ai naming confusion:

| Old Name | New Name |
|:---------|:---------|
| `simAPI_input.json` | `stratus_telemetry.json` |
| `simAPI_output.jsonl` | `stratus_commands.jsonl` |
| `~/.local/share/StratusAI/` | `~/.local/share/StratusATC/` |
| `SimApiWatcher` (Rust) | `TelemetryWatcher` |
| `simapi.rs` | `telemetry.rs` |
| `client/src/simapi/` | `client/src/telemetry/` |
| `StratusML` | `StratusATC` |

**Files Updated:**

- `stratus-rs/stratus-core/src/telemetry.rs`
- `stratus-rs/stratus-core/src/lib.rs`
- `stratus-rs/stratus-gui/src/app.rs`
- `adapters/xplane/src/stratus_plugin.c`
- `adapters/xplane/PI_Stratus.py`
- `client/src/core/sim_data.py`
- All documentation files

#### 2. Rust Migration: P0 Features ✅

Implemented critical performance features in Rust:

| Module | Purpose | Lines |
|:-------|:--------|------:|
| `streaming.rs` | Streaming Ollama for low-latency TTS | 235 |
| `warmup.rs` | Model warmup heartbeat service | 215 |

#### 3. Python Latency Features (Reference) ✅

Created Python reference implementations (to be ported to Rust):

| Module | Purpose |
|:-------|:--------|
| `streaming_llm.py` | Streaming Ollama client |
| `model_warmup.py` | Keep model hot |
| `readback_scoring.py` | Phraseology training feedback |
| `controller_personality.py` | 6 regional ATC styles |
| `response_cache.py` | Speculative pre-generation |
| `ambient_chatter.py` | Radio realism |
| `context_builder.py` | Rich prompt context |

#### 4. Documentation ✅

- Created `docs/PRIVACY.md` - Privacy-first marketing
- Created `docs/ATC_REFERENCE.md` - FAA 7110.65 excerpts
- Created `docs/PHRASEOLOGY_GUIDE.md` - 30 canonical exchanges
- Created `docs/REFERENCE_SOURCES.md` - Version-pinned sources
- Updated `BACKLOG.md` with Latency Advantage Sprint tickets

---

## Architecture: Rust-First Policy

> **Critical**: New features MUST be implemented in Rust first.
> Python code is legacy/reference only.

### Rust Codebase (`stratus-rs/`)

```
stratus-rs/
├── Cargo.toml              # Workspace root
├── stratus-core/           # Core library
│   └── src/
│       ├── lib.rs          # Public exports
│       ├── telemetry.rs    # X-Plane JSON watcher
│       ├── ollama.rs       # Sync Ollama client
│       ├── streaming.rs    # Streaming Ollama (NEW)
│       ├── warmup.rs       # Model warmup (NEW)
│       └── atc.rs          # Prompt builder
└── stratus-gui/            # Iced GUI
    └── src/
        ├── main.rs         # Entry point
        ├── app.rs          # Iced application
        ├── theme.rs        # Dark theme
        └── comlink.rs      # Axum web server
```

### Data Flow

```
X-Plane Plugin ─────► stratus_telemetry.json ─────► Rust Client
                                                         │
                                                         ▼
                                              StreamingOllama
                                                         │
                                                         ▼
                          stratus_commands.jsonl ◄───── TTS
```

---

## Regression Guards

### Naming Consistency Check

Run before every commit:

```bash
# Must return 0 matches (excluding tests/)
grep -ri "simapi\|SimApi\|SimAPI" --include="*.rs" --include="*.py" \
  --include="*.c" --include="*.md" . 2>/dev/null | \
  grep -v __pycache__ | grep -v target/ | grep -v tests/ | wc -l
```

### Rust Build Check

```bash
cd stratus-rs && cargo check --workspace
```

### Python Syntax Check

```bash
python3 -m py_compile client/src/core/*.py
```

---

## Key Files

| File | Purpose |
|:-----|:--------|
| `stratus-rs/stratus-core/src/telemetry.rs` | Telemetry file watcher (Rust) |
| `stratus-rs/stratus-core/src/streaming.rs` | Streaming LLM client (Rust) |
| `stratus-rs/stratus-core/src/warmup.rs` | Model warmup service (Rust) |
| `adapters/xplane/src/stratus_plugin.c` | X-Plane native plugin |
| `client/src/core/sim_data.py` | Legacy Python telemetry reader |

---

## Quick Start

### Run Rust Client

```bash
cd stratus-rs
cargo run
```

### Run Python Client (Legacy)

```bash
cd client
python3 -m src.main
```

### Build X-Plane Plugin

```bash
cd adapters/xplane
./setup_sdk.sh
mkdir build && cd build
cmake .. && make
```

---

## Next Steps

### P0: Critical Path to Rust MVP

1. [ ] Wire `StreamingOllama` into GUI app
2. [ ] Add D-Bus speech client for TTS/STT
3. [ ] End-to-end test with X-Plane

### P1: Port Remaining Python Features

1. [ ] `readback_scoring.py` → Rust
2. [ ] `controller_personality.py` → Rust
3. [ ] `context_builder.py` → Rust

---

## Prohibited Patterns

> These patterns indicate regression. Fix immediately if found.

| Pattern | Reason |
|:--------|:-------|
| `simAPI` | Old naming, must use `stratus_telemetry` |
| `StratusAI` | Old directory, must use `StratusATC` |
| `StratusML` | Old branding, must use `StratusATC` |
| New Python modules | Write in Rust first |
| Cloud API calls | Project is 100% offline |

---

## Git Commit

```bash
git add -A
git commit -m "Complete SimAPI→Telemetry rename + Rust streaming/warmup

- Renamed all simAPI references to stratus_telemetry
- Added streaming.rs for low-latency Ollama
- Added warmup.rs for model hot-keeping  
- Created Latency Advantage Sprint tickets
- Documented privacy-first architecture
- Updated all documentation for consistency"
```
