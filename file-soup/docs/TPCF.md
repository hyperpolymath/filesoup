# Tri-Perimeter Contribution Framework (TPCF)

FSLint uses the **Tri-Perimeter Contribution Framework** for graduated access control and trust management.

## Overview

TPCF provides three distinct perimeters of access, each with different trust levels and permissions:

```
┌─────────────────────────────────────────────┐
│ Perimeter 1: Inner Sanctum (Core Team)     │
│ - Merge to main, releases, security        │
│ - Full repository access                    │
└───────────────┬─────────────────────────────┘
                │
┌───────────────▼─────────────────────────────┐
│ Perimeter 2: Blessed Garden (Trusted)      │
│ - Triage issues, review PRs                 │
│ - Push to feature branches                  │
└───────────────┬─────────────────────────────┘
                │
┌───────────────▼─────────────────────────────┐
│ Perimeter 3: Community Sandbox (Open)      │
│ - Fork, PR, issues, discussions             │
│ - Public participation                       │
└─────────────────────────────────────────────┘
```

## Perimeter 3: Community Sandbox

**Status**: **OPEN** - Anyone can participate

### Permissions

- ✅ Fork the repository
- ✅ Submit pull requests
- ✅ Create issues
- ✅ Participate in discussions
- ✅ Review code (comments only)
- ❌ Direct push access
- ❌ Merge pull requests

### Purpose

Safe experimentation without commitment. Contributors can:

- Test ideas in forks
- Submit experimental PRs
- Get feedback without risk
- Learn the codebase
- Build trust over time

### Progression

Contributors can advance to Perimeter 2 by:

1. **Consistent Contributions**: 3+ months of quality PRs
2. **Community Engagement**: Helpful in issues/discussions
3. **Technical Understanding**: Demonstrated codebase knowledge
4. **Good Judgment**: Thoughtful code reviews and suggestions

## Perimeter 2: Blessed Garden

**Status**: **EARNED** - Requires nomination and vote

### Permissions

- ✅ All Perimeter 3 permissions
- ✅ Triage issues (labels, assignment)
- ✅ Review pull requests (approve/request changes)
- ✅ Push to feature branches
- ✅ Participate in maintainer discussions
- ❌ Merge to main
- ❌ Create releases
- ❌ Access security reports

### Requirements

- **Time**: 3+ months active contribution
- **Quality**: Consistent high-quality PRs
- **Trust**: Demonstrated good judgment
- **Availability**: Responsive to review requests
- **Vote**: Nominated by Perimeter 1, approved by majority

### Responsibilities

- **Triage**: Respond to issues within 7 days
- **Review**: Review assigned PRs within 14 days
- **Mentoring**: Help Perimeter 3 contributors
- **Standards**: Uphold code quality and community guidelines

### Progression

Advance to Perimeter 1 by:

1. **Long-term Commitment**: 6+ months in Perimeter 2
2. **Deep Expertise**: Significant architectural contributions
3. **Leadership**: Mentoring, RFC authorship
4. **Trust**: Unanimous vote from Perimeter 1

## Perimeter 1: Inner Sanctum

**Status**: **CORE TEAM** - Highest trust level

### Permissions

- ✅ All Perimeter 2 permissions
- ✅ Merge pull requests to main
- ✅ Create and publish releases
- ✅ Access security vulnerability reports
- ✅ Modify repository settings
- ✅ Invite new contributors to Perimeter 2

### Requirements

- **Time**: 6+ months in Perimeter 2 (9+ months total)
- **Expertise**: Deep understanding of entire codebase
- **Trust**: Unanimous approval from existing Perimeter 1
- **Commitment**: Ongoing availability for critical decisions

### Responsibilities

- **Releases**: Coordinate and publish versioned releases
- **Security**: Respond to security reports within 48 hours
- **Direction**: Guide project architecture and roadmap
- **Governance**: Participate in major decisions
- **Mentorship**: Develop Perimeter 2 contributors

## Access Control Matrix

| Action | P3 (Sandbox) | P2 (Garden) | P1 (Sanctum) |
|--------|--------------|-------------|--------------|
| Fork repo | ✅ | ✅ | ✅ |
| Create PR | ✅ | ✅ | ✅ |
| File issues | ✅ | ✅ | ✅ |
| Comment | ✅ | ✅ | ✅ |
| Triage issues | ❌ | ✅ | ✅ |
| Review PRs | Comment only | Approve/Block | Approve/Block |
| Push branches | ❌ | Feature only | All branches |
| Merge PRs | ❌ | ❌ | ✅ |
| Create releases | ❌ | ❌ | ✅ |
| Security access | ❌ | ❌ | ✅ |
| Modify settings | ❌ | ❌ | ✅ |

