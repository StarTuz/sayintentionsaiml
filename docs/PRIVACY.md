# Privacy & Data Security

Stratus ATC is built **local-first** — your flight data, voice, and AI interactions never leave your machine.

---

## 🔒 Zero Cloud Dependency

| What Stays Local | What's Sent to Cloud |
|:-----------------|:---------------------|
| ✅ Your voice recordings | ❌ Nothing |
| ✅ Flight telemetry | ❌ Nothing |
| ✅ AI conversations | ❌ Nothing |
| ✅ Aircraft position | ❌ Nothing |
| ✅ Frequencies & squawk codes | ❌ Nothing |

**Stratus ATC sends exactly zero bytes to external servers.**

---

## 🧠 Local AI Architecture

Unlike cloud-based ATC solutions, Stratus runs the full AI stack locally:

```
┌─────────────────────────────────────────────────────────┐
│                     Your Computer                        │
│  ┌─────────────────────────────────────────────────────┐ │
│  │  Stratus Client                                     │ │
│  │  • Voice Input → Local Whisper                      │ │
│  │  • AI Brain → Local Ollama                          │ │
│  │  • Voice Output → Local Piper TTS                   │ │
│  └─────────────────────────────────────────────────────┘ │
│                         ↕                                │
│  ┌─────────────────────────────────────────────────────┐ │
│  │  X-Plane / MSFS                                     │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                          ↕
                    ❌ No Internet Required
```

---

## 🆚 Comparison with Cloud Services

| Feature | Stratus ATC | Cloud ATC Services |
|:--------|:------------|:-------------------|
| **Voice data** | Stays on device | Sent to cloud servers |
| **Flight tracking** | Stays on device | May be logged |
| **Subscription** | Free forever | $15-30/month |
| **Works offline** | ✅ Yes | ❌ Requires internet |
| **Data retention** | You control | Vendor policy |
| **Audit logs** | Local only | Cloud-stored |
| **Model choice** | You choose | Fixed by vendor |

---

## 🛡️ Security Guarantees

### No Telemetry

Stratus ATC contains no analytics, tracking, or usage reporting. The application does not:

- Phone home
- Check licenses
- Upload crash reports
- Track usage patterns

### Open Source

The entire codebase is available for inspection:

```bash
git clone https://github.com/your-repo/stratus-atc
```

You can verify every line of code.

### Offline-First Design

Stratus works with **zero network connectivity**:

1. Download Ollama and your preferred AI model
2. Install Stratus ATC
3. Disconnect from internet
4. Fly indefinitely

---

## 📊 Local Logging

Stratus does maintain local logs for your benefit:

| Log | Location | Purpose | Retention |
|:----|:---------|:--------|:----------|
| ATC Responses | `~/.local/share/StratusATC/atc_responses.jsonl` | Training review | User-controlled |
| Latency Metrics | `~/.local/share/StratusATC/latency.jsonl` | Performance tuning | 30 days |
| Error Logs | `~/.local/share/StratusATC/stratus.log` | Debugging | Current session |

**All logs are local-only and can be deleted at any time.**

To disable response logging:

```ini
# config.ini
[privacy]
log_responses = false
```

---

## 🔑 No Account Required

Stratus ATC does not require:

- User registration
- Email address
- Payment information
- License keys

Download, run, fly.

---

## 🌍 True Offline Operation

Perfect for:

- 🏔️ Mountain cabins without internet
- ✈️ Laptops on actual flights
- 🔐 Security-conscious users
- 🌐 Regions with unreliable connectivity
- 💸 Users who don't want subscriptions

---

## 📜 Data Ownership

You own:

- All voice recordings
- All AI model weights you download
- All logs generated
- All configuration files

We cannot:

- Access your data
- Revoke your access
- Require renewals
- Collect analytics

**Your cockpit, your data, your choice.**

---

## Questions?

See [REFERENCE_SOURCES.md](REFERENCE_SOURCES.md) for information about FAA source materials used.
All documentation is version-controlled and inspectable.
