<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# FSLint — Project Topology

## System Architecture

```
                    ┌───────────────────────────────────────────────┐
                    │                  fslint-cli                    │
                    │   scan · query · plugins · enable · disable   │
                    └──────────────────────┬────────────────────────┘
                                           │
                    ┌──────────────────────▼────────────────────────┐
                    │                fslint-core                     │
                    │                                               │
                    │  ┌─────────────┐  ┌────────────┐  ┌────────┐ │
                    │  │  Scanner    │  │  Query     │  │ Cache  │ │
                    │  │  Engine     │  │  Parser    │  │ Layer  │ │
                    │  └──────┬──────┘  └────────────┘  └────────┘ │
                    └─────────│─────────────────────────────────────┘
                              │
                    ┌─────────▼─────────────────────────────────────┐
                    │              PLUGIN ECOSYSTEM                  │
                    │                                               │
                    │  ┌────────────┐ ┌──────────┐ ┌────────────┐  │
                    │  │ git-status │ │ file-age │ │  grouping  │  │
                    │  └────────────┘ └──────────┘ └────────────┘  │
                    │  ┌──────────────┐ ┌──────────────────────┐   │
                    │  │ bundle-check │ │  secret-scanner      │   │
                    │  └──────────────┘ └──────────────────────┘   │
                    │  ┌──────────────┐ ┌──────────────────────┐   │
                    │  │ ai-detection │ │  duplicate-finder    │   │
                    │  └──────────────┘ └──────────────────────┘   │
                    └───────────────────────┬───────────────────────┘
                                            │
                              ┌──────────────▼──────────────┐
                              │  fslint-plugin-api (trait)   │
                              │  fslint-plugin-sdk (helpers) │
                              └──────────────┬──────────────┘
                                             │
                    ┌────────────────────────▼──────────────────────┐
                    │              TARGET FILESYSTEM                 │
                    │    Files · Directories · Bundles · Symlinks    │
                    └───────────────────────────────────────────────┘

                    ┌───────────────────────────────────────────────┐
                    │             REPO INFRASTRUCTURE                │
                    │  Justfile            .machine_readable/        │
                    │  .bot_directives/    0-AI-MANIFEST.a2ml        │
                    │  contractiles/       .github/workflows/        │
                    └───────────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
PLUGIN FOUNDATION
  fslint-plugin-api                 ██████████ 100%    Trait, types, builders, tests, fuzz
  fslint-plugin-sdk                 ████████░░  80%    Path, metadata, patterns, context

CORE ENGINE
  fslint-core (scanner)             ░░░░░░░░░░   0%    Not yet implemented
  fslint-core (query parser)        ░░░░░░░░░░   0%    Not yet implemented
  fslint-core (cache layer)         ░░░░░░░░░░   0%    Not yet implemented
  fslint-cli                        ░░░░░░░░░░   0%    Not yet implemented

PLUGINS
  git-status                        ░░░░░░░░░░   0%    Planned for v0.2.0
  file-age                          ░░░░░░░░░░   0%    Planned for v0.2.0
  grouping                          ░░░░░░░░░░   0%    Planned for v0.2.0
  bundle-check                      ░░░░░░░░░░   0%    Planned for v0.2.0
  secret-scanner                    ░░░░░░░░░░   0%    Planned for v0.3.0
  ai-detection                      ░░░░░░░░░░   0%    Planned for v0.3.0
  duplicate-finder                  ░░░░░░░░░░   0%    Planned for v0.3.0

INFRASTRUCTURE
  Justfile                          ██████████ 100%    Standard build automation
  .machine_readable/                ██████████ 100%    STATE, META, ECOSYSTEM aligned
  0-AI-MANIFEST.a2ml                ██████████ 100%    AI agent entry point
  .bot_directives/                  ██████████ 100%    8 bot constraint files
  .github/workflows/                ██████████ 100%    16 standard workflows

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ██░░░░░░░░  ~25%   Plugin API/SDK stable, engine next
```

## Key Dependencies

```
fslint-cli ──────► fslint-core ──────► fslint-plugin-api
                       │                      ▲
                       ▼                      │
                   plugins/* ──────► fslint-plugin-sdk
                       │
                       ▼
                   Filesystem
```

## Output Flow

```
Filesystem ──► Scanner ──► Plugins ──► Cache ──► Query ──► Formatter ──► User
                                                               │
                                                    ┌──────────┼──────────┐
                                                    ▼          ▼          ▼
                                                  Table      JSON      Simple
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
