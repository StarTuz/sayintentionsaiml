# Stratus ATC - Team Onboarding

> **Welcome to the Stratus Advisory Team.** This document introduces the project's mission, architecture, codebase, history, and future vision.

---

## 1. The Mission

**Stratus is an open-source, fully offline AI Air Traffic Controller for flight simulators.**

We're building something that doesn't exist: a local-first, privacy-respecting ATC that runs entirely on your hardware. No cloud. No subscription. No internet required.

### Why This Matters

| Problem | Stratus Solution |
|---------|------------------|
| Cloud ATC services cost $15-30/month | **Free forever** (open source) |
| Voice data sent to remote servers | **Voice stays local** (Whisper + Piper) |
| Linux users ignored by commercial ATC | **Linux is our primary platform** |
| Simulator ATC is robotic and scripted | **AI generates natural, context-aware responses** |

### The Vision

> *"Talk to ATC like you're flying IRL—but on your computer, offline, for free."*

---

## 2. Origin Story

### How We Got Here

**2024 Q4**: The project began as an attempt to integrate SayIntentions.AI (a commercial cloud ATC) with X-Plane on Linux. We discovered:

1. The commercial service had latency issues and required constant internet
2. Linux support was lacking
3. Users were sending voice data to remote servers with no privacy guarantees

**Decision Point**: Instead of wrapping a cloud service, we pivoted to building a **fully local AI ATC**.

**2025 January**:

- ✅ Completed Phase 1: Core infrastructure (X-Plane plugin, Ollama integration, Qt6 GUI)
- ✅ Completed Phase 2: ATC logic with FAA phraseology and airport awareness
- 🔄 Cleaned codebase: Removed 800+ lines of dead cloud API code
- 🔄 Rebranded plugin: `SayIntentionsAIml` → `StratusATC`
- 🔄 Started Rust migration (`stratus-rs/`) for performance-critical components

**Today**: We have a working prototype that can:

- Read X-Plane telemetry in real-time
- Generate context-aware ATC responses using local LLMs
- Speak responses using local TTS
- Display communications in a modern Qt6 GUI
- Serve a mobile-friendly web interface (ComLink)

---

## 3. Architecture: "Brain vs Motor"

This is the most important architectural concept in Stratus.

```
┌──────────────────────────────────────────────────────────────────────┐
│                           BRAIN (Stratus)                             │
│                                                                       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐       │
│  │   ATC Logic     │  │  Prompt Engine  │  │   Sim Adapter   │       │
│  │  (Phraseology)  │  │ (Context Build) │  │  (X-Plane IPC)  │       │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘       │
│           │                    │                    │                 │
│           └────────────────────┼────────────────────┘                 │
│                                │                                      │
│                        ┌───────▼───────┐                              │
│                        │   D-Bus Call  │                              │
│                        │   Think(ctx)  │                              │
│                        └───────┬───────┘                              │
└────────────────────────────────┼──────────────────────────────────────┘
                                 │
                    ═════════════╪═════════════ (STRICT BOUNDARY)
                                 │
┌────────────────────────────────┼──────────────────────────────────────┐
│                        ┌───────▼───────┐                              │
│                        │ speechserver  │                              │
│                        │   daemon      │                              │
│                        └───────┬───────┘                              │
│                                │                                      │
│  ┌─────────────────┐  ┌───────┴───────┐  ┌─────────────────┐         │
│  │   Whisper STT   │  │    Ollama     │  │   Piper TTS     │         │
│  │  (Voice → Text) │  │   (LLM API)   │  │  (Text → Voice) │         │
│  └─────────────────┘  └───────────────┘  └─────────────────┘         │
│                                                                       │
│                           MOTOR (speechd-ng)                          │
└───────────────────────────────────────────────────────────────────────┘
```

### Critical Rule

> **Never modify `speechserverdaemon` from a Stratus task.**
>
> The daemon is a standalone service used by multiple projects. If Stratus needs a new capability (e.g., frequency awareness), implement it in Stratus by injecting context into the `Think(context)` D-Bus call.

---

## 4. Data Flow

### Simulator → Client → AI → Audio

```
X-Plane                  Stratus Client               AI Stack
────────                 ──────────────               ────────
   │                           │                          │
   │ DataRefs (1Hz)            │                          │
   ├──────────────────────────►│                          │
   │ → stratus_telemetry.json       │                          │
   │                           │ Build ATC prompt         │
   │                           │ (lat, lon, alt, freq,    │
   │                           │  airport, flight phase)  │
   │                           │                          │
   │                           │ Think(context) ──────────┤
   │                           │                          │ Ollama
   │                           │◄───── Response ──────────┤
   │                           │                          │
   │                           │ Speak(text) ─────────────┤
   │                           │                          │ Piper TTS
   │                           │◄───── Audio ─────────────┤
   │                           │                          │
   │ stratus_commands.jsonl       │                          │
   │◄──────────────────────────┤                          │
   │ (Squawk, freq commands)   │                          │
```

### Key Files

| Path | Purpose |
|:-----|:--------|
| `~/.local/share/StratusATC/stratus_telemetry.json` | Telemetry from X-Plane |
| `~/.local/share/StratusATC/stratus_commands.jsonl` | Commands to X-Plane |
| `~/.local/share/StratusATC/stratus_atc.log` | Plugin debug log |

---

## 5. Codebase Overview

### Directory Structure

