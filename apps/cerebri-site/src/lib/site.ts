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
    title: 'Make planning evidence explicit.',
    summary: 'CPIR 0.2 adds bounded temporal source state while retaining separate scope, knowledge, provenance, constraints, preferences, policy and capability. Legacy 0.1 inputs remain accepted.',
    technical: 'Resolved processing state and epistemic knowledge state are intentionally separate. Omitted, empty and populated scope filters retain different semantics.',
    research: 'The schema remains internal: 0.2 accepts temporal source state; 0.1 is retained for legacy inputs and the website example. Neither is a stable v1 promise.',
    status: 'Implemented' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'cpir',
    sourcePath: 'examples/request.json',
    sourceLabel: 'Canonical CPIR fixture',
    bullets: ['Schemas 0.1 and 0.2 are validated explicitly.', 'MISSING is not the same as UNRESOLVED.', 'Capability does not silently expand planning scope.']
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
    eyebrow: 'Architecture Constellation',
    title: 'Keep authority directional and inspectable.',
    summary: 'Independent temporal/types foundations feed semantics, constraints and planning; core exposes a thin synchronous facade; transports delegate.',
    technical: 'Repository checks enforce workspace dependency direction and keep research outside production membership. Core owns no database, HTTP client or provider schema.',
    research: 'Future adapters may become asynchronous when real I/O justifies it, without moving domain truth into transports.',
    status: 'Foundation' as StatusKind,
    truth: 'EDUCATIONAL' as TruthKind,
    visual: 'architecture',
    sourcePath: 'docs/architecture/decisions/ADR-0001-workspace.md',
    sourceLabel: 'ADR-0001',
    bullets: ['Core stays transport-neutral.', 'Research never becomes a hidden production dependency.', 'Frontend clients consume typed outputs.']
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
    eyebrow: 'Roadmap Strata',
    title: 'Implemented foundations below, research horizons above.',
    summary: 'The roadmap deliberately sequences deterministic planner depth before provider integration, repair, learning and neural planning.',
    technical: 'Software remains unreleased 0.2.0. Bounded deterministic schedule/graph/recurrence oracles and malformed-input campaigns now cover the planner baseline. Independent review and release qualification remain before 0.3.0.',
    research: 'Dates are planning targets, not promises; releases remain gated by repository truth and verified CI.',
    status: 'Planned' as StatusKind,
    truth: 'REAL' as TruthKind,
    visual: 'roadmap',
    sourcePath: 'docs/development/roadmap/README.md',
    sourceLabel: 'Current roadmap',
    bullets: ['0.2.0 Temporal Core is implemented.', 'Planner verification is implemented; release qualification is next.', 'Providers, learning and neural layers remain later milestones.']
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
