# Stratus ATC Guardrails Requirements

> These are NON-NEGOTIABLE requirements. Code that violates these MUST NOT be merged.

## 1. AIAM: Agent Governance

### No-Touch Zones

- **FORBIDDEN:** Modifying or deleting system binaries (e.g., `/usr/bin/`, `/home/startux/.local/bin/`).
- **FORBIDDEN:** Modifying `speechserverdaemon` source code from a Stratus task (see Architecture Boundary).
- **FORBIDDEN:** Deleting project source files based on heuristic assumptions without Explicit Verification (EV).
- **FORBIDDEN:** Modifying X-Plane installation files outside `Output/plugins/StratusATC/`.

### Architecture Boundary

```
Stratus (Brain)               speechserverdaemon (Motor)
─────────────────              ───────────────────────────
ATC logic, prompts             TTS/STT/LLM (Piper, Vosk, Ollama)
Telemetry tracking             D-Bus interface
X-Plane plugin                 Audio device management
          │                              │
          └──────── D-Bus ───────────────┘
```

> **CRITICAL:** The `speechserverdaemon` is a standalone, release-stage service used by multiple projects. All ATC-specific logic MUST be implemented in Stratus by injecting context into `Think(context)` calls.

### Action Risk Tiers

| Tier | Risk Level | Examples |
|:-----|:-----------|:---------|
| **T0** | Safe | Read-only, linting, UI state, `view_file` |
| **T1** | Normal | Incremental code edits, new feature files, test additions |
| **T2** | High-Risk | Dependency changes (`Cargo.toml`, `requirements.txt`), API changes, schema changes |
| **T3** | Restricted | File DELETIONS, binary changes, system config, X-Plane SDK updates |

### Mandatory Verification (EV)

- **REQUIRED:** Before T2/T3 actions, agents MUST use `view_file` or `ls -l` to verify current state.
- **REQUIRED:** All T3 actions must be logged with a `justification` in the audit log.

---

## 2. Input Validation

### LLM Response Validation

```python
# REQUIRED: Validate Ollama responses before TTS
if not response or len(response.strip()) < 3:
    logger.warning("Empty or invalid LLM response, skipping TTS")
    return

# REQUIRED: Sanitize unexpected content
if any(marker in response for marker in ["<|", "|>", "```"]):
    logger.warning("LLM response contains markup, cleaning...")
    response = clean_llm_response(response)
```

### Telemetry Validation

```python
# REQUIRED: Validate telemetry before prompt injection
def validate_telemetry(data: dict) -> bool:
    required_fields = ["latitude", "longitude", "altitude", "heading"]
    return all(field in data and data[field] is not None for field in required_fields)
```

### Rate Limiting

```python
# REQUIRED: Max 1 ATC request per 2 seconds (LLM cold-start overhead)
MIN_REQUEST_INTERVAL_S = 2.0
if time.time() - last_request_time < MIN_REQUEST_INTERVAL_S:
    return  # Rate limited
```

---

## 3. Output Validation

### Entity Verification

```python
# REQUIRED: Verify airport exists before using in prompt
airport = airport_manager.find_nearest(lat, lon)
if airport is None:
    logger.warning("No airport found for coordinates, using generic ATC")
    facility_name = "Regional Approach"
```

### Destructive Action Confirmation

```python
# REQUIRED: High-risk sim control commands need confirmation
DANGEROUS_COMMANDS = ["gear up", "emergency", "mayday", "squawk 7700"]
if any(cmd in ai_response.lower() for cmd in DANGEROUS_COMMANDS):
    await confirm_with_pilot("Confirm emergency declaration?")
```

---

## 4. Error Handling

### No Silent Failures

```python
# FORBIDDEN:
try:
    send_to_ollama(prompt)
except:
    pass

# REQUIRED:
try:
    send_to_ollama(prompt)
except Exception as e:
    logger.error(f"Ollama request failed: {e}")
    show_user_error("AI service temporarily unavailable")
```

### Timeout Handling

```python
# REQUIRED: Handle LLM cold-start timeouts gracefully
# Ollama initial inference can take up to 30s
try:
    response = await asyncio.wait_for(ollama_think(prompt), timeout=30.0)
except asyncio.TimeoutError:
    logger.warning("LLM response timed out after 30s")
    show_status("AI is loading, please wait...")
```

---

## 5. Async Safety (Rust)

### No Blocking in Async

```rust
// FORBIDDEN: Blocking calls in async context
let status = child.wait()?;

// REQUIRED: Use spawn_blocking
tokio::task::spawn_blocking(move || {
    child.wait()
}).await??;
```

### File I/O in Async

```rust
// REQUIRED: Use tokio::fs for file operations
use tokio::fs;
let telemetry = fs::read_to_string(TELEMETRY_INPUT_PATH).await?;
```

---

## 6. Audit Logging

### Action Log

```python
# REQUIRED: Log all ATC interactions
logger.info(
    f"ATC Exchange | Facility: {facility_name} | "
    f"Pilot: {pilot_message[:50]}... | "
    f"ATC: {atc_response[:50]}..."
)
```

### Telemetry Log

```python
# REQUIRED: Log telemetry updates for debugging
logger.debug(f"Telemetry: alt={altitude}ft hdg={heading}° spd={speed}kts")
```

---

## 7. X-Plane Plugin Safety

### Plugin Boundaries

```c
// REQUIRED: Plugin must not crash X-Plane on failure
if (json_file == NULL) {
    XPLMDebugString("StratusATC: Failed to open telemetry file, skipping write\n");
    return;  // Graceful degradation, NOT a crash
}
```

### DataRef Access

```c
// REQUIRED: Validate DataRefs before use
XPLMDataRef alt_ref = XPLMFindDataRef("sim/flightmodel/position/elevation");
if (alt_ref == NULL) {
    XPLMDebugString("StratusATC: elevation DataRef not found\n");
    return;
}
```

---

## Summary: Critical Invariants

1. **Never modify `speechserverdaemon`** from Stratus context
2. **Validate all telemetry** before injecting into prompts
3. **Handle 30s LLM timeouts** without freezing the UI
4. **No silent failures** - every error path must be logged
5. **Plugin must not crash X-Plane** - graceful degradation only
