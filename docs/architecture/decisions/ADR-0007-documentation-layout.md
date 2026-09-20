# ADR-0007: Canonical documentation under docs/
Date: 2026-09-20
Status: Accepted; amends the Master archive/location rule only.

## Context
Specifications, reviews and standards crowded the repository root and obscured the implementation.

## Decision
Keep README, CONTRIBUTING, SECURITY, CHANGELOG and AGENTS as root entry points. Store the
current Master in docs/architecture/specifications/, ADRs in docs/architecture/decisions/,
roadmaps and progress in docs/development/, testing standards in docs/testing/, and historical
reviews in docs/archive/reviews/. Preserve relative links and historical/non-normative banners.
[The documentation index](../../README.md) provides a single navigation map.

## Consequences and alternatives
The Master path changes; its revision, content authority and ADR precedence do not.
Root duplicate copies or redirect stubs would create competing authorities, so they are removed.
Generated Pages output follows the new paths and is rebuilt from a clean generated directory.

## Compatibility
Old repository paths need updating in bookmarks. No runtime or CPIR changes.
[Master](../specifications/master-v0.4.md) is synchronized with this location amendment.
