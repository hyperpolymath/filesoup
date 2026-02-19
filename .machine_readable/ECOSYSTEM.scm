;; SPDX-License-Identifier: PMPL-1.0-or-later
(ecosystem
  (metadata
    (version "0.1.0")
    (last-updated "2026-02-19"))

  (project
    (name "filesoup")
    (purpose "Cross-platform file system linter that transforms scattered directory trees into logical, metadata-rich bundles. Makes Linux and Windows filesystems as navigable as macOS packages through composable plugin-based intelligence.")
    (role file-system-intelligence))

  (position-in-ecosystem
    (description "FSLint is the file system intelligence layer for the RSR 2026 fleet. It provides the low-level directory analysis that higher-level tools consume for dashboards, CI gating, and automated cleanup."))

  (related-projects
    (project "live-files"
      (relationship orchestration-consumer)
      (description "Orchestration layer that calls FSLint to audit fleet directory health and feed results into the Estate SSG dashboard"))
    (project "formdb"
      (relationship parent-hub)
      (description "FormDB is the parent engine. FSLint can act as a data source for FormDB's file metadata storage"))
    (project "estate-ssg"
      (relationship dashboard-renderer)
      (description "Forth-based SSG that consumes FSLint linting reports to render file system health dashboards"))
    (project "panic-attacker"
      (relationship sibling-scanner)
      (description "Security vulnerability scanner. FSLint's secret-scanner plugin complements panic-attack's deeper analysis"))
    (project "echidna"
      (relationship sibling-prover)
      (description "Proof validation tool. Could verify FSLint bundle-check assertions formally"))
    (project "hypatia"
      (relationship ci-consumer)
      (description "Neurosymbolic CI/CD intelligence. Can use FSLint scan data for repository health metrics"))
    (project "gitbot-fleet"
      (relationship automation-consumer)
      (description "Bot orchestration fleet. Bots can trigger FSLint scans and act on results (e.g., rhodibot for RSR compliance)")))

  (integration-points
    (point "FSLint JSON output consumed by Nickel for validation and Estate SSG for rendering")
    (point "fslint-plugin-api is the stable contract for third-party plugin development")
    (point "fslint scan --format json piped into CI workflows for automated directory health checks")))
