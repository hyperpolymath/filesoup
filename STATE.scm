;;; STATE.scm - Project Checkpoint
;;; file-soup
;;; Format: Guile Scheme S-expressions
;;; Purpose: Preserve AI conversation context across sessions
;;; Reference: https://github.com/hyperpolymath/state.scm

;; SPDX-License-Identifier: MIT OR AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

;;;============================================================================
;;; METADATA
;;;============================================================================

(define metadata
  '((version . "0.1.0")
    (schema-version . "1.0")
    (created . "2025-12-15")
    (updated . "2025-12-17")
    (project . "file-soup")
    (repo . "github.com/hyperpolymath/file-soup")))

;;;============================================================================
;;; PROJECT CONTEXT
;;;============================================================================

(define project-context
  '((name . "file-soup")
    (tagline . "*Cross-platform file system intelligence tool with a Notepad++-style plugin architecture*")
    (version . "0.1.0")
    (license . "MIT OR AGPL-3.0-or-later")
    (rsr-compliance . "gold-target")

    (tech-stack
     ((primary . "Rust")
      (package-management . "Guix (primary) / Nix (fallback)")
      (ci-cd . "GitHub Actions + GitLab CI + Bitbucket Pipelines")
      (security . "CodeQL + OSSF Scorecard + cargo-audit")))))

;;;============================================================================
;;; CURRENT POSITION
;;;============================================================================

(define current-position
  '((phase . "v0.1 - Core Foundation & Security Hardening")
    (overall-completion . 40)

    (components
     ((rsr-compliance
       ((status . "complete")
        (completion . 100)
        (notes . "SHA-pinned actions, SPDX headers, multi-platform CI, OpenSSF Scorecard")))

      (security-hardening
       ((status . "complete")
        (completion . 100)
        (notes . "All GitHub Actions SHA-pinned, permissions declared, deprecated actions replaced")))

      (license-consistency
       ((status . "complete")
        (completion . 100)
        (notes . "Dual license (MIT OR AGPL-3.0-or-later) consistent across Cargo.toml, guix.scm, flake.nix")))

      (documentation
       ((status . "foundation")
        (completion . 40)
        (notes . "README, CLAUDE.md, META/ECOSYSTEM/STATE.scm present")))

      (testing
       ((status . "minimal")
        (completion . 15)
        (notes . "CI/CD scaffolding exists, limited test coverage - tests pass but few written")))

      (core-functionality
       ((status . "in-progress")
        (completion . 35)
        (notes . "Plugin architecture implemented, 8 plugins scaffolded, CLI functional")))))

    (working-features
     ("RSR-compliant CI/CD pipeline with SHA-pinned actions"
      "Multi-platform mirroring (GitHub, GitLab, Bitbucket)"
      "SPDX license headers on all workflow files"
      "Dual license consistency (MIT OR AGPL-3.0-or-later)"
      "Plugin trait system and API"
      "File scanner with directory walker"
      "CLI with table/JSON/simple output"
      "Config system (~/.config/fslint/config.toml)"
      "8 plugin scaffolds (git-status, file-age, grouping, version-detection, ocr-status, ai-detection, duplicate-finder, secret-scanner)"))))

;;;============================================================================
;;; ROUTE TO MVP
;;;============================================================================

(define route-to-mvp
  '((target-version . "1.0.0")
    (definition . "Stable release with all Phase 1 + Phase 2 features, comprehensive tests, security audit")

    (milestones
     ((v0.2
       ((name . "Core Plugins Complete")
        (status . "pending")
        (items
         ("Complete git-status plugin implementation"
          "Complete file-age plugin implementation"
          "Complete grouping plugin implementation"
          "Add unit tests for core plugins (>70% coverage)"
          "Performance baseline benchmarks"))))

      (v0.3
       ((name . "Extended Plugins")
        (status . "pending")
        (items
         ("Complete version-detection plugin"
          "Complete ocr-status plugin"
          "Complete ai-detection plugin"
          "Complete duplicate-finder plugin"
          "Complete secret-scanner plugin"))))

      (v0.5
       ((name . "Performance & Query Engine")
        (status . "pending")
        (items
         ("Smart caching by (path, mtime, size) tuple"
          "--max-depth default to prevent deep recursion"
          "Query engine: name:, ext:, newest: syntax"
          "Lazy plugin execution"
          "Test coverage > 70%"))))

      (v0.8
       ((name . "WASM Plugin Support")
        (status . "pending")
        (items
         ("wasmtime runtime integration"
          "WASM plugin SDK"
          "Plugin sandboxing"
          "Third-party plugin loading"))))

      (v1.0
       ((name . "Production Release")
        (status . "pending")
        (items
         ("Comprehensive test coverage (>80%)"
          "Performance optimization and profiling"
          "Security audit completion"
          "User documentation complete"
          "cargo install fslint ready"
          "Homebrew formula"
          "winget package"))))))))

