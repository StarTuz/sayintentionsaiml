# Architecture Decision: Offline ATC (Local) vs Cloud (SayIntentions)

## Summary
We are pivoting to a **Fully Localized Offline ATC Architecture**. While we preserve the legacy `cloud` provider stubs, our primary mission is to replicate the SayIntentions experience using local hardware.

## Comparison Matrix

| Feature | Offline ATC (Local) | Cloud (SayIntentions) |
| :--- | :--- | :--- |
| **Cost** | **Free** (Open Source) | **Paid** (Subscription) |
| **Privacy** | **Total** (No telemetry leaves PC) | **Cloud** (Audio/Telemetry sent) |
| **Latency** | **Offline/Native** (<200ms D-Bus) | **HTTP** (>1000ms delay) |
| **Integrations** | **X-Plane Native (C)** | **Windows Proccess/DLL** |

## The "Brain and Motor" Division
- **Stratus (The Brain)**: Replicates the logic of SayIntentions. It handles frequency changes, transponder states, and proximity alerts. It sends a "context-heavy" prompt to the motor.
- **speechserverdaemon (The Motor)**: A stateless utility that takes text and speaks (TTS), or takes a microphone stream and returns text (STT).

## Why Offline?
1.  **Unlimited Usage**: No daily token limits or cloud outages.
2.  **Privacy**: Your voice and sim location are never uploaded.
3.  **Performance**: 100% native Linux implementation without Wine or emulation.
