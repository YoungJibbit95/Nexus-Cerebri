# Nexus Cerebri — Communication, Explanation & Visual Guide

**Status:** Internal working specification  
**Audience:** Maintainers, documentation agents, website agents, design agents and reviewers  
**Public-facing:** No. This document governs public explanations; its editorial machinery is not public copy.

## 1. Mission

Nexus Cerebri must remain technically serious while becoming understandable at several levels of depth.

The governing rule is:

> **Simplify the explanation, never the truth.**

Public communication is downstream of repository reality. A short explanation, a technical explanation and a diagram are not different stories. They are different-resolution projections of the same system.

The preferred explanatory progression is:

`curiosity → concrete problem → intuition → example → terminology → mechanism → formalization → implementation → evidence`

Do not invert that sequence simply because source code or the workspace is organized differently.

## 2. Source-of-truth hierarchy

Before changing a factual public claim, establish the strongest current authority. Use, approximately, this order:

1. accepted specifications and contracts;
2. current implementation and public types;
3. tests and fixtures that demonstrate observable behavior;
4. accepted ADRs and maintained architecture documentation;
5. release/status/version authorities;
6. maintained reference documentation;
7. README and website prose.

Existing explanatory text is never sufficient evidence for an implementation claim merely because it is already published.

When sources disagree:

- do not choose the most convenient interpretation;
- record the discrepancy;
- follow the strongest current implementation/contract evidence;
- narrow or omit unsupported claims;
- never change runtime semantics merely to make prose true.

## 3. Truth invariants

Every explanation must preserve distinctions that affect behavior or authority, including:

- implemented vs. experimental vs. future behavior;
- deterministic behavior vs. learned behavior;
- fact vs. inference vs. preference;
- hard constraint vs. policy vs. permission;
- candidate generation vs. validation vs. ordering;
- proposal vs. authorization vs. execution;
- absent vs. explicit null where a contract distinguishes them;
- half-open interval semantics and other exact temporal boundaries;
- current provider/mock boundaries vs. production integration;
- current deterministic observations vs. future learning features;
- bounded guarantees vs. broader ambitions.

A surface explanation may omit machinery. It may not collapse a distinction that changes the reader's mental model of what Cerebri does, what it proves, or what it is allowed to change.

## 4. Explanation parity

For every important concept, maintain this dependency direction:

`repository truth → precise technical explanation → intermediate explanation → surface explanation → visual representation`

Each layer must be entailed by the layer before it.

A useful internal review question is: if a technical reader opens the implementation after reading the surface explanation, do they discover more detail about the same system, or a different system?

If the answer is “a different system,” the public explanation is wrong.

## 5. Capability status

Internally classify material claims as:

- **BUILT** — observable in current repository/runtime behavior.
- **EXPERIMENTAL** — active lab or research work that is not a stable product capability.
- **FUTURE / RESEARCH** — intended or explored, not implemented as current behavior.
- **UNCLEAR** — repository evidence is insufficient or contradictory.

Never infer implementation from a package name, roadmap heading or architectural placeholder.

In particular, do not describe neural inference, learned preferences, autonomous mutation, provider-backed execution, natural-language parsing, EvaluationEpisode behavior, learned search/ranking or similar work as current unless repository evidence establishes it.

## 6. Progressive depth

Do not force every reader through one compromised paragraph. Present one truth at increasing depth.

### Level 0 — Orientation

Answer: **What is this about?**

Use ordinary language and no unexplained project vocabulary.

### Level 1 — Intuition

Answer: **Why is this problem nontrivial and what matters?**

Use a recognizable situation.

### Level 2 — Worked example

Let the reader follow one concrete request through the system.

Prefer one canonical example reused throughout the site and README instead of unrelated toy examples.

### Level 3 — Mechanism

Only now introduce technical vocabulary such as facts, hard constraints, preferences, planning scope, candidates, validation, ordering and CPIR.

### Level 4 — Technical depth

Expose exact types, schemas, contracts, interval rules, algorithms, invariants, serialization, failure modes, source paths and tests.

Technical depth must be usable for verification, not decorative complexity.

## 7. Meaning before terminology

The default sequence for a concept is:

1. present a recognizable situation;
2. ask the question the system must answer;
3. establish the intuitive distinction;
4. demonstrate it with a concrete example;
5. visualize the relationship or transformation;
6. introduce the formal term;
7. define it precisely;
8. connect it to current Cerebri implementation;
9. provide code, schema, math and tests when useful.

Do not lead the landing page with acronyms, crate names or architecture labels unless the reader deliberately chose a technical route.

