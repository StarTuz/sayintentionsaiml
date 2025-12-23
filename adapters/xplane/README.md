# SayIntentionsAIml X-Plane Plugin

Native C plugin adapter for X-Plane 11/12 that bridges simulator data to the SayIntentions native client.

## Directory Structure (XPLM 3.0 Fat Plugin Format)

```
X-Plane 12/
└── Resources/
    └── plugins/
        └── SayIntentionsAIml/          <- This folder
            ├── lin_x64/
            │   └── SayIntentionsAIml.xpl   <- Linux 64-bit
            ├── mac_x64/
            │   └── SayIntentionsAIml.xpl   <- macOS 64-bit (Universal)
            └── win_x64/
                └── SayIntentionsAIml.xpl   <- Windows 64-bit (optional)
```

## Building

### Prerequisites

1. **X-Plane SDK**: Download from https://developer.x-plane.com/sdk/plugin-sdk-downloads/
2. **CMake 3.16+**
3. **C Compiler**: GCC (Linux), Clang/Xcode (macOS)

### Setup SDK

```bash
cd adapters/xplane
./setup_sdk.sh
```

Or manually:
1. Download SDK ZIP from X-Plane developer site
2. Extract to `adapters/xplane/sdk/`
3. Verify `sdk/CHeaders/XPLM/XPLMPlugin.h` exists

### Build (Linux)

```bash
cd adapters/xplane
mkdir build && cd build
cmake ..
make
```

Output: `SayIntentionsAIml/lin_x64/SayIntentionsAIml.xpl`

### Build (macOS)

```bash
cd adapters/xplane
mkdir build && cd build
cmake ..
make
```

Output: `SayIntentionsAIml/mac_x64/SayIntentionsAIml.xpl`

## Installation

Copy the entire `SayIntentionsAIml` folder to:
- **X-Plane 12**: `X-Plane 12/Resources/plugins/`
- **X-Plane 11**: `X-Plane 11/Resources/plugins/`

## What It Does

1. **Reads** X-Plane DataRefs (position, radios, autopilot, etc.)
2. **Writes** to `~/.local/share/SayIntentionsAI/simAPI_input.json` at 1Hz
3. **Reads** commands from `~/.local/share/SayIntentionsAI/simAPI_output.jsonl`
4. **Applies** commands back to the simulator

## Data Paths

| Platform | Data Directory |
|----------|----------------|
| Linux    | `~/.local/share/SayIntentionsAI/` |
| macOS    | `~/Library/Application Support/SayIntentionsAI/` |
| Windows  | `%LOCALAPPDATA%\SayIntentionsAI\` |

## License

MIT License - Community/Open Source Project
