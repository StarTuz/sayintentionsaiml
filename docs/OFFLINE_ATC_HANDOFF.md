# ARCHITECTURE HANDOFF: Offline ATC Separation

**IMPORTANT**: This document defines the strict project boundaries for Stratus and its speech backend.

## 1. Project Roles
- **Stratus (Client)**: **THE BRAIN**.
  - Responsible for: X-Plane data, Radio/Frequency tracking, ATC logic, Prompt Engineering.
  - Controls: The "Content" of the speech and the "Logic" of the ATC.
- **speechserverdaemon (External)**: **THE MOTOR**.
  - Responsible for: TTS (Piper), STT (Wyoming/Vosk), LLM (Ollama).
  - Controls: The "Physicality" of the speech and the raw AI "Inference".

## 2. Strict Boundary Rules
1. **NEVER** modify the `speechserverdaemon` source code from a Stratus task. The daemon is a standalone, release-stage service used by other apps.
2. If the daemon lacks a feature (e.g., frequency tracking), **implement it in the Stratus client** by maintaining state and injecting it into the context of the `Think(context)` call.
3. **STT Fallback**: The daemon handles STT engines. It is configured via `Speech.toml` to prefer Wyoming but falls back to Vosk and other internal mechanisms. Stratus should simply call `ListenVad` and handle the result.

## 3. Latency & Timeouts
- All D-Bus calls should have a maximum timeout of **10 seconds**.
- To achieve this, the daemon must be configured for **Wyoming** (Fastest engine).
- Slow cold-starts are managed by the daemon's internal threading; the client should expect a response or a timeout within 10s.

## 4. User Customization
- Users tailor their experience (choosing LLM models, voices, etc.) by editing **`~/.config/speechd-ng/Speech.toml`**.
- Stratus documentation should point users there rather than trying to provide UI for daemon internals.