## 8. Questions that organize explanations

Prefer human questions over component names:

- What does the system know?
- What is it not allowed to violate?
- What would be preferable but is not mandatory?
- What part of the world is inside the planning scope?
- What candidates are actually considered?
- Why is one candidate rejected?
- If several candidates are valid, how are they ordered?
- What exactly has been proven by the search assessment?
- What still has to happen before anything can be executed?
- Which information survives when the request becomes structured data?

Introduce the project-specific term after the underlying idea is understandable.

## 9. Canonical example policy

Maintain one repository-backed planning example and reuse it across explanation layers.

The canonical example must:

- exercise only currently supported semantics;
- distinguish educational human wording from actual structured input;
- never imply a free-form NLP parser unless one exists;
- never call a generic buffer “travel” unless travel semantics are actually represented;
- use the current CPIR/planner boundary honestly;
- expose rejected and valid candidates where useful;
- show deterministic ordering without implying learning;
- explain `ProvenOptimal` only for the declared grid and objective;
- preserve exact interval behavior, including half-open boundaries.

The current canonical communication fixture is defined in
[`terminology-and-canonical-example.md`](terminology-and-canonical-example.md).

## 10. Mathematics

Use mathematics as compressed reasoning, not visual authority.

For each meaningful formula, present:

1. the question;
2. the intuition;
3. a small numerical example where useful;
4. the notation;
5. a term-by-term reading;
6. where the math is used in current Cerebri;
7. whether it is implemented, educational background, experimental or future research.

Never place generic neural-network mathematics next to current deterministic planning in a way that makes the viewer infer that the planner is running a neural model.

Historical explanations must distinguish influence, derivation and invention accurately.

## 11. Code and structured data

Code should reveal rather than overwhelm.

Prefer:

`small valid fragment → one additional concept → combined representation → real complete example`

When showing pseudo-data, make its educational role unmistakable through context. When claiming to show the real representation, use the current schema and validate it.

Technical routes should lead to real repository artifacts:

- CPIR/schema documentation;
- Rust types;
- planner implementation;
- constraint and preference implementation;
- representative tests;
- API/bridge contracts;
- lifecycle and execution boundaries.

## 12. Public writing style

Public prose should sound like a knowledgeable person explaining something they understand.

Prefer concrete nouns and verbs, specific limitations, examples before abstractions, active voice where natural and modest confidence backed by evidence.

Avoid:

- generic AI or startup marketing language;
- inflated adjectives without technical substance;
- unexplained acronyms;
- repetitive slogan structures;
- capability claims derived from ambition rather than code;
- public commentary about how accessible or beginner-friendly the copy is;
- phrases such as “in simple terms,” “don't worry about the math,” or “we made this easy.”

The visitor should experience clarity; they should not be told about the editorial strategy.

## 13. Origin story

The project story may communicate that Nexus Cerebri began as a learning project and that following the mathematics behind influential neural-network work helped turn intimidating notation into understandable operations.

Keep historical attribution accurate. If the 1986 backpropagation paper is mentioned, attribute it to David E. Rumelhart, Geoffrey E. Hinton and Ronald J. Williams.

Do not transform that origin story into a claim that the current deterministic planner performs modern neural inference. Research ambition and present implementation are separate facts.

## 14. Landing-page information architecture

The landing page should behave as a guided journey, not a compressed specification.

Recommended sequence:

1. central question / curiosity;
2. concise origin;
3. why planning is a useful learning domain;
4. one canonical planning problem;
5. facts, rules, preferences and scope;
6. candidate exploration;
7. rejection and validation;
8. comparison/ordering;
9. verification and search assessment;
10. introduction of CPIR and architecture vocabulary;
11. technical descent into exact artifacts;
12. explicit current / experimental / research horizon.

The reader should normally encounter a concept's meaning before its project-specific name.

## 15. README information architecture

The repository README serves a different purpose from the landing page. A suitable order is:

1. identity and one-sentence description;
2. why the project exists;
3. what Cerebri actually does today;
4. compact canonical example;
5. truthful high-level flow;
6. status, version and release qualification;
7. quick start;
8. workspace / architecture map;
9. contracts and specifications;
10. testing and verification;
11. research direction;
12. deeper documentation and website links.

Do not delete governance, release, contract or qualification details. Reposition them so they do not become the first conceptual barrier.

## 16. Visual grammar

Build a semantic visual system rather than a gallery of unrelated diagrams.

Recurring objects may include:

