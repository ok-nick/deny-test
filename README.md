# deny-test

A throwaway fixture for demoing the c2pa-rs dependency-audit workflow
(`.github/workflows/cargo-deny.yml`). It pins deliberately vulnerable and
non-compliant dependencies so `cargo deny` reports a realistic mix of findings.

## What it should report

- **Advisories (one issue each):**
  - `RUSTSEC-2020-0071` — potential segfault in `time`
  - `RUSTSEC-2020-0159` — segfault in `localtime_r` via `chrono`
  - `RUSTSEC-2021-0003` — buffer overflow in `smallvec`
  - `RUSTSEC-2025-0009` / `RUSTSEC-2025-0010` — `ring` (panic / unmaintained)
- **License / bans / sources (one aggregated issue):**
  - `banned` — `num-traits` (explicitly denied in `deny.toml`)
  - `unlicensed` — `ring` (non-SPDX license, no clarification)

## How to see it

Push this to a GitHub repo and either push to `main` or run the **Dependency
audit** workflow from the Actions tab. The async path opens issues labelled
`dependency-audit`. (On a pull request it only runs when the PR touches
`Cargo.toml` / `Cargo.lock` / `deny.toml`, and blocks instead of filing issues.)
