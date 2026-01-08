# Guardrails Template

> **Usage:** Copy this template to `GUARDRAILS.md` and customize for your project.
> These are NON-NEGOTIABLE requirements. Code that violates these MUST NOT be merged.

## 1. AIAM: Agent Governance

### No-Touch Zones

- **FORBIDDEN:** Modifying or deleting system-level binaries or configuration outside the workspace.
- **FORBIDDEN:** Deleting project source files based on heuristic assumptions without Explicit Verification (EV).
- **FORBIDDEN:** [Add project-specific forbidden zones]

### Action Risk Tiers

| Tier | Risk Level | Examples |
|:-----|:-----------|:---------|
| **T0** | Safe | Read-only, linting, UI state, `view_file` |
| **T1** | Normal | Incremental code edits, new feature files, test additions |
| **T2** | High-Risk | Dependency changes, API changes, schema migrations |
| **T3** | Restricted | File DELETIONS, binary changes, system configuration |

### Mandatory Verification (EV)

- **REQUIRED:** Before T2/T3 actions, agents MUST use `view_file` or `ls -l` to verify current state.
- **REQUIRED:** All T3 actions must be logged with a `justification` in the audit log.

---

## 2. Input Validation

### Confidence Thresholding

```
// REQUIRED: Reject ambiguous or low-confidence inputs
if confidence < THRESHOLD:
    log_warning("Rejecting low-confidence input")
    return
```

### Rate Limiting

```
// REQUIRED: Prevent command flooding
if time_since_last < MIN_INTERVAL:
    return  // Rate limited
```

---

## 3. Output Validation

### Entity Verification

```
// REQUIRED: Verify entities exist before action
if not entity_exists(target):
    show_error("Entity not found")
    return
```

### Destructive Action Confirmation

```
// REQUIRED: High-risk commands need confirmation
if is_dangerous(command):
    await confirm_with_user("Are you sure?")
```

---

## 4. Error Handling

### No Silent Failures

```
// FORBIDDEN:
try { dangerous_action() } catch { /* ignore */ }

// REQUIRED:
try { dangerous_action() } 
catch (e) { log_error(e); show_user_error(e) }
```

---

## 5. Audit Logging

### Decision Tracking

```
// REQUIRED: Log all significant actions
log_info({
    action: "command_executed",
    source: source,
    confidence: confidence,
    result: result
})
```

---

## 6. Project-Specific Rules

> Add domain-specific guardrails here (e.g., database safety, external API limits, hardware access).

---

## Summary: Critical Invariants

1. [List your project's critical invariants]
2. [These should never be violated]
3. [Keep this list short and memorable]
