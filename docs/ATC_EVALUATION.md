# Stratus Offline ATC: Evaluation vs Real Life & SayIntentions.AI

## Executive Summary

| Aspect | Real Life ATC | SayIntentions.AI | Stratus Offline |
|--------|---------------|------------------|-----------------|
| **Phraseology Accuracy** | 100% (the standard) | ~95% (FAA/ICAO) | ~70% (learning) |
| **Voice Quality** | N/A (real humans) | 800+ controller voices | 1 Piper TTS voice |
| **Latency** | <1 second | 2-3 seconds (cloud) | 3-10 seconds (local) |
| **Cost** | N/A | $10/month | Free (your hardware) |
| **Privacy** | N/A | Cloud-based | 100% offline |
| **Traffic Awareness** | Yes (radar) | Yes (AI traffic) | No (single-pilot) |
| **Airport Database** | Complete | 88,000 airports | None (LLM inference) |
| **Procedure Following** | SID/STAR/IAP | SID/STAR/IAP | Basic clearances only |

---

## Detailed Comparison

### 1. Phraseology Accuracy

**Real Life ATC:**

- Source: FAA Order 7110.65, ICAO Doc 4444
- Format: `[Callsign], [Facility], [Instruction]`
- Examples: "November One Two Three Alpha Bravo, Ground, readability five"
- Readback/Hearback safety loop is mandatory

**SayIntentions.AI:**

- Claims ~95% FAA/ICAO accuracy
- "NearlyHuman" update adds natural pauses, occasional "ums"
- Community reports occasional odd vectors/altitudes during approach
- Actively refined via user feedback

**Stratus Offline:**

- ✅ Correct format (callsign first)
- ✅ Uses readability scale
- ❌ No 'niner' pronunciation (LLM dependent)
- ❌ No context memory (each response is independent)
- ❌ May hallucinate airport names from coordinates

---

### 2. Voice Quality

**SayIntentions.AI:**

- 1,500+ unique pilot voices
- 800+ controller voices
- Regional accents based on location
- Human-contributed voice program

**Stratus Offline:**

- Single Piper TTS voice (en_US-lessac-medium)
- Robotic compared to SayIntentions
- No regional variation
- Neural TTS - clear but not human

**Gap Analysis:**
To match SayIntentions, Stratus would need:

- Multiple Piper voice models
- Voice selection based on facility location
- Possibly custom ATC-trained voices

---

### 3. Features Comparison

| Feature | SayIntentions | Stratus Offline |
|---------|---------------|-----------------|
| Radio check responses | ✅ | ✅ |
| Taxi clearances | ✅ Full routing | ⚠️ Basic only |
| Takeoff/Landing clearance | ✅ | ✅ |
| SID/STAR procedures | ✅ | ❌ |
| ILS/RNAV approaches | ✅ | ❌ |
| Traffic advisories | ✅ (AI traffic) | ❌ |
| Frequency handoffs | ✅ | ⚠️ Basic |
| ATIS integration | ✅ | ❌ |
| ACARS/CPDLC | ✅ | ❌ |
| AI Copilot | ✅ | ❌ |
| Student Pilot Mode | ✅ | ❌ |
| Multilingual | ✅ | ❌ |

---

### 4. Architecture Comparison

**SayIntentions.AI:**

```
┌─────────────────┐      ┌─────────────────┐
│  Flight Sim     │─────►│  Cloud Backend  │
│  (MSFS/X-Plane) │◄─────│   (GPT-based)   │
└─────────────────┘      └─────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  88,000 Airports  │
                    │  Real Procedures  │
                    │  Traffic Injection│
                    └───────────────────┘
```

**Stratus Offline:**

```
┌─────────────────┐      ┌─────────────────┐
│   X-Plane       │─────►│  Stratus Client │
│                 │◄─────│   (ATC Brain)   │
└─────────────────┘      └────────┬────────┘
                                  │ D-Bus
                         ┌────────▼────────┐
                         │speechserverdaemon│
                         │  (Ollama/Piper) │
                         └─────────────────┘
```

---

### 5. Strengths of Stratus Offline

| Strength | Description |
|----------|-------------|
| **Privacy** | 100% local - no voice data leaves your machine |
| **No Subscription** | Free to use (only hardware costs) |
| **No Internet Required** | Works completely offline |
| **Open Architecture** | Can swap LLM models (Mistral, Phi, etc.) |
| **Customizable** | Full source access |
| **Low Latency Potential** | Can improve with better local models |

---

### 6. Gaps to Address

| Gap | Priority | Difficulty | Solution |
|-----|----------|------------|----------|
| Airport database | High | Medium | Download FAA airport data, lookup by lat/lon |
| Multiple voices | Medium | Low | Add more Piper voice models |
| SID/STAR procedures | High | High | Integrate with navigation database |
| Context memory | High | Medium | Maintain conversation history |
| Traffic awareness | Low | Very High | Would require AI traffic injection |
| ATIS | Medium | Medium | Generate from METAR data |
| Taxi routing | High | High | Airport taxiway graph database |

---

## Recommendations

### Short Term (1-2 weeks)

1. **Add airport database** - CSV with ICAO codes, names, tower frequencies
2. **Context memory** - Track last 5-10 exchanges for coherent conversation
3. **More voices** - Add different Piper models for variety

### Medium Term (1-2 months)

4. **METAR/ATIS integration** - Generate realistic weather reports
2. **Basic SID/STAR awareness** - Know departure/arrival procedures
3. **Improved prompting** - Fine-tune for specific ATC scenarios

### Long Term (3+ months)

7. **Custom ATC voice model** - Train a Piper voice on controller samples
2. **Navigation database** - Full procedure awareness
3. **Multi-pilot** - Handle other AI traffic (ambitious)

---

## Conclusion

**Stratus Offline ATC is currently at ~40% feature parity with SayIntentions.AI**, but offers unique advantages in privacy, cost, and offline operation.

The core Listen→Think→Speak pipeline works. The primary gaps are:

1. **Airport identification** (needs database, not LLM inference)
2. **Procedure awareness** (SID/STAR/approaches)
3. **Voice variety** (sounds robotic compared to commercial offerings)

For casual VFR practice, Stratus is functional. For realistic IFR training, SayIntentions remains significantly ahead.
