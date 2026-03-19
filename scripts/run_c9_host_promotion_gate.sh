#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
C8_DIR="$OUT_DIR/c8-host-reliability"
OUT_JSON="$OUT_DIR/c9-host-promotion-signoff.json"
OUT_MD="$OUT_DIR/c9-host-promotion-signoff.md"
HISTORY_WINDOW="${C9_HISTORY_WINDOW:-1}"

mkdir -p "$OUT_DIR"
cd "$ROOT"

python3 - "$ROOT/PLAN.md" "$ROOT/README.md" "$OUT_JSON" "$OUT_MD" "$C8_DIR" "$HISTORY_WINDOW" <<'PY'
import json
import pathlib
import re
import sys

plan_path = pathlib.Path(sys.argv[1])
readme_path = pathlib.Path(sys.argv[2])
out_json = pathlib.Path(sys.argv[3])
out_md = pathlib.Path(sys.argv[4])
c8_dir = pathlib.Path(sys.argv[5])
history_window = int(sys.argv[6])

plan_text = plan_path.read_text(encoding="utf-8")
readme_text = readme_path.read_text(encoding="utf-8")

host_marker = "## Phase C: Host-Cell Deepening"
host_idx = plan_text.find(host_marker)
if host_idx == -1:
    raise SystemExit("missing host-cell Phase C marker in PLAN.md")
host_plan_text = plan_text[host_idx:]

required_classification = {
    "C.1": "required",
    "C.2": "required",
    "C.3": "required",
    "C.4": "required",
    "C.5": "required",
    "C.6": "required",
    "C.7": "required",
    "C.8": "required",
    "C.9": "required",
}

rows = []
issues = []

for phase in required_classification:
    section_pat = re.compile(rf"^### {re.escape(phase)}\).*", re.MULTILINE)
    section_match = section_pat.search(host_plan_text)
    if not section_match:
        rows.append({"phase": phase, "classification": required_classification[phase], "status": "missing", "result": "FAIL"})
        issues.append(f"missing section for {phase} in PLAN.md")
        continue

    next_section = re.search(r"^### C\.\d+\)", host_plan_text[section_match.end():], re.MULTILINE)
    end_idx = section_match.end() + (next_section.start() if next_section else len(host_plan_text[section_match.end():]))
    section_text = host_plan_text[section_match.start():end_idx]
    complete = "**Status**: COMPLETE" in section_text if phase != "C.9" else True

    if phase != "C.9" and not complete:
        issues.append(f"{phase} is not marked COMPLETE")

    rows.append(
        {
            "phase": phase,
            "classification": required_classification[phase],
            "status": "complete" if complete else "incomplete",
            "result": "PASS" if complete else "FAIL",
        }
    )

required_readme_markers = [
    "## Host-Cell Coverage Classification (Phase C)",
    "| C.1 | required |",
    "| C.8 | required |",
    "| C.9 | required |",
]
for marker in required_readme_markers:
    if marker not in readme_text:
        issues.append(f"README missing marker: {marker}")

c8_summary = c8_dir / "c8-host-reliability-summary.json"
if not c8_summary.exists():
    issues.append("missing C.8 summary artifact")
else:
    summary_data = json.loads(c8_summary.read_text(encoding="utf-8"))
    if summary_data.get("result") != "PASS":
        issues.append("C.8 summary result is not PASS")

history_dir = c8_dir / "history"
history_files = sorted(history_dir.glob("*.json"), reverse=True)
if len(history_files) < history_window:
    issues.append(f"insufficient C.8 history snapshots: have {len(history_files)} need {history_window}")
else:
    for h in history_files[:history_window]:
        data = json.loads(h.read_text(encoding="utf-8"))
        if data.get("result") != "PASS":
            issues.append(f"non-green C.8 history snapshot: {h.name}")

overall = "PASS" if not issues else "FAIL"

payload = {
    "version": 1,
    "history_window": history_window,
    "classification": required_classification,
    "rows": rows,
    "issues": issues,
    "result": overall,
}
out_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# C.9 Host Promotion Signoff",
    "",
    f"- history_window: {history_window}",
    f"- c8_summary: {c8_summary}",
    f"- result: {overall}",
    "",
    "| Phase | Classification | Status | Result |",
    "|---|---|---|---|",
]
for row in rows:
    lines.append(f"| {row['phase']} | {row['classification']} | {row['status']} | {row['result']} |")

lines += ["", "## Validation Issues"]
if issues:
    lines.extend([f"- {issue}" for issue in issues])
else:
    lines.append("- none")

out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"Wrote {out_json}")
print(f"Wrote {out_md}")

if issues:
    raise SystemExit(1)
PY

echo "C.9 host promotion gate PASS"
