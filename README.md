# SayIntentionsAIml - Linux & Mac Client for SayIntentions.AI

**Community / Open Source Port**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status: Active Development](https://img.shields.io/badge/Status-Active%20Development-brightgreen.svg)]()

A native Linux and macOS client for the [SayIntentions.AI](https://sayintentions.ai) ATC service.

> **🎉 BREAKTHROUGH (Dec 23, 2024)**: Native client confirmed feasible!  
> We have successfully tested the REST API and downloaded real ATC audio.  
> See [HANDOFF.md](HANDOFF.md) for full details.

> **Status**: Active Development  
> **Current Focus**: Building the Python client

## What is SayIntentions.AI?

SayIntentions.AI provides realistic AI-powered Air Traffic Control for flight simulators. Their official client is Windows-only. This project aims to bring native support to Linux and macOS users.

## Project Structure

```
SayIntentionsML/
├── client/                 # Native Python client (GUI + Audio)
│   └── src/
│       ├── core/          # SAPI interface (cloud communication)
│       ├── ui/            # PySide6 GUI
│       ├── audio/         # Audio capture/playback
│       └── simapi/        # SimAPI file watcher
│
├── adapters/
│   └── xplane/            # X-Plane Native Plugin (C)
│       ├── SayIntentionsAIml/
│       │   ├── lin_x64/   # ✅ Linux plugin (built)
│       │   ├── mac_x64/   # 🔜 macOS plugin
│       │   └── win_x64/   # Optional Windows plugin
│       ├── src/           # Plugin source code
│       └── sdk/           # X-Plane SDK (downloaded)
│
└── docs/                   # Documentation
```

## Quick Start

### 1. Build the X-Plane Plugin (Linux)

```bash
cd adapters/xplane
./setup_sdk.sh           # Download X-Plane SDK
mkdir build && cd build
cmake .. && make
```

### 2. Install the Plugin

```bash
cp -r SayIntentionsAIml ~/X-Plane\ 12/Resources/plugins/
```

### 3. Run the Client (Mock Mode)

```bash
cd client
pip install -r requirements.txt
python src/main.py
```

## Simulator Support

| Simulator | Linux | macOS | Status |
|-----------|-------|-------|--------|
| X-Plane 12 | ✅ | 🔜 | Native plugin |
| X-Plane 11 | ✅ | 🔜 | Native plugin |
| MSFS 2024 | 🔄 | ❌ | Via Proton bridge |
| MSFS 2020 | 🔄 | ❌ | Via Proton bridge |

- ✅ Supported
- 🔜 In Progress
- 🔄 Planned (requires Proton)
- ❌ Not Possible

## Architecture

This project uses **SimAPI**, the same file-based protocol as the official Windows client:

```
┌─────────────┐      JSON Files       ┌──────────────────┐
│  X-Plane    │ ──────────────────────│  Native Client   │
│  Plugin     │  simAPI_input.json    │  (Python/Qt)     │
│  (C)        │ <─────────────────────│                  │
│             │  simAPI_output.jsonl  │                  │
└─────────────┘                       └────────┬─────────┘
                                               │ REST/Audio
                                               ▼
                                      ┌──────────────────┐
                                      │ SayIntentions.AI │
                                      │ Cloud (SAPI)     │
                                      └──────────────────┘
```

## Documentation

- **[HANDOFF.md](HANDOFF.md)** - 🎉 Project handoff with breakthrough confirmation
- **[SAPI Findings](docs/SAPI_FINDINGS.md)** - API research with live test results
- [Project Status](PROJECT_STATUS.md) - Current state and next steps
- [Assessment and Roadmap](ASSESSMENT_AND_ROADMAP.md) - Technical feasibility study
- [X-Plane 12 Platform State](docs/XPLANE_12_PLATFORM.md) - Current XP12 features & Web API
- [X-Plane Extensions](docs/XPLANE_EXTENSIONS.md) - Integrations to fill X-Plane feature gaps
- [X-Plane Plugin README](adapters/xplane/README.md) - Build instructions

## Contributing

This is an open-source community project. Contributions welcome!

## License

MIT License

## Disclaimer

This is an unofficial community project and is not affiliated with SayIntentions.AI. 
Use of the SayIntentions.AI service requires a valid subscription from the official provider.
