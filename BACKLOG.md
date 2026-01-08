# Stratus ATC - Project Backlog

> **Last Updated:** 2026-01-07
> **Source:** Team Advisory Meeting + SayIntentions Analysis

---

## Priority Levels

| Priority | Meaning | Timeline |
|:---------|:--------|:---------|
| **P0** | Critical / Blocking | This sprint |
| **P1** | High / Core functionality | Next 2 sprints |
| **P2** | Medium / Enhancement | Q1 2025 |
| **P3** | Low / Nice-to-have | Backlog |

---

## P0: Critical (This Sprint)

### STRATUS-001: Measure End-to-End Latency

**Owner:** Carmack  
**Type:** Infrastructure  
**Description:**  
Establish baseline latency metrics for the full PTT → STT → LLM → TTS → Audio pipeline.

**Acceptance Criteria:**

- [ ] Instrument code to log timestamps at each stage
- [ ] Create benchmark script that runs 10 test utterances
- [ ] Document results in `docs/LATENCY_BASELINE.md`
- [ ] Set target: <2 seconds end-to-end

**Why:** Without measurement, we can't optimize.

---

### STRATUS-002: Add Frequency Display to UI Header

**Owner:** Hernandez  
**Type:** UX  
**Description:**  
Pilots need to see COM1/COM2 frequencies at a glance without scrolling.

**Acceptance Criteria:**

- [ ] Add frequency display widget to Qt6 header bar
- [ ] Show format: `COM1: 118.700 | COM2: 121.500`
- [ ] Update from telemetry in real-time (1Hz)
- [ ] Style: High contrast, monospace font

**Why:** Basic pilot expectation. Critical for realism.

---

### STRATUS-003: LLM Response Validation

**Owner:** Russell, Stamos  
**Type:** Safety  
**Description:**  
Validate LLM output before sending to TTS to catch hallucinations and prompt injection.

**Acceptance Criteria:**

- [ ] Create `validate_atc_response()` function
- [ ] Check for unexpected content markers (`<|`, `|>`, triple backticks)
- [ ] Check for obviously wrong values (altitudes > 60,000ft, frequencies outside band)
- [ ] Log validation failures with full context
- [ ] Fallback to "Say again" on validation failure

**Why:** LLMs hallucinate. Aviation can't tolerate nonsense commands.

---

### STRATUS-004: Response Logging for SECA

**Owner:** Leike  
**Type:** Safety / Audit  
**Description:**  
Log all LLM responses for post-hoc analysis and behavioral drift detection.

**Acceptance Criteria:**

- [ ] Create `~/.local/share/StratusATC/atc_responses.jsonl`
- [ ] Log: timestamp, prompt_hash, response, validation_result
- [ ] Rotate logs at 100MB
- [ ] Add config option to disable (privacy)

**Why:** Required for Self-Evolving Capability Audit.

---

## P0: Latency Advantage Sprint (Priority)

> **Team Consensus:** These tickets leverage our local-first architecture to achieve
> performance that cloud competitors cannot match. Estimated: 50% perceived latency reduction.

### STRATUS-024: Streaming LLM to TTS

**Owner:** Carmack, Dogan  
**Type:** Performance  
**Priority:** P0 — Critical  
**Description:**  
Stream Ollama tokens directly to TTS instead of waiting for complete response.

**Current Flow:**

```
PTT → STT → [Wait for full LLM response] → TTS → Audio
```

**Target Flow:**

```
PTT → STT → LLM token → TTS chunk → Audio (while LLM continues)
```

**Acceptance Criteria:**

- [ ] Implement streaming callback from Ollama API
- [ ] Buffer tokens until natural phrase boundary (punctuation, pause)
- [ ] Send phrase chunks to TTS incrementally
- [ ] Target: First audio plays within 500ms of LLM start

**Why:** Cuts perceived latency by 40-60%. Cloud services can't do this efficiently.

---

### STRATUS-025: Model Warmup Heartbeat

**Owner:** Carmack  
**Type:** Performance  
**Priority:** P0  
**Description:**  
Keep Ollama model hot by sending periodic lightweight prompts.

**Problem:** First LLM call after idle can take 5-15s (model loading).

**Solution:**

- Send heartbeat prompt every 30 seconds: `"Ready"` → `"Standing by"`
- Discard response, but keeps model in GPU memory

**Acceptance Criteria:**