;;;============================================================================
;;; BLOCKERS & ISSUES
;;;============================================================================

(define blockers-and-issues
  '((critical
     ())  ;; No critical blockers

    (high-priority
     ())  ;; No high-priority blockers - security issues resolved

    (medium-priority
     ((test-coverage
       ((description . "Limited test infrastructure - 0 unit tests currently")
        (impact . "Risk of regressions during development")
        (needed . "Comprehensive test suites for all plugins")))))

    (low-priority
     ((documentation-gaps
       ((description . "API documentation incomplete")
        (impact . "Harder for plugin developers")
        (needed . "Rustdoc comments on public APIs")))))))

;;;============================================================================
;;; CRITICAL NEXT ACTIONS
;;;============================================================================

(define critical-next-actions
  '((immediate
     (("Implement git-status plugin logic" . high)
      ("Implement file-age plugin logic" . high)
      ("Add unit tests for plugin API" . high)))

    (this-week
     (("Complete grouping plugin" . medium)
      ("Add integration tests" . medium)
      ("Benchmark scanner performance" . low)))

    (this-month
     (("Reach v0.2 milestone" . high)
      ("Add caching system" . medium)
      ("Query engine prototype" . medium)))))

;;;============================================================================
;;; SESSION HISTORY
;;;============================================================================

(define session-history
  '((snapshots
     ((date . "2025-12-17")
      (session . "security-hardening")
      (accomplishments
       ("Fixed license inconsistencies (MIT OR AGPL-3.0-or-later across all config)"
        "SHA-pinned all GitHub Actions in 14 workflow files"
        "Added SPDX headers to all workflows"
        "Added permissions declarations to all workflows"
        "Replaced deprecated actions (actions-rs/toolchain -> dtolnay/rust-toolchain)"
        "Replaced deprecated actions (actions/create-release -> softprops/action-gh-release)"
        "Verified cargo check and cargo test pass"
        "Updated STATE.scm with current roadmap"))
      (notes . "Security hardening sprint - OpenSSF Scorecard compliance improved"))

     ((date . "2025-12-15")
      (session . "initial-state-creation")
      (accomplishments
       ("Added META.scm, ECOSYSTEM.scm, STATE.scm"
        "Established RSR compliance"
        "Created initial project checkpoint"))
      (notes . "First STATE.scm checkpoint created via automated script")))))

;;;============================================================================
;;; HELPER FUNCTIONS (for Guile evaluation)
;;;============================================================================

(define (get-completion-percentage component)
  "Get completion percentage for a component"
  (let ((comp (assoc component (cdr (assoc 'components current-position)))))
    (if comp
        (cdr (assoc 'completion (cdr comp)))
        #f)))

(define (get-blockers priority)
  "Get blockers by priority level"
  (cdr (assoc priority blockers-and-issues)))

(define (get-milestone version)
  "Get milestone details by version"
  (assoc version (cdr (assoc 'milestones route-to-mvp))))

;;;============================================================================
;;; EXPORT SUMMARY
;;;============================================================================

(define state-summary
  '((project . "file-soup")
    (version . "0.1.0")
    (overall-completion . 40)
    (next-milestone . "v0.2 - Core Plugins Complete")
    (critical-blockers . 0)
    (high-priority-issues . 0)
    (updated . "2025-12-17")))

;;; End of STATE.scm
