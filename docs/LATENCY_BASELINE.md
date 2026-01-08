# Latency Baseline - Stratus ATC

> **STRATUS-001: Measure End-to-End Latency**

## Pipeline Stages

```
PTT Press → STT (Whisper) → LLM (Ollama) → TTS (Piper) → Audio
```

## Measurement Points

| Mark | Stage | Description |
|:-----|:------|:------------|
| `start` | PTT | Push-to-talk button pressed |
| `stt_complete` | STT | Whisper transcription finished |
| `llm_start` | LLM | Before calling Ollama Think() |
| `llm_complete` | LLM | Ollama response received |
| `tts_start` | TTS | Before calling Piper Speak() |
| `tts_complete` | TTS | TTS audio generated |
| `end` | Done | Complete pipeline (response shown to user) |

## Target Latency

| Metric | Target | Notes |
|:-------|:-------|:------|
| **Total E2E** | < 2000ms | PTT → UI update |
| **STT** | < 500ms | Whisper via VAD |
| **LLM** | < 1000ms | Ollama warm |
| **TTS** | < 300ms | Piper |

## Log Location

```
~/.local/share/StratusATC/latency.jsonl
```

## Example Log Entry

```json
{
  "session_id": "ptt_1736330000_1",
  "start_time": 1736330000.123,
  "marks": {
    "stt_complete": 1736330000.523,
    "llm_start": 1736330000.525,
    "llm_complete": 1736330001.125,
    "tts_start": 1736330001.130,
    "tts_complete": 1736330001.330
  },
  "end_time": 1736330001.335,
  "total_ms": 1212.0,
  "segments_ms": {
    "start_to_stt_complete": 400.0,
    "stt_complete_to_llm_start": 2.0,
    "llm_start_to_llm_complete": 600.0,
    "llm_complete_to_tts_start": 5.0,
    "tts_start_to_tts_complete": 200.0,
    "tts_complete_to_end": 5.0
  },
  "timestamp": "2026-01-07T18:50:00"
}
```

## Viewing Latency Data

```bash
# View recent latency logs
tail -20 ~/.local/share/StratusATC/latency.jsonl | jq .

# Get average latency
jq -s 'map(.total_ms) | add / length' ~/.local/share/StratusATC/latency.jsonl

# Count over-target
jq -s 'map(select(.total_ms > 2000)) | length' ~/.local/share/StratusATC/latency.jsonl
```

## Optimization Targets

If over target, focus on:

1. **LLM Cold Start** — First query takes 5-15s. Use smaller model or keep warm.
2. **LLM Model Size** — `llama3.2:3b` faster than `mistral:7b`
3. **STT Timeout** — Current 10s timeout is conservative
4. **TTS** — Piper is fast, unlikely bottleneck

## Status

- [x] Instrumentation added to `main_window.py`
- [x] `core/latency.py` module created
- [ ] Baseline measurements (run tests)
- [ ] Document actual numbers
