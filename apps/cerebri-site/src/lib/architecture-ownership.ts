// Presentation metadata, checked against workspace manifests by the browser suite.
// Order is the inward inspection path, not a complete dependency graph or runtime trace.
export const runtimeLayers = [
  {
    id: 'transports', concept: 'Structured input', role: 'Transport boundary',
    correspondence: 'Typed requests enter; typed results return.',
    owners: [
      { name: 'cerebri-api', path: 'apps/cerebri-api/Cargo.toml', dependencies: ['cerebri-core'] },
      { name: 'cerebri-node', path: 'bindings/node/Cargo.toml', dependencies: ['cerebri-core'] }
    ],
    seam: 'REST / Node delegate inward to core. They do not own planning semantics.'
  },
  {
    id: 'core', concept: 'Planning entry', role: 'Synchronous facade',
    correspondence: 'plan / validate / temporal inspection',
    owners: [{ name: 'cerebri-core', path: 'crates/cerebri-core/Cargo.toml', dependencies: ['cerebri-types', 'cerebri-planner', 'cerebri-temporal'] }],
    seam: 'Core delegates to planner and temporal. No executor is exposed here.'
  },
  {
    id: 'planner', concept: 'Bounded planning', role: 'Planning ownership',
    correspondence: 'CPIR → compiled snapshot → candidates → proposed placements',
    owners: [{ name: 'cerebri-planner', path: 'crates/cerebri-planner/Cargo.toml', dependencies: ['cerebri-types', 'cerebri-temporal', 'cerebri-constraints', 'cerebri-semantics', 'cerebri-preferences'] }],
    seam: 'Planner consumes domain rules and returns ordered candidate records.'
  },
  {
    id: 'validity', concept: 'Rules & observations', role: 'Domain contracts',
    correspondence: 'Hard validity / semantic state / ranking features',
    owners: [
      { name: 'cerebri-constraints', path: 'crates/cerebri-constraints/Cargo.toml', dependencies: ['cerebri-types', 'cerebri-temporal'] },
      { name: 'cerebri-semantics', path: 'crates/cerebri-semantics/Cargo.toml', dependencies: ['cerebri-types', 'cerebri-temporal'] },
      { name: 'cerebri-preferences', path: 'crates/cerebri-preferences/Cargo.toml', dependencies: ['cerebri-types', 'cerebri-temporal'] }
    ],
    seam: 'These domain crates depend on the independent foundations.'
  },
  {
    id: 'foundations', concept: 'Time & identity', role: 'Independent foundations',
    correspondence: 'Typed identity / temporal primitives',
    owners: [
      { name: 'cerebri-types', path: 'crates/cerebri-types/Cargo.toml', dependencies: [] },
      { name: 'cerebri-temporal', path: 'crates/cerebri-temporal/Cargo.toml', dependencies: [] }
    ],
    seam: 'Neither foundation has a dependency on another Cerebri crate.'
  }
] as const;