- request;
- fact;
- unknown;
- hard constraint;
- preference;
- planning scope;
- candidate;
- ordering signal;
- violation;
- evidence;
- validation;
- result;
- experimental/research signal.

A recurring concept should retain consistent shape, typographic treatment, semantic color role, connection behavior, interaction state, mobile representation and non-color accessibility cue.

Never create attractive categories that do not exist in the system.

Avoid generic “AI” imagery such as glowing brains, robot heads, meaningless neural meshes or Matrix-like backgrounds as a substitute for explanation.

Prefer a visual character closer to an interactive scientific notebook, precise modern operating system, excellent digital textbook or research instrument.

## 17. Progressive diagrams

Do not show the full technical architecture before teaching the pieces that give it meaning.

Prefer one conceptual diagram that grows:

`request → information / rules / preferences → planning → candidates → validation / comparison → result`

Only after that model is understood should it expand into crates, schemas, bridges, APIs and authority boundaries.

A diagram should answer a question. If prose already answers the question more clearly, remove the diagram.

## 18. Semantic motion

Animation is explanatory infrastructure, not decoration.

Meaningful motion should correspond to an operation such as:

- information entering a process;
- a relationship being established;
- a concept being decomposed;
- a candidate being generated or rejected;
- ordering changing;
- validation succeeding or failing;
- an abstraction moving from intuitive to exact representation;
- a local detail expanding into system context.

Decorative ambient motion may exist sparingly, but it must not compete with reading.

Respect `prefers-reduced-motion`. No essential meaning may depend on animation, hover or timing.

## 19. Localization

German and English are first-class versions of the same factual system.

They do not need literal sentence parity. They do need parity for:

- capabilities;
- current / experimental / future status;
- examples;
- technical terminology;
- numbers and units;
- limitations;
- guarantees and non-guarantees;
- technical links.

Use locale/browser language only as an initial choice. Provide a visible switch and persist explicit preference. Do not infer language solely from geography.

## 20. Accessibility and performance

Accessibility is part of explanatory correctness.

Require:

- keyboard-operable interactions;
- meaningful focus states;
- semantic document structure;
- text alternatives for explanatory diagrams;
- non-color status indicators;
- sufficient contrast;
- reduced-motion behavior;
- responsive math and code;
- no essential information available only via hover or animation.

Prefer HTML, CSS and SVG for explanatory graphics where appropriate. Use heavier rendering technology only when it materially improves explanation.

## 21. Review gate for public explanations

Before committing a changed public explanation, answer internally:

1. What exact repository fact supports this sentence?
2. Is the claim BUILT, EXPERIMENTAL, FUTURE / RESEARCH or UNCLEAR?
3. Did simplification remove a distinction that changes behavior?
4. Does the technical explanation describe the same system more precisely?
5. Does the visual imply anything the text does not establish?
6. Do German and English preserve the same factual claim?
7. Could an educational example be mistaken for the real wire format?
8. Could research intent be mistaken for implemented intelligence?
9. Is specialist terminology introduced before the underlying idea?
10. Is editorial meta-language leaking into public copy?

If a question exposes a problem, fix it before committing.

## 22. Change governance

Explanatory content is part of the product contract.

When semantics, schemas, status, authority or user-visible behavior change, review whether the change invalidates:

- landing-page explanations;
- README claims;
- glossary entries;
- diagrams;
- canonical examples;
- code snippets;
- status labels;
- translations.

Maintain a compact traceability map for central public claims rather than duplicating exact technical truth in many places.

## 23. Definition of done

A communication/design slice is complete only when:

- factual claims have been checked against current repository authority;
- simple and technical explanations agree;
- current, experimental and future capabilities are distinguishable;
- terminology follows conceptual grounding;
- examples use supported semantics or are clearly educational abstractions;
- DE/EN versions preserve factual parity;
- visuals encode real distinctions;
- motion has an explanatory purpose;
- accessibility remains intact;
- relevant checks actually executed are reported accurately;
- changed public claims have a reviewable evidence trail;
- the PR remains narrow enough for responsible review.

## 24. Compact constitution

When priorities compete, apply these rules in order:

1. Repository truth before public prose.
2. Preserve current runtime semantics.
3. Keep current capability separate from future ambition.
4. Meaning before terminology.
5. Concrete example before abstraction.
6. Surface explanations must be projections of technical truth.
7. Technical depth must remain available and useful.
8. Every visual must encode meaning.
9. Every meaningful motion must explain a change.
10. Do not tell readers that the writing is accessible; make it clear.
11. Simplify the explanation, never the truth.
