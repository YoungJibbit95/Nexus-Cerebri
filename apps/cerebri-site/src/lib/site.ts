export type TruthKind = 'REAL' | 'EDUCATIONAL' | 'FUTURE CONCEPT';
export type StatusKind = 'Implemented' | 'Foundation' | 'Experimental' | 'Planned' | 'Research' | 'Deferred';

export const sections = {
  explore: {
    eyebrow: 'Cerebri Atlas',
    title: 'Explore the computational coordinate field.',
    summary: 'A map of the concepts that already exist, the boundaries that protect them, and the research layers that are still deliberately separate.',
    technical: 'The Atlas groups temporal primitives, CPIR evidence, constraints, bounded search, validation and execution authorization without collapsing their ownership boundaries.',
    research: 'Future neural and learned layers are shown as adjacent research strata only. They never replace deterministic validity, permission or execution proofs.',
    status: 'Foundation' as StatusKind,
    truth: 'EDUCATIONAL' as TruthKind,
    visual: 'atlas',
    sourcePath: 'docs/architecture/specifications/master-v0.4.md',
    sourceLabel: 'Master Specification 0.4',
    bullets: ['Facts remain distinct from inference.', 'Constraints remain distinct from permissions.', 'Planner output is never execution authority.']
  },
  cpir: {
    eyebrow: 'CPIR Explorer',
    title: 'What information does the planner need?',
    summary: 'To find 30 minutes for an appointment, Cerebri needs its duration, a search window and the times already occupied. CPIR puts those details, rules and planning permissions into separate fields. A calling application supplies this data; the current planner does not read a sentence.',
    technical: 'The new appointment has no assigned time: its time.value is RESOLVED with knowledge MISSING. The duration is KNOWN: 1800 seconds. A missing duration would prevent the search from starting.',
    research: 'This example uses CPIR 0.2. The format also supports temporal source data; older 0.1 requests remain accepted without it. CPIR is still an internal, evolving format. Learned interpretation remains future work.',
    status: 'Implemented' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'cpir',
    sourcePath: 'examples/request.json',
    sourceLabel: 'Read the complete example input',
    bullets: ['The new appointment still needs a time.', 'The existing 09:00–10:00 appointment blocks overlapping starts.', 'Planning permissions do not authorize a calendar change.']
  },
  planning: {
    eyebrow: 'Deterministic Planning Field',
    title: 'Search a declared grid, then say exactly what was proven.',
    summary: 'The current planner performs bounded single-event grid search and ranks feasible candidates deterministically.',
    technical: 'PROVEN_OPTIMAL is only valid after the declared grid is fully traversed with a solution. Candidate limits can reduce the assessment to BEST_FOUND.',
    research: 'Bounded recurrence compilation, dependency/cycle evidence, composed hard constraints and explicit ordering keys are implemented. Joint repair and broader release validation remain future work.',
    status: 'Implemented' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'planning',
    sourcePath: 'crates/cerebri-planner/src/search.rs',
    sourceLabel: 'Rust planner search',
    bullets: ['Discrete grid proof, not continuous global optimality.', 'Tie-breaking is deterministic.', 'The website renders Rust output; it does not search in TypeScript.']
  },
  time: {
    eyebrow: 'Temporal Lens',
    title: 'Treat time as a typed boundary, not a display string.',
    summary: 'Cerebri uses UTC instants, explicit IANA zones and validated half-open intervals, with bounded recurrence diagnostics.',
    technical: 'Gap and fold handling must be explicit. Free time is only known under complete coverage; incomplete coverage leaves the complement unknown.',
    research: 'Existing daily/weekly series compile into bounded CPIR 0.2 occupancy. Monthly/yearly rules, exceptions, prospective series and recurrence constraints remain unsupported.',
    status: 'Implemented' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'time',
    sourcePath: 'docs/architecture/decisions/ADR-0009-bounded-temporal-diagnostics.md',
    sourceLabel: 'ADR-0009',
    bullets: ['Intervals are [start, end).', 'DST ambiguity is never silently guessed.', 'Diagnostics are bounded and completeness-aware.']
  },
  safety: {
    eyebrow: 'Lifecycle Proof Chain',
    title: 'Promotion is earned one proof at a time.',
    summary: 'Planning, validation, translation, authorization and execution are distinct lifecycle stages with stronger types and checks at each boundary.',
    technical: 'Only AuthorizedActionPlan can enter execution. Freshness, content-bound identity, atomic claims and provider revision checks belong to execution preflight.',
    research: 'Durable production ledgers, authenticated providers and reconciliation adapters are intentionally not implemented yet.',
    status: 'Foundation' as StatusKind,
    truth: 'EDUCATIONAL' as TruthKind,
    visual: 'safety',
    sourcePath: 'docs/architecture/decisions/ADR-0005-execution.md',
    sourceLabel: 'ADR-0005',
    bullets: ['Planner ≠ Executor.', 'Validation does not grant permission.', 'Unknown provider outcome requires reconciliation.']
  },
  architecture: {
    eyebrow: 'Architecture',
    title: 'Which part does what?',
    summary: 'Rust checks the appointment details, tests possible times and compares valid options. The API and Node bridge pass requests to that code. The Lab and website show the results. Carrying out a change requires separate validation, permission checks and an executor.',
    technical: 'cerebri-core forwards planning calls to cerebri-planner. The planner owns CPIR and the stages through AuthorizedActionPlan; cerebri-integrations owns execution. An import arrow shows a code dependency, not an execution step.',
    research: 'cerebri-ml defines metadata and interfaces, but the current planner does not call a learned model. Learning experiments in research/ stay outside production dependencies.',
    status: 'Foundation' as StatusKind,
    truth: 'EDUCATIONAL' as TruthKind,
    visual: 'architecture',
    sourcePath: 'docs/en/foundation.md',
    sourceLabel: 'Read the architecture walkthrough',
    bullets: ['Rust calculates; the interfaces display its results.', 'A proposal is not permission to execute.', 'Learning experiments do not run inside the planner.']
  },
  lab: {
    eyebrow: 'Cerebri Lab',
    title: 'Inspect the core without becoming the core.',
    summary: 'Lab is the separate developer and research workspace for planner results, temporal diagnostics, traces and raw structured output.',
    technical: 'The Lab calls the same Rust-backed API routes used by other transports. It contains no JavaScript planner and exposes no execution endpoint.',
    research: 'ML and dataset surfaces are reserved interfaces only; they must not simulate successful inference or training behavior.',
    status: 'Experimental' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'lab',
    sourcePath: 'apps/cerebri-lab/README.md',
    sourceLabel: 'Cerebri Lab README',
    bullets: ['Separate package and route surface.', 'Developer/research UI, not an end-user calendar.', 'No provider mutation controls.']
  },
  roadmap: {
    eyebrow: 'Learning and research',
    title: 'Where could learning help?',
    summary: 'Cerebri began with a wish to understand how neural networks learn. Building the planner makes a related question concrete: could learned suggestions help find useful plans while every option still has to pass explicit checks? This is a research direction; the current planner follows fixed rules.',
    technical: 'Rust already produces deterministic ranking observations. Evaluation Episode Phase A defines synthetic record types and checks their wire format; it does not collect observations or replay runs. The ML crate defines metadata and interfaces, with no trained model in the planner. Software 0.2.0 remains unreleased.',
    research: 'Later experiments could study request interpretation, preferences and search guidance. Each would need reproducible comparisons with the deterministic baseline. Facts, required rules, permissions and execution would remain outside learned authority.',
    status: 'Planned' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'roadmap',
    sourcePath: 'docs/en/research.md',
    sourceLabel: 'Read the learning and research path',
    bullets: ['The current planner does not learn from your choices.', 'Neural-network lessons are planned, not implemented.', 'Future experiments must show what improves and what still fails.']
  },
  developers: {
    eyebrow: 'Developer Surface',
    title: 'One Rust authority, multiple thin transports.',
    summary: 'The synchronous core facade backs REST and the provisional Node process bridge. Public web pages consume generated fixtures, not live runtime calls.',
    technical: 'The API exposes planning, validation and temporal diagnostics under /v1. Execution is intentionally absent from the REST surface.',
    research: 'Native bindings, worker isolation and provider adapters are future transport/application choices, not planner semantics.',
    status: 'Foundation' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'developers',
    sourcePath: 'apps/cerebri-api/src/main.rs',
    sourceLabel: 'Rust API application',
    bullets: ['REST delegates to core.', 'Node bridge is provisional.', 'No public execution endpoint exists.']
  }
} as const;

export type SectionKey = keyof typeof sections;
export const sectionKeys = Object.keys(sections) as SectionKey[];

export const primaryNav: Array<{ href: string; label: string }> = [
  { href: '/explore/', label: 'Explore' },
  { href: '/cpir/', label: 'CPIR' },
  { href: '/planning/', label: 'Planning' },
  { href: '/time/', label: 'Time' },
  { href: '/safety/', label: 'Safety' },
  { href: '/architecture/', label: 'Architecture' },
  { href: '/roadmap/', label: 'Roadmap' },
  { href: '/developers/', label: 'Developers' },
  { href: '/docs/', label: 'Docs' }
];
