#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/phase-o-optional"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
SUMMARY_JSON="$OUT_DIR/phase-o-optional-signoff.json"
SUMMARY_MD="$OUT_DIR/phase-o-optional-signoff.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# O.10 is no longer classified here: it has been promoted to a required gate
# and is enforced through run_phase_o_signoff.sh + the claim contract.

# O.8 evidence: check if a Rust-owned executor integration path exists and
# consume the deterministic gate artifact if it has been run.
o8_hits="$( (rg -n "tokio|rust-owned executor|rust_executor|swift_task_enqueueGlobal_hook|O\.8|run_o8_rust_executor_gate|runtime_o8_rust_executor_probe|RustExecutorInterop" src scripts examples 2>/dev/null || true) | wc -l | tr -d ' ')"
o8_gate_json="$ROOT/target/runtime-probe/o8-rust-executor/o8-rust-executor-summary.json"
o8_gate_present=0
o8_gate_status="missing"
o8_gate_debug_pass=0
o8_gate_release_pass=0
if [[ -f "$o8_gate_json" ]]; then
  o8_gate_present=1
  read -r o8_gate_status o8_gate_debug_pass o8_gate_release_pass <<EOF
$(python3 - "$o8_gate_json" <<'PY'
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as handle:
    data = json.load(handle)

print(
    data.get('status', 'missing'),
    data.get('debug_pass', 0),
    data.get('release_pass', 0),
)
PY
)
EOF
fi

# O.9 evidence: consume the O.9 distributed watch artifact for capability status,
# then check compiler support for Distributed + repository probes for classification.
o9_watch_json="$ROOT/target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.json"
o9_watch_present=0
o9_watch_status="unknown"
o9_watch_reason=""
o9_implementation_ready=0

if [[ -f "$o9_watch_json" ]]; then
  o9_watch_present=1
  python3 - "$o9_watch_json" <<'PY' > "$TMP_DIR/o9_watch_parsed"
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as handle:
    data = json.load(handle)

# Output as key=value pairs for safe sourcing
watch_status = data.get('watch_status', 'unknown')
watch_reason = data.get('watch_reason', 'not provided').replace('"', '\\"')
impl_ready = "1" if data.get('o9_implementation_ready', False) else "0"

print(f'o9_watch_status="{watch_status}"')
print(f'o9_watch_reason="{watch_reason}"')
print(f'o9_implementation_ready={impl_ready}')
PY
  source "$TMP_DIR/o9_watch_parsed"
fi

cat > "$TMP_DIR/o9_distributed_probe.swift" <<'EOF'
import Distributed

distributed actor O9ProbeActor {
    distributed func ping() async -> Int { 1 }
}
EOF
if swiftc -typecheck "$TMP_DIR/o9_distributed_probe.swift" >/dev/null 2>&1; then
  o9_module_supported=1
else
  o9_module_supported=0
fi
o9_hits="$( (rg -n "distributed actor|import Distributed|O\.9" src scripts examples 2>/dev/null || true) | wc -l | tr -d ' ')"

# Classification policy (Wave O7-O10 updated):
# - O.8 remains experimental unless concrete integration artifacts exist.
# - O.9 classification now driven by watch artifact status:
#   - If watch is green (SUPPORTED), promote to optional.
#   - If watch is partial or unsupported, keep as experimental/not-promoted.
if [[ "$o8_gate_present" -eq 1 ]]; then
  o8_classification="experimental"
  o8_promotion="not-promoted"
  if [[ "$o8_gate_status" == "PASS" ]]; then
    o8_reason="deterministic gate artifact present and passing; retained as experimental until promotion policy explicitly upgrades O.8"
  else
    o8_reason="deterministic gate artifact present but not green; retained as experimental"
  fi
elif [[ "$o8_hits" -gt 0 ]]; then
  o8_classification="experimental"
  o8_promotion="not-promoted"
  o8_reason="initial O.8 scaffolding detected; deterministic gate artifact not yet generated"
