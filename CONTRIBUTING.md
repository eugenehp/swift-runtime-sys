# Contributing to swift-runtime-sys

## Parity gate policy

Every pull request must pass all three CI parity jobs before it can be merged:

| Job | Script | Required |
|-----|--------|----------|
| `parity (macos-14)` | `run_parity_matrix.sh` | ✅ required |
| `parity (macos-15)` | `run_parity_matrix.sh` | ✅ required |
| `stress` | `run_parity_stress.sh 3` (PR) / `10` (main) | ✅ required |
| `protocol_dispatch` | `run_protocol_dispatch_matrix.sh` | ✅ required |

Merges are blocked when any required job fails or is skipped.

### Configuring branch protection (repo admin required)

1. Go to **Settings → Branches → Add branch protection rule** for `main`.
2. Enable **Require status checks to pass before merging**.
3. Add the following required status checks:
   - `Parity matrix (macos-14)`
   - `Parity matrix (macos-15)`
   - `Parity stress gate (macOS arm64)`
   - `Protocol dispatch matrix (macOS arm64)`
4. Enable **Require branches to be up to date before merging**.
5. Optionally enable **Restrict pushes that create matching branches** and
   **Do not allow bypassing the above settings**.

## Running parity locally

Run the full parity gate before opening a PR:

```bash
# Full matrix (equivalent to CI parity job)
./scripts/run_parity_matrix.sh

# Stress run (PR budget)
FUZZ_CASES=64 ./scripts/run_parity_stress.sh 3

# Protocol dispatch matrix
./scripts/run_protocol_dispatch_matrix.sh
```

Release floor (before tagging a production release):

```bash
FUZZ_CASES=128 ./scripts/run_parity_stress.sh 100
```

## Reproducing a stress failure

Every stress failure includes the seed and reproduce command in the summary
file (`target/runtime-probe/stress/stress-summary-*.md`):

```
- run 7: command failed; seed=55433 FUZZ_CASES=128;
  reproduce: RUNTIME_FUZZ_SEED=55433 RUNTIME_FUZZ_CASES=128 ./scripts/run_parity_matrix.sh;
  log: target/runtime-probe/stress/run-...-7.log
```

Copy the reproduce command and run it locally to debug the failure.

## Parity scope

The v1 parity scope is frozen in the README (see **Parity Scope (v1)**).
Do not claim parity for experimental or out-of-scope features.
Any change to scope must update the README scope section and PLAN.md.
