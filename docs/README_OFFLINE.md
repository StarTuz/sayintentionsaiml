# Offline ATC: User Customization Guide

Stratus provides a fully localized "Offline ATC" experience, giving you high-performance, private AI interaction without the need for a cloud subscription.

## AI & LLM Tailoring

The "Brain" of your ATC is powered by Ollama running locally. You can customize which model is used to suit your hardware and balance speed vs. intelligence.

### Changing LLM Models
To change the active AI model:
1. Refer to the `speechserverdaemon` documentation.
2. Edit your `Speech.toml` configuration (usually located in `~/.config/speechd-ng/Speech.toml`).
3. Update the `ollama_model` field:
   ```toml
   ollama_model = "llama3"   # Or "mistral", "phi3", etc.
   ```
4. Restart the `speechserverdaemon` service.

## Performance & Latency Warning

Stratus expects a response from the speech engine within **10 seconds**. 

If you experience D-Bus `NoReply` or "Engine Latency" errors:
- **STT Backend**: Ensure you are using the **Wyoming/Whisper** backend in `speechserverdaemon`. The CLI-based Vosk backend often exceeds 10s due to model loading overhead.
- **LLM Choice**: Lighter models (like `phi3` or `mistral`) respond significantly faster than larger models (like `llama3:70b`).
- **Hardware**: AI processing speed depends entirely on your local GPU/CPU performance.
