;; SPDX-License-Identifier: PMPL-1.0-or-later
(meta
  (metadata
    (version "0.1.0")
    (last-updated "2026-02-19"))

  (project-info
    (type monorepo)
    (languages (rust))
    (license "PMPL-1.0-or-later")
    (author "Jonathan D.A. Jewell (hyperpolymath) <jonathan.jewell@open.ac.uk>"))

  (architecture-decisions
    (adr "Plugin Architecture"
      (status accepted)
      (context "FSLint needs extensible file intelligence without monolithic coupling")
      (decision "Composable plugin system with trait-based API. Each plugin is an independent crate implementing the Plugin trait. Plugins can be enabled/disabled individually at runtime via config.")
      (consequences "Clean separation of concerns. Third-party plugins possible. Small binary if unused plugins excluded."))

    (adr "Bundle-fication Model"
      (status accepted)
      (context "Linux and Windows scatter files across deep hierarchies. Users see fragments, not logical units. macOS .app bundles show that directories can be treated as single entities.")
      (decision "FSLint introduces a bundle-check plugin that identifies directory structures meeting package criteria and presents them as single logical entities in output.")
      (consequences "Requires defining what constitutes a 'package' on each platform. bundle-check plugin becomes a core differentiator."))

    (adr "Smart Caching"
      (status accepted)
      (context "Re-scanning large directory trees is expensive. Most files do not change between scans.")
      (decision "Cache plugin results keyed on (path, mtime, size) tuple. Unchanged files reuse cached results.")
      (consequences "90%+ cache hit rates on re-scans. Slight memory overhead for cache storage."))

    (adr "Query Engine"
      (status accepted)
      (context "Users need to filter scan results by multiple criteria simultaneously")
      (decision "Space-separated filter syntax: name:, ext:, tag:, size_gt:, size_lt:, newest:, <plugin>:<status>")
      (consequences "Simple to type in CLI. Composable. No complex query language to learn."))

    (adr "Workspace Layout"
      (status accepted)
      (context "Multiple crates need clear boundaries: API, SDK, core engine, CLI, plugins")
      (decision "Rust workspace with fslint-plugin-api, fslint-plugin-sdk at top level. fslint-core, fslint-cli, and plugins/ directory to be added.")
      (consequences "Clear dependency graph. Plugin authors depend only on API and optionally SDK.")))

  (development-practices
    (practice "All Rust code must have SPDX headers")
    (practice "Fuzzing targets for any code handling external input")
    (practice "Plugin trait is the stable public API — changes require semver bump")
    (practice "Offline-first: no network calls unless explicitly configured")))
