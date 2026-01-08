# Reference Sources for Stratus ATC

> **Version Control Notice**: This document pins the specific versions of FAA documents used as the source of truth for Stratus ATC phraseology and procedures.

## Primary Sources (In-Repo)

All reference material is curated and stored in-repo. **Do not fetch at runtime.**

| Document | Stratus File | Purpose |
|:---------|:-------------|:--------|
| FAA 7110.65 excerpts | `docs/ATC_REFERENCE.md` | ATC procedures & phraseology |
| Canonical exchanges | `docs/PHRASEOLOGY_GUIDE.md` | 30 VFR templates |
| Validation rules | `client/src/core/validation.py` | Code-level enforcement |

## Source Document Versions

### FAA Order 7110.65 (Air Traffic Control)

- **Version Used**: 7110.65Z (current as of 2024)
- **Effective Date**: 2024-02-01
- **Chapters Referenced**:
  - Chapter 2: General Control (readback requirements)
  - Chapter 4: IFR (for scope boundaries)
  - Chapter 7: VFR (primary focus)
- **Source URL**: <https://www.faa.gov/air_traffic/publications/atpubs/atc_html/>

### Aeronautical Information Manual (AIM)

- **Version Used**: AIM 2024
- **Chapters Referenced**:
  - Chapter 4: Air Traffic Control
  - Chapter 5: Air Traffic Procedures
- **Source URL**: <https://www.faa.gov/air_traffic/publications/atpubs/aim_html/>

### Pilot/Controller Glossary

- **Version Used**: 2024 edition
- **Source URL**: <https://www.faa.gov/air_traffic/publications/atpubs/pcg_html/>

## Update Policy

1. Reference documents should be reviewed annually
2. Changes require team review (Captain Martinez, Controller Hayes)
3. All updates must be version-controlled in git
4. Never fetch external documents at runtime

## Scope Boundaries

### Currently Supported (VFR)

- ✓ VFR flight following
- ✓ Traffic advisories
- ✓ Pattern operations (Class D)
- ✓ Taxi and ground operations
- ✓ Uncontrolled field announcements

### Not Supported (Future Phases)

- ✗ IFR clearances and releases
- ✗ SIDs/STARs
- ✗ Instrument approaches (ILS, VOR, RNAV)
- ✗ Holding patterns
- ✗ RVSM operations
- ✗ Oceanic/international procedures

---

*Last Updated: 2026-01-07*
*Reviewed By: Advisory Team*
