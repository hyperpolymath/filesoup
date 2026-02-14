<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md -- Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-14 -->

# TOPOLOGY -- filesoup

## System Architecture

```
filesoup/
├── .machine_readable/    # RSR state files
├── .github/workflows/    # CI/CD
├── contractiles/         # RSR contractile agreements
├── fslint-plugin-api/    # FSLint plugin API
├── fslint-plugin-sdk/    # FSLint plugin SDK
├── README.adoc           # Overview
└── Justfile              # Task runner
```

## Completion Dashboard

| Component | Status | Progress |
|-----------|--------|----------|
| RSR Structure | Active | `████████░░` 80% |
| fslint-plugin-api | Active | `██████░░░░` 60% |
| fslint-plugin-sdk | Active | `██████░░░░` 60% |
| Documentation | Active | `██████░░░░` 60% |

## Key Dependencies

- RSR Template: `rsr-template-repo`
- fslint-plugin-sdk depends on fslint-plugin-api