```
Stratus/
├── client/                    # Python GUI (current primary)
│   └── src/
│       ├── main.py            # Entry point
│       ├── core/
│       │   ├── providers/     # ATC backends (local only now)
│       │   ├── sim_data.py    # Telemetry parsing
│       │   └── airport_manager.py  # Airport database lookup
│       └── ui/
│           └── main_window.py # Qt6 GUI (1656 lines)
│
├── stratus-rs/               # Rust rewrite (in progress)
│   ├── stratus-core/         # Shared library
│   │   └── src/
│   │       ├── atc.rs        # Prompt building
│   │       ├── ollama.rs     # LLM client
│   │       └── telemetry.rs     # Telemetry watcher
│   └── stratus-gui/          # Iced GUI + ComLink server
│       └── src/
│           ├── app.rs        # Elm-style app (300 lines)
│           └── comlink.rs    # Axum web server
│
├── adapters/xplane/          # X-Plane C plugin
│   ├── src/stratus_plugin.c  # DataRef reading, JSON writing
│   └── StratusATC/           # Built plugin (fat binary)
│
└── docs/                     # Documentation
    ├── ATC_ROADMAP.md        # Feature phases
    ├── ATC_PHRASEOLOGY.md    # FAA reference
    └── VFR_PHRASEOLOGY.md    # VFR comms guide
```

### Key Classes

| Class | File | Purpose |
|:------|:-----|:--------|
| `MainWindow` | `main_window.py` | Qt6 GUI, prompt building, audio handling |
| `AirportManager` | `airport_manager.py` | OurAirports database, nearest-airport lookup |
| `SimDataInterface` | `sim_data.py` | JSON telemetry parser |
| `StratusApp` | `app.rs` | Rust Iced GUI (Elm architecture) |
| `OllamaClient` | `ollama.rs` | Async HTTP client for Ollama API |

---

## 6. Current Phase: Where We Are

### Completed ✅

| Phase | Description | Status |
|:------|:------------|:-------|
| Phase 1 | Core Infrastructure | ✅ Done |
| Phase 2 | Airport Awareness | ✅ Done |

### In Progress 🚧

| Phase | Description | Status |
|:------|:------------|:-------|
| Phase 3 | Voice Input (Whisper STT, PTT) | 🚧 Active |
| Rust Migration | Performance-critical components | 🚧 Scaffolded |

### Planned 📋

| Phase | Description |
|:------|:------------|
| Phase 4 | Sim Control (parse AI → set squawk/frequency) |
| Phase 5 | Realistic Ground Ops (taxi, ATIS, runway assignment) |
| Phase 6 | VFR Flight Following (handoffs, traffic advisories) |
| Phase 7 | IFR Operations (SIDs, STARs, approach clearances) |
| Phase 8 | Packaging (AppImage, deb, dmg) |

---

## 7. Where We Want to Go

### Short-term (Q1 2025)

1. **Complete Voice Input** - PTT → Whisper → LLM → TTS loop
2. **Sim Control** - AI responses parsed and executed (set squawk via DataRef)
3. **Package for Distribution** - AppImage for Linux

### Medium-term (2025)

1. **Full VFR Experience** - Departure to destination with realistic handoffs
2. **Rust Migration** - Sub-1s latency, single binary distribution
3. **macOS Support** - Universal Binary plugin

### Long-term Vision

1. **Living World** - AI traffic, sequencing, go-arounds
2. **IFR Operations** - Full instrument flight with SIDs/STARs
3. **Community Voices** - Crowdsourced TTS models for regional accents

### The AGI Connection

Stratus is a testbed for **safe human-AI interaction in high-stakes domains**:

- Real-time decision-making under time pressure
- Domain-specific knowledge (FAA procedures) combined with natural language
- Graceful degradation when AI fails
- Human always has final authority

> *"If we can build an AI that pilots trust, we've learned something important about alignment."*

---

## 8. How to Run

### Python Client (Current)

```bash
cd /home/startux/Code/Stratus
source .venv/bin/activate
python client/src/main.py
```

### Rust Client (In Development)

```bash
cd /home/startux/Code/Stratus/stratus-rs
cargo run --release -p stratus-gui
```

### Run Tests

```bash
cd /home/startux/Code/Stratus
PYTHONPATH=. pytest tests/
```

### Check Plugin Logs

```bash
tail -f ~/.local/share/StratusATC/stratus_atc.log
```

---

## 9. Key Documents

| Document | Purpose |
|:---------|:--------|
| [GUARDRAILS.md](GUARDRAILS.md) | Non-negotiable safety requirements |
| [TEAM_STRUCTURE.md](TEAM_STRUCTURE.md) | Advisory team personas |
| [ATC_ROADMAP.md](docs/ATC_ROADMAP.md) | Feature development phases |
| [OFFLINE_ATC_HANDOFF.md](docs/OFFLINE_ATC_HANDOFF.md) | Architecture boundary rules |

---

## 10. Your Role

As an advisory team member, you bring domain expertise to guide development decisions.

### When to Engage

- **Code Reviews**: Does this change align with your domain?
- **Feature Design**: What would a real [pilot/controller/security expert] expect?
- **Architecture**: Is this the simplest solution that works?
- **Red Team**: What could go wrong? How could this be abused?

### Key Questions to Always Ask

1. **Captain Martinez / Controller Hayes**: "Is this how it works in the real world?"
2. **Dogan / Carmack**: "Is this the simplest architecture that solves the problem?"
3. **Stamos / Russell / Leike**: "What's the worst that could happen? Is the human in control?"
4. **Hernandez**: "Can a pilot use this without distraction?"

---

**Welcome aboard. Let's build something great.**
