# Stratus ☁️

**Open Source AI Air Traffic Control for Flight Simulators**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux-orange.svg)]()
[![Status: Active Development](https://img.shields.io/badge/Status-Active%20Development-brightgreen.svg)]()

Stratus is a native Linux ATC (Air Traffic Control) system for flight simulators. It provides realistic voice-controlled ATC interactions using local AI and text-to-speech.

## Features

- 🎙️ **Voice Control** - Talk to ATC using natural speech
- 🔊 **Realistic ATC Audio** - AI-generated ATC responses with proper phraseology
- ✈️ **X-Plane Integration** - Native plugin for X-Plane 11/12
- 🖥️ **Qt6 GUI** - Modern dark-themed desktop client
- 🌐 **ComLink Web UI** - Touch-friendly interface for tablets/VR
- 🐧 **Linux Native** - No Wine or Proton required

## Quick Start

### Prerequisites

- X-Plane 11 or 12 with [XPPython3](https://xppython3.readthedocs.io/)
- Python 3.10+
- Qt6 (for GUI)

### Installation

```bash
# Clone the repository
git clone https://github.com/StarTuz/stratus.git
cd stratus

# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install -r client/requirements.txt

# Run (auto-installs X-Plane plugin)
python client/src/main.py
```

### ComLink (Tablet/VR Interface)

Access from any device on your network:
```
http://localhost:8080/comlink
```

## Architecture

```
┌─────────────┐      JSON Files       ┌──────────────────┐
│  X-Plane    │ ──────────────────────│  Stratus Client  │
│  Plugin     │  simAPI_input.json    │  (Python/Qt6)    │
└─────────────┘                       └────────┬─────────┘
                                               │
                              ┌────────────────┼────────────────┐
                              │                │                │
                              ▼                ▼                ▼
                       ┌──────────┐    ┌──────────┐    ┌──────────┐
                       │ LocalATC │    │ SayIntent│    │ (Future) │
                       │ LLM+TTS  │    │   (opt)  │    │ backends │
                       └──────────┘    └──────────┘    └──────────┘
```

## Simulator Support

| Simulator | Linux | macOS | Status |
|-----------|-------|-------|--------|
| X-Plane 12 | ✅ | 🔜 | Native plugin |
| X-Plane 11 | ✅ | 🔜 | Native plugin |
| MSFS 2024 | 🔄 | ❌ | Via Proton (planned) |

## Documentation

- [HANDOFF.md](HANDOFF.md) - Project status and technical details
- [ASSESSMENT_AND_ROADMAP.md](ASSESSMENT_AND_ROADMAP.md) - Roadmap
- [X-Plane Plugin](adapters/xplane/README.md) - Plugin documentation

## Contributing

Contributions welcome! This is a community-driven open source project.

## License

MIT License - See [LICENSE](LICENSE) for details.
