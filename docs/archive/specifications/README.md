# Archived Master Specifications

**Historical and non-normative.** Only the [current Master](../../architecture/specifications/master-v0.4.md)
and its Accepted amendments define the current architecture. Terminology and dates inside these
archived files describe their original state, not present software/release or safety guarantees.

On 2026-09-22, Git inspection found exact standalone revisions 0.1, 0.2 and 0.3. They were restored
byte-for-byte from the final committed copy of each revision before its successor, in accordance
with [ADR-0007](../../architecture/decisions/ADR-0007-documentation-layout.md) and the
[archive standard](../../development/documentation-git-release-standard.md).
No historical prose was reconstructed or edited. All source commits are dated 2026-09-19.

| Revision | Source commit and original path | Git blob SHA |
| --- | --- | --- |
| [0.1](master-v0.1.md) | [82e92a6](https://github.com/YoungJibbit95/Nexus-Cerebri/blob/82e92a672cba010bb6b7182614dca6bf80e1e808/docs/00_MASTER_SPECIFICATION_v0.1.md) | `d8eadb58031d6bd4f9c09f66b87df4e5739e9d8b` |
| [0.2](master-v0.2.md) | [13b79e8](https://github.com/YoungJibbit95/Nexus-Cerebri/blob/13b79e8582b1368a69aefb144c156fb27f7d1c41/00_MASTER_SPECIFICATION_v0.2.md) | `c9fe943dc1b831e68119d975ccdf5bd03046462f` |
| [0.3](master-v0.3.md) | [a260f9f](https://github.com/YoungJibbit95/Nexus-Cerebri/blob/a260f9f26b47b797ed1f32689e1bb492d968d35c/00_MASTER_SPECIFICATION_v0.3.md) | `5c8409998e253ade93055cb8c8195edf2c8ae3a3` |

Git history remains the authority for edits within a revision, including later amendments to 0.4.
No duplicate snapshot of the still-current 0.4 revision is added. To verify an archive, compare
its bytes with `git show <source-commit>:<original-path>` from the links above.