## Decision Making

### P3 Decisions (Community Sandbox)

- **Scope**: Personal forks, experimental PRs
- **Process**: Individual choice
- **Approval**: None required for forks

### P2 Decisions (Blessed Garden)

- **Scope**: Issue triage, PR reviews
- **Process**: Individual judgment
- **Oversight**: P1 can override

### P1 Decisions (Inner Sanctum)

#### Minor Decisions

- **Examples**: Bug fixes, documentation, small features
- **Process**: Single P1 approval
- **Timeline**: Immediate to 7 days

#### Major Decisions

- **Examples**: Breaking changes, new dependencies, architecture
- **Process**: RFC → Discussion → 2/3 P1 vote
- **Timeline**: 14-30 days

#### Critical Decisions

- **Examples**: License changes, governance changes, project direction
- **Process**: RFC → Community input → Unanimous P1 agreement
- **Timeline**: 30-90 days

## Benefits of TPCF

### For Contributors (P3)

- **Low Barrier**: Anyone can contribute
- **Safe Experimentation**: Forks allow risk-free exploration
- **Clear Path**: Visible progression to higher trust
- **Emotional Safety**: No commitment required

### For Trusted Contributors (P2)

- **Meaningful Work**: Triage and review reduce maintainer burden
- **Recognition**: Visible trust and responsibility
- **Influence**: Shape project direction
- **Mentorship**: Guide new contributors

### For Core Team (P1)

- **Distributed Load**: P2 handles triage and review
- **Quality Control**: Only final merges require P1
- **Strategic Focus**: More time for architecture and direction
- **Bus Factor**: Multiple people can handle critical tasks

### For the Project

- **Sustainable**: Prevents maintainer burnout
- **Welcoming**: Clear path for new contributors
- **Quality**: Graduated trust ensures code quality
- **Security**: Critical access limited to most trusted

## Anti-Patterns to Avoid

### Gatekeeping

- ❌ Don't reject PRs without clear explanation
- ❌ Don't impose arbitrary requirements
- ❌ Don't make progression impossible

### Favoritism

- ❌ Don't advance friends without merit
- ❌ Don't block competitors unfairly
- ❌ Don't create in-groups and out-groups

### Stagnation

- ❌ Don't keep people in P2 indefinitely
- ❌ Don't avoid promoting qualified candidates
- ❌ Don't resist new perspectives

## Implementation in FSLint

### GitHub Branch Protection

- **main**: Requires P1 review and approval
- **feature/***: P2+ can push
- **fork/***: P3 submits via PR

### GitHub Teams

- **@fslint/core** (P1): Full access
- **@fslint/trusted** (P2): Triage + review
- **Everyone** (P3): Fork + PR

### Labels

- **p1-required**: Needs P1 review before merge
- **p2-approved**: P2 has reviewed, awaits P1 merge
- **p3-welcome**: Good for new contributors

## Current Status

### Perimeter 1 (Core Team)

- FSLint Contributors (Lead Maintainer)

*Seeking additional P1 members from qualified P2 contributors*

### Perimeter 2 (Trusted Contributors)

*None yet - first P2 members will be promoted from active P3 contributors*

### Perimeter 3 (Community)

**Everyone is welcome!** Join us at:
- GitHub Issues: https://github.com/Hyperpolymath/file-soup/issues
- GitHub Discussions: https://github.com/Hyperpolymath/file-soup/discussions

## FAQ

### Can I go directly from P3 to P1?

Rarely. In exceptional cases (e.g., significant architectural contribution, existing maintainer of similar project), but typically progression is P3 → P2 → P1.

### What if I disagree with a decision?

- **P3**: Discuss in issues/PRs, appeal to P2/P1
- **P2**: Escalate to P1 vote
- **P1**: Follow RFC process for major decisions

### Can I lose access?

Yes, if:
- **Inactivity**: 6+ months without participation → demoted one perimeter
- **Code of Conduct violation**: May result in removal
- **Voluntary**: You can step down anytime

### How long does progression take?

- **P3 → P2**: Typically 3-6 months of active contribution
- **P2 → P1**: Typically 6-12 months in P2
- **Total**: 9-18 months from first PR to P1

But quality matters more than time!

---

**TPCF Version**: 1.0

**Last Updated**: 2025-11-22

**Questions?** Ask in GitHub Discussions or email maintainers@fslint.org