- [ ] Background thread sends heartbeat every 30s when idle
- [ ] Configurable interval in settings
- [ ] Kill heartbeat when PTT active (don't compete for GPU)
- [ ] Log warmup status

**Why:** Eliminates cold-start surprise latency.

---

### STRATUS-026: Speculative Pre-generation Cache

**Owner:** Carmack, Dogan  
**Type:** Performance  
**Priority:** P1  
**Description:**  
Pre-generate likely ATC responses based on current flight phase.

**Example (Taxiing):**

- Pre-generate: "Hold short runway 28"
- Pre-generate: "Taxi to runway 28 via Alpha"
- Pre-generate: "Contact tower on 118.7"

When pilot speaks, fuzzy-match to cache. If hit → **instant response**.

**Acceptance Criteria:**

- [ ] Define 5-10 templates per flight phase
- [ ] Background generation during idle moments
- [ ] Cache invalidation when context changes (new airport, runway change)
- [ ] Metric: Cache hit rate %

**Why:** Sub-200ms response for common exchanges.

---

### STRATUS-027: Phraseology Feedback Scoring

**Owner:** Controller Hayes, Leike  
**Type:** Feature  
**Priority:** P1  
**Description:**  
Parse pilot readbacks and score correctness for training value.

**Checks:**

1. Did pilot read back hold short instruction?
2. Did pilot include runway number in takeoff/landing readback?
3. Did pilot read back squawk code?
4. Did pilot read back frequency?

**Acceptance Criteria:**

- [ ] Create `score_readback(atc_instruction, pilot_response)` function
- [ ] Return score (0-100) + list of missed items
- [ ] Optional "Training Mode" that provides feedback after each exchange
- [ ] Log scores for progress tracking

**Why:** Unique training value — no competitor offers this.

---

### STRATUS-028: Regional Controller Personality

**Owner:** Captain Martinez, Hernandez  
**Type:** Feature  
**Priority:** P2  
**Description:**  
Adjust ATC personality based on region/facility type.

**Profiles:**

- **NY/Boston:** Fast, curt, no-nonsense
- **Midwest:** Standard FAA, neutral
- **SoCal:** Relaxed, slightly casual
- **Remote/Rural:** Chatty, time to talk

**Implementation:**

- Personality injected into system prompt
- TTS speed/pitch adjusted per profile
- User-selectable in settings

**Acceptance Criteria:**

- [ ] Define 4 personality templates
- [ ] Map to regions (optional auto-detect from position)
- [ ] TTS parameter adjustment per personality

**Why:** Immersion differentiator.

---

### STRATUS-029: Ambient Radio Chatter

**Owner:** Hernandez, Controller Hayes  
**Type:** Feature  
**Priority:** P2  
**Description:**  
Generate occasional background radio transmissions for realism.

**Elements:**

- Other aircraft requesting services (pre-recorded or AI-generated)
- Controller working other traffic
- Frequency bleed/static effects
- Occasional "blocked" transmissions

**Acceptance Criteria:**

- [ ] Library of 20+ ambient transmission clips
- [ ] Random playback during quiet periods
- [ ] Volume ducking when player transmits
- [ ] Toggle on/off in settings

**Why:** Single-aircraft ATC feels artificial.

---

### STRATUS-030: Privacy-First Marketing

**Owner:** Stamos  
**Type:** Documentation  
**Priority:** P1  
**Description:**  
Explicitly market the privacy advantages of local-first architecture.

**Key Points:**

1. **No telemetry** — Flight data never leaves your machine
2. **No subscription** — Download models, use forever
3. **True offline** — Works without internet
4. **Open source** — Audit the code yourself
5. **No vendor lock-in** — Your data, your models

**Acceptance Criteria:**

- [ ] Add "Privacy" section to README.md
- [ ] Create `docs/PRIVACY.md` with detailed explanation
- [ ] Add privacy badge to main page

**Why:** Genuine competitive advantage, free marketing.

---

### STRATUS-031: Context Window Maximization

**Owner:** Dr. Dogan  
**Type:** Feature  
**Priority:** P2  
**Description:**  
Leverage full local context window without cloud API cost concerns.

**Inject into prompt:**

- Full airport diagram context (runway layout, taxiways)
- Recent flight history (last 10 exchanges)
- Current weather/METAR
- Active NOTAMs (if available)
- Other traffic in pattern (if simulated)

**Acceptance Criteria:**

- [ ] Define maximum context budget (8K tokens for small models, 32K for large)
- [ ] Priority ranking: conversation > weather > airport > NOTAM
- [ ] Dynamic trimming to fit budget
- [ ] Measure impact on response quality

**Why:** Cloud APIs charge per token. We don't.

---

## P1: High Priority (Next 2 Sprints)

### STRATUS-005: Flight Phase Detection

**Owner:** Captain Martinez, Controller Hayes  
**Type:** Core Logic  
**Description:**  
Track aircraft state through flight phases to enable phase-appropriate ATC.

**Phases:**

1. Parked / Pre-flight
2. Taxi
3. Takeoff / Departure
4. Cruise / En-route
5. Descent / Approach
6. Landing / Rollout
7. Taxi-in

**Acceptance Criteria:**

- [ ] Create `FlightPhaseTracker` class
- [ ] Detect phase from telemetry (groundspeed, altitude, gear, flaps)
- [ ] Inject current phase into ATC prompt context
- [ ] Unit tests for phase transition edge cases

**Why:** "Cleared to land" makes no sense during taxi.

---

### STRATUS-006: Squawk Code Handling

**Owner:** Controller Hayes  
**Type:** Core Logic  
**Description:**  
Process transponder codes and trigger appropriate ATC behavior.

**Acceptance Criteria:**

- [ ] Parse squawk code from telemetry
- [ ] Detect emergency codes: 7500 (hijack), 7600 (no radio), 7700 (emergency)
- [ ] Generate appropriate ATC response for emergencies
- [ ] Assign squawk codes on initial contact (VFR: 1200, or assigned)

**Why:** Core ATC functionality.

---

### STRATUS-007: Handoff Protocol

**Owner:** Controller Hayes  
**Type:** Core Logic  
**Description:**  
Implement facility transitions (Tower → Departure → Center → Approach → Tower).

**Acceptance Criteria:**

- [ ] Define transition triggers (altitude, distance from airport)
- [ ] Generate handoff phraseology: "Contact NorCal Approach on 124.0"
- [ ] Track current controlling facility
- [ ] Update ATC identity on handoff

**Why:** Essential for realistic en-route experience.

---

### STRATUS-008: ComLink Real Implementation

**Owner:** Hernandez, Dogan  
**Type:** Feature  
**Description:**  
Make ComLink web interface functional (currently placeholder).

**Acceptance Criteria:**

- [ ] Display real telemetry from Telemetry
- [ ] Show communication log
- [ ] PTT button (WebSocket for audio?)
- [ ] Frequency display and tuning
- [ ] Mobile-responsive design

**Why:** VR/tablet users need this.

---

### STRATUS-009: Refactor main_window.py

**Owner:** Dogan  
**Type:** Tech Debt  
**Description:**  
Split 1656-line god class into focused components.

**New Structure:**

```
ui/
├── main_window.py      # Coordinator only (~300 lines)
├── comm_widget.py      # Communications display
├── telemetry_panel.py  # Sim data display
├── settings_manager.py # Persistence
├── audio_panel.py      # Audio controls
└── status_bar.py       # Connection status
```

**Acceptance Criteria:**

- [ ] No single file > 500 lines
- [ ] Clear interfaces between components
- [ ] All existing functionality preserved
- [ ] Tests pass

**Why:** Technical debt blocks Phase 3 velocity.

---

## P2: Medium Priority (Q1 2025)

### STRATUS-010: Pilot Mode Selection

**Owner:** Captain Martinez  
**Type:** Feature  
**Inspiration:** SayIntentions Student/Standard/Pro modes

**Description:**  
Adjust ATC verbosity based on pilot experience level.

**Modes:**

- **Student:** Slower speech, explicit instructions, repeat key info
- **Standard:** Real-world ATC pacing
- **Pro:** Clipped, fast, assumes proficiency

**Acceptance Criteria:**

- [ ] Add mode selector to settings
- [ ] Adjust prompt templates per mode
- [ ] Adjust TTS speed per mode

---

### STRATUS-011: AI Copilot Mode

**Owner:** Captain Martinez  
**Type:** Feature  
**Inspiration:** SayIntentions AI Copilot

**Description:**  
AI handles radio communications on behalf of pilot.

**Capabilities:**

- [ ] Auto-readback clearances
- [ ] Auto-check-in on frequency changes
- [ ] Set transponder from clearances
- [ ] Configurable: Full/Partial/Pilot-only

**Acceptance Criteria:**

- [ ] Copilot toggle in UI
- [ ] Visual indicator when copilot is transmitting
- [ ] Override button to take manual control

**Why:** Reduces workload, great for learning.

---

### STRATUS-012: Readback Verification

**Owner:** Captain Martinez  
**Type:** Core Logic  
**Description:**  
ATC should catch incorrect pilot readbacks.

**Acceptance Criteria:**

- [ ] Parse pilot readback from STT
- [ ] Compare against issued clearance
- [ ] Generate correction: "Negative, runway 28 LEFT"
- [ ] Track readback state

---

### STRATUS-013: ATIS Simulation

**Owner:** Controller Hayes  
**Type:** Feature  
**Description:**  
Generate ATIS from METAR data.

**Acceptance Criteria:**

- [ ] Fetch METAR from aviationweather.gov (optional, can be offline)
- [ ] Generate ATIS information (Alpha, Bravo, etc.)
- [ ] Include: wind, altimeter, active runway, remarks
- [ ] Update hourly

---

### STRATUS-014: Traffic Injection (Audio Only)

**Owner:** Controller Hayes  
**Type:** Feature  
**Inspiration:** SayIntentions traffic injection

**Description:**  
Simulate other aircraft on frequency for realism.

**Acceptance Criteria:**

- [ ] Generate synthetic traffic callsigns
- [ ] Play occasional "background" ATC transmissions
- [ ] Reference player in sequence: "Cessna 12345, number 2 following a 737 on 3-mile final"

**Why:** Single-aircraft ATC feels artificial.

---

### STRATUS-015: Navigraph Integration

**Owner:** Dogan  
**Type:** Feature  
**Inspiration:** SayIntentions Navigraph integration

**Description:**  
Use Navigraph charts for procedure data.

**Acceptance Criteria:**

- [ ] Read Navigraph AIRAC cycle data
- [ ] Extract SID/STAR names
- [ ] Use in ATC clearances

**Note:** Requires Navigraph subscription (optional feature).

---

### STRATUS-016: SimBrief Integration

**Owner:** Dogan  
**Type:** Feature  
**Inspiration:** SayIntentions SimBrief integration

**Description:**  
Import flight plans from SimBrief.

**Acceptance Criteria:**

- [ ] Read SimBrief OFP (Operational Flight Plan)
- [ ] Extract route, alternates, fuel
- [ ] Pre-populate ATC context with flight plan

---

## P3: Backlog (Future)

### STRATUS-017: Emergency Handling

**Owner:** Controller Hayes  
**Type:** Feature  
**Description:**  
Full emergency procedure support (engine failure, medical, fuel).

---

### STRATUS-018: IFR Clearance Delivery

**Owner:** Controller Hayes  
**Type:** Feature  
**Description:**  
Full IFR clearance with CRAFT format (Clearance, Route, Altitude, Frequency, Transponder).

---

### STRATUS-019: Multiple Voice Personas

**Owner:** Hernandez  
**Type:** Feature  
**Description:**  
Different TTS voices for different facilities/regions.

---

### STRATUS-020: Linux AppImage Packaging

**Owner:** Dogan  
**Type:** Distribution  
**Description:**  
Single-file Linux distribution.

---

### STRATUS-021: macOS Support

**Owner:** Dogan  
**Type:** Platform  
**Description:**  
Universal Binary plugin for macOS.

---

### STRATUS-022: Rust Migration Completion

**Owner:** Carmack, Dogan  
**Type:** Tech Debt / Performance  
**Description:**  
Complete `stratus-rs` to replace Python client for sub-1s latency.

---

### STRATUS-023: Living World (AI Traffic)

**Owner:** Controller Hayes  
**Type:** Feature (Long-term)  
**Description:**  
Inject AI aircraft into X-Plane, controlled by Stratus ATC.

---

## Differentiation from SayIntentions

### We Keep (Borrowed Ideas)

- AI Copilot concept (STRATUS-011)
- Student/Pro modes (STRATUS-010)
- Traffic audio injection (STRATUS-014)
- Navigraph/SimBrief integration (STRATUS-015, 016)

### We Improve

| SayIntentions | Stratus Improvement |
|:--------------|:--------------------|
| Cloud-dependent | **100% local, offline** |
| Subscription ($15-30/mo) | **Free forever** |
| Voice data sent to servers | **Voice stays on device** |
| Windows/Mac focused | **Linux-first** |
| Closed source | **Open source, community-driven** |
| Fixed LLM model | **User chooses Ollama model** |
| No safety guardrails | **SECA, response validation, audit logs** |

### We Add (Novel)

- **SECA Framework** - Self-Evolving Capability Audit for AI safety
- **Blue/Red Team Structure** - Adversarial code review built-in
- **Rust core** - Sub-1s latency target
- **ComLink** - Open web interface for VR/tablets
- **X-Plane native plugin** - No Python dependency in sim

---

## Ticket Workflow

1. **Triage** → P0/P1/P2/P3
2. **Owner assigned** → Persona takes responsibility
3. **In Progress** → Add `[WIP]` prefix
4. **Review** → Red Team review for P0/P1
5. **Done** → Move to completed section

---

## Completed

*(Move tickets here when done)*