else
  o8_classification="experimental"
  o8_promotion="not-promoted"
  o8_reason="no Rust-owned executor integration artifacts found"
fi

if [[ "$o9_watch_present" -eq 1 ]] && [[ "$o9_watch_status" == "SUPPORTED" ]]; then
  o9_classification="optional"
  o9_promotion="not-promoted"
  o9_reason="Watch artifact shows SUPPORTED: both typecheck and runtime library available; O.9 implementation work can begin"
elif [[ "$o9_watch_present" -eq 1 ]] && [[ "$o9_watch_status" == "PARTIAL" ]]; then
  o9_classification="experimental"
  o9_promotion="not-promoted"
  o9_reason="Watch artifact shows PARTIAL: $o9_watch_reason; defer O.9 until full support available"
elif [[ "$o9_watch_present" -eq 1 ]]; then
  o9_classification="experimental"
  o9_promotion="not-promoted"
  o9_reason="Watch artifact shows UNSUPPORTED: $o9_watch_reason; O.9 blocked on host capability"
elif [[ "$o9_module_supported" -eq 1 ]]; then
  o9_classification="optional"
  o9_promotion="not-promoted"
  o9_reason="Distributed module/typecheck supported (legacy probe); no watch artifact yet"
else
  o9_classification="experimental"
  o9_promotion="not-promoted"
  o9_reason="Distributed module/typecheck unsupported on host toolchain; no watch artifact"
fi

cat > "$SUMMARY_JSON" <<EOF
{
  "timestamp": "${STAMP}",
  "phase": "O-optional",
  "result": "PASS",
  "tracks": {
    "o8_rust_owned_executor": {
      "classification": "${o8_classification}",
      "promotion_status": "${o8_promotion}",
      "evidence": {
        "repo_marker_hits": ${o8_hits},
        "gate_artifact_present": ${o8_gate_present},
        "gate_status": "${o8_gate_status}",
        "gate_debug_pass": ${o8_gate_debug_pass},
        "gate_release_pass": ${o8_gate_release_pass}
      },
      "reason": "${o8_reason}"
    },
    "o9_distributed_actor_surface": {
      "classification": "${o9_classification}",
      "promotion_status": "${o9_promotion}",
      "evidence": {
        "watch_artifact_present": ${o9_watch_present},
        "watch_status": "${o9_watch_status}",
        "watch_reason": "${o9_watch_reason}",
        "implementation_ready": ${o9_implementation_ready},
        "module_typecheck_supported": ${o9_module_supported},
        "repo_marker_hits": ${o9_hits}
      },
      "reason": "${o9_reason}"
    }
  }
}
EOF

cat > "$SUMMARY_MD" <<EOF
# Phase O Optional Tracks Signoff

- timestamp: ${STAMP}
- result: PASS

| Track | Classification | Promotion | Evidence | Rationale |
|---|---|---|---|---|
| O.8 Rust-owned executor | ${o8_classification} | ${o8_promotion} | repo_marker_hits=${o8_hits}, gate_artifact_present=${o8_gate_present}, gate_status=${o8_gate_status}, debug_pass=${o8_gate_debug_pass}, release_pass=${o8_gate_release_pass} | ${o8_reason} |
| O.9 Distributed actor surface | ${o9_classification} | ${o9_promotion} | watch_present=${o9_watch_present}, watch_status=${o9_watch_status}, implementation_ready=${o9_implementation_ready}, module_typecheck=${o9_module_supported}, repo_marker_hits=${o9_hits} | ${o9_reason} |

O.10 Observation runtime surface is promoted to required on this host cell and is validated by the Phase O required signoff plus the run_o10_observation_gate.sh artifact.
Promotion rule: optional/experimental tracks do not block required parity until explicitly promoted and gated.
EOF

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"
echo "Phase O optional tracks signoff PASS"
