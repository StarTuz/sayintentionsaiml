# ATC Reference Document

> **Stratus ATC Source of Truth**
> Based on FAA Order 7110.65 and Aeronautical Information Manual (AIM)
> Version: 1.0 | Last Updated: 2026-01-07

## Scope

Stratus currently supports **VFR operations only**:

- VFR Flight Following
- Traffic Advisories
- Pattern Operations (Class D)
- Uncontrolled Field Operations

IFR clearances, approaches, and departures are **out of scope** for this phase.

---

## 1. Airspace Classes (Relevant to VFR)

| Class | ATC Service | Communication | Entry Requirements |
|:------|:------------|:--------------|:-------------------|
| **B** | Mandatory | Required | Clearance required |
| **C** | Mandatory | Required | Two-way contact |
| **D** | Mandatory | Required | Two-way contact |
| **E** | Available | Available | None (VFR) |
| **G** | None | None | None |

### Controller Role by Airspace

- **Class B/C**: Approach/Departure Control (TRACON)
- **Class D**: Tower
- **Class E (surface)**: Tower or FSS
- **Class G**: UNICOM/CTAF (pilot self-announce)

---

## 2. VFR Communication Sequence

### 2.1 Initial Contact (Cold Call)

```
PILOT: "[Facility], [Callsign], [Position], [Request]"
ATC:   "[Callsign], [Facility], go ahead" OR "standby"
```

**Example**:

```
PILOT: "NorCal Approach, Cessna 12345, 10 miles south of Sacramento, VFR request"
ATC:   "Cessna 12345, NorCal Approach, go ahead"
```

### 2.2 Flight Following Request

```
PILOT: "[Callsign], [Type], [Position], [Altitude], request flight following to [Destination]"
ATC:   "[Callsign], squawk [Code]" (then later) "radar contact, [Position]"
```

**Key Points** (FAA 7110.65 4-1-18):

- ATC assigns squawk FIRST
- Radar contact confirmation comes SECOND
- Altimeter setting given WITH or AFTER radar contact

### 2.3 Radar Contact

```
ATC: "[Callsign], radar contact, [Position]. Altimeter [Setting]"
```

### 2.4 Traffic Advisories

```
ATC: "[Callsign], traffic [Clock position], [Distance], [Direction], [Type if known], [Altitude]"
PILOT: "Traffic in sight" OR "Negative contact, [Callsign]"
```

**Example**:

```
ATC:   "Cessna 12345, traffic 2 o'clock, 3 miles, southbound, altitude indicates 4,500"
PILOT: "Traffic in sight, Cessna 12345"
```

### 2.5 Service Termination

```
PILOT: "[Callsign], field in sight, cancel flight following"
ATC:   "[Callsign], radar service terminated, squawk VFR, frequency change approved"
```

---

## 3. Pattern Operations (Class D)

### 3.1 Inbound Sequence

1. Contact Tower 10-15 miles out
2. Report position and intentions
3. Receive pattern entry instructions
4. Report pattern position (downwind, base, final)
5. Receive landing clearance

### 3.2 Pattern Entry Phraseology

```
PILOT: "[Tower], [Callsign], [Position], inbound for landing with [ATIS]"
ATC:   "[Callsign], [Tower], enter [left/right] [downwind/base/straight-in] runway [XX]"
```

### 3.3 Landing Clearance

```
ATC: "[Callsign], cleared to land runway [XX]" 
     OR
     "[Callsign], cleared for the option runway [XX]"
```

**Distinction** (7110.65 3-10-4):

- "Cleared to land" = land and exit
- "Cleared for the option" = pilot's choice (touch-and-go, stop-and-go, low approach, full stop)

### 3.4 Go-Around

```
ATC:   "[Callsign], go around"
PILOT: "Going around, [Callsign]"
```

---

## 4. Ground Operations

### 4.1 Taxi Clearances

```
ATC: "[Callsign], taxi to runway [XX] via [Route], hold short of runway [YY]"
```

**Critical Rule** (7110.65 3-7-2):

- ALL taxi instructions that cross or hold short of a runway require readback
- "Hold short of runway XX" MUST be read back verbatim

### 4.2 Takeoff Clearance

```
ATC:   "[Callsign], runway [XX], cleared for takeoff"
PILOT: "Cleared for takeoff runway [XX], [Callsign]"
```

**Never say**: "Cleared to takeoff" (ground or clearance)

---

## 5. Mandatory Readbacks (7110.65 2-4-3)

Pilots MUST read back:

1. ✅ Hold short instructions
2. ✅ Runway assignments (takeoff/landing)
3. ✅ Altimeter settings (IFR)
4. ✅ Frequency changes
5. ✅ Transponder codes

Acknowledgment only ("Roger", "Wilco"):

- Traffic advisories
- Radar contact notification
- Weather information

---

## 6. Standard Phraseology

### Numbers

| Spoken | Written |
|:-------|:--------|
| "niner" | 9 |
| "tree" | 3 |
| "fife" | 5 |
| "point" | . (decimal) |

### Altitudes

- Below 18,000: "four thousand five hundred" (4,500)
- At/above 18,000: "flight level tree five zero" (FL350)

### Headings

- "Fly heading two seven zero" (270°)
- Always three digits

### Frequencies

- "Contact approach on one two four point five" (124.5)
- "Monitor ATIS on one two seven point eight five" (127.85)

---

## 7. Emergency Codes

| Code | Meaning | ATC Response |
|:-----|:--------|:-------------|
| 7500 | Hijack | Acknowledge without alerting |
| 7600 | NORDO (radio failure) | Light gun signals |
| 7700 | General emergency | Priority handling |
| 1200 | VFR (default) | Normal |

---

## 8. Out of Scope (Future Phases)

The following are NOT currently supported:

- IFR clearances and releases
- SIDs/STARs
- Approaches (ILS, VOR, RNAV)
- Holding patterns
- RVSM operations
- Oceanic procedures
- Military operations

---

## References

1. **FAA Order 7110.65Z** - Air Traffic Control (current as of 2024)
2. **Aeronautical Information Manual (AIM)** - Chapter 4: Air Traffic Control
3. **Pilot/Controller Glossary** - FAA
4. **AC 90-66B** - Non-Towered Airport Flight Operations

> **Note**: This document is a curated subset for simulation purposes.
> Real-world operations require complete FAA documentation.
