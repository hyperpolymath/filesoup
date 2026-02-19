;; SPDX-License-Identifier: PMPL-1.0-or-later
(state
  (metadata
    (version "0.1.0")
    (last-updated "2026-02-19")
    (status active))

  (project-context
    (name "filesoup")
    (purpose "Cross-platform file system linter that transforms scattered directory trees into logical, metadata-rich bundles — making Linux and Windows filesystems as navigable as macOS packages")
    (completion-percentage 25))

  (components
    (component "fslint-plugin-api"
      (status "complete")
      (description "Plugin trait definitions, core types (PluginContext, PluginResult, PluginStatus, PluginError, PluginMetadata), builder methods, unit tests, fuzzing"))
    (component "fslint-plugin-sdk"
      (status "substantially-complete")
      (description "Helper utilities for plugin authors: path helpers, file metadata (age, size formatting), regex pattern matching, context helpers, tests, fuzzing"))
    (component "fslint-core"
      (status "planned")
      (description "Scanning engine with smart caching (path, mtime, size), .gitignore support, configurable max-depth, query parser"))
    (component "fslint-cli"
      (status "planned")
      (description "CLI entry point: scan, query, plugins, enable, disable, config subcommands. Output formats: Table, JSON, Simple"))
    (component "plugins"
      (status "planned")
      (description "Individual plugin crates: git-status, file-age, grouping, bundle-check, version-detection, ocr-status, ai-detection, duplicate-finder, secret-scanner")))

  (route-to-mvp
    (milestone "v0.2.0 Core Engine"
      (tasks
        (task "Implement fslint-core scanning engine with caching" (status pending))
        (task "Implement fslint-cli with scan/query/plugins subcommands" (status pending))
        (task "Implement core plugins: git-status, file-age, grouping, bundle-check" (status pending))
        (task "Add parallel file scanning" (status pending))))
    (milestone "v0.3.0 Extended Intelligence"
      (tasks
        (task "WASM-based plugin runtime" (status pending))
        (task "Plugins: secret-scanner, ai-detection, duplicate-finder" (status pending))
        (task "macOS bundle collapsing (.app as single entity)" (status pending))))
    (milestone "v1.0.0 Stable"
      (tasks
        (task "Shell extension integration (Nautilus, Dolphin, Finder, Explorer)" (status pending))
        (task "Virtual filesystem across disks and cloud storage" (status pending)))))

  (blockers-and-issues
    (issue "fslint-core and fslint-cli not yet implemented — plugin API/SDK exist but no scanner to drive them")
    (issue "Cargo.toml author emails use protonmail — should be open.ac.uk"))

  (critical-next-actions
    (action "Implement fslint-core with scanning engine, query parser, and caching layer")
    (action "Implement fslint-cli with subcommands and output formatters")
    (action "Create plugins/ directory with git-status and file-age as first plugins")
    (action "Fix author email in Cargo.toml files")))
