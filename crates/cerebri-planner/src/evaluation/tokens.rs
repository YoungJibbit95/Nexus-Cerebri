//! Closed architecture tokens (consolidated contract section 7).
//! Explicit strings are authoritative; enum names and discriminants are not.
use serde::{Deserialize, Serialize};

macro_rules! tokens {
    ($name:ident { $($variant:ident => $token:literal),+ $(,)? }) => {
        #[doc = concat!("Closed canonical token registry for ", stringify!($name), ".")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name {
            $(#[serde(rename = $token)] $variant),+
        }
        impl $name {
            /// The exact architecture-owned token, independent of Rust naming.
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $token),+ }
            }
        }
    };
}

tokens!(OperationToken {
    Create => "CREATE",
    Move => "MOVE",
    Update => "UPDATE",
    Cancel => "CANCEL",
    FindSlot => "FIND_SLOT",
    Reschedule => "RESCHEDULE",
    Optimize => "OPTIMIZE",
    Plan => "PLAN",
    Analyze => "ANALYZE",
});

tokens!(ProvenanceToken {
    UserExplicit => "USER_EXPLICIT",
    IntegrationFact => "INTEGRATION_FACT",
    SystemFact => "SYSTEM_FACT",
    PersonalLearned => "PERSONAL_LEARNED",
    GlobalLearned => "GLOBAL_LEARNED",
    ModelInference => "MODEL_INFERENCE",
    Default => "DEFAULT",
});

tokens!(PreferenceSourceToken {
    ExplicitCurrentRequest => "EXPLICIT_CURRENT_REQUEST",
    SessionContext => "SESSION_CONTEXT",
    PersonalLearned => "PERSONAL_LEARNED",
    GlobalLearned => "GLOBAL_LEARNED",
    Default => "DEFAULT",
});

tokens!(MutationKindToken {
    CreateEvent => "CREATE_EVENT",
    MoveEvent => "MOVE_EVENT",
    UpdateEvent => "UPDATE_EVENT",
    DeleteEvent => "DELETE_EVENT",
});

tokens!(DeploymentModeToken {
    Local => "LOCAL",
    Test => "TEST",
    Shadow => "SHADOW",
    Suggestion => "SUGGESTION",
    Confirmation => "CONFIRMATION",
    LimitedAutomation => "LIMITED_AUTOMATION",
});

tokens!(PlanningObjectKindToken {
    Event => "EVENT",
    Task => "TASK",
    Deadline => "DEADLINE",
    Availability => "AVAILABILITY",
    Resource => "RESOURCE",
});

tokens!(KnowledgeStateToken {
    Known => "KNOWN",
    Missing => "MISSING",
    Unknown => "UNKNOWN",
    Uncertain => "UNCERTAIN",
    Ambiguous => "AMBIGUOUS",
    Unresolved => "UNRESOLVED",
});

tokens!(FactValueKindToken {
    ScheduledTime => "SCHEDULED_TIME",
    Deadline => "DEADLINE",
    ExternalLock => "EXTERNAL_LOCK",
    Availability => "AVAILABILITY",
});

tokens!(HardConstraintKindToken {
    NoOverlap => "NO_OVERLAP",
    ExplicitTime => "EXPLICIT_TIME",
    ExplicitDate => "EXPLICIT_DATE",
    EarliestStart => "EARLIEST_START",
    LatestEnd => "LATEST_END",
    Deadline => "DEADLINE",
    MinDuration => "MIN_DURATION",
    FixedDuration => "FIXED_DURATION",
    AvailabilityWindow => "AVAILABILITY_WINDOW",
    DependencyOrder => "DEPENDENCY_ORDER",
    RequiredBuffer => "REQUIRED_BUFFER",
    RecurrenceRule => "RECURRENCE_RULE",
    TimezoneIntegrity => "TIMEZONE_INTEGRITY",
    ExternalLock => "EXTERNAL_LOCK",
});

tokens!(CoverageToken {
    Complete => "COMPLETE",
    Incomplete => "INCOMPLETE",
});

tokens!(RecurrenceFrequencyToken {
    Daily => "DAILY",
    Weekly => "WEEKLY",
});

tokens!(WeekdayToken {
    Monday => "MONDAY",
    Tuesday => "TUESDAY",
    Wednesday => "WEDNESDAY",
    Thursday => "THURSDAY",
    Friday => "FRIDAY",
    Saturday => "SATURDAY",
    Sunday => "SUNDAY",
});

tokens!(GapPolicyToken {
    Reject => "REJECT",
    Skip => "SKIP",
});

tokens!(FoldPolicyToken {
    Reject => "REJECT",
    Earlier => "EARLIER",
    Later => "LATER",
});

tokens!(SeriesStateToken {
    Existing => "EXISTING",
    Prospective => "PROSPECTIVE",
});

tokens!(EntityPresenceToken {
    Declared => "DECLARED",
    ReferenceOnly => "REFERENCE_ONLY",
});

tokens!(PlannerAdmissionClassToken {
    Valid => "VALID",
    ValidWithUncertainty => "VALID_WITH_UNCERTAINTY",
    InsufficientInformation => "INSUFFICIENT_INFORMATION",
});

tokens!(PlanningOutcomeToken {
    Solution => "SOLUTION",
    NoSolution => "NO_SOLUTION",
    NeedsRelaxation => "NEEDS_RELAXATION",
    InsufficientInformation => "INSUFFICIENT_INFORMATION",
});

tokens!(SearchAssessmentToken {
    ProvenOptimal => "PROVEN_OPTIMAL",
    Complete => "COMPLETE",
    BestFound => "BEST_FOUND",
});

tokens!(ProofClaimToken {
    None => "NONE",
    CompleteTraversal => "COMPLETE_TRAVERSAL",
    ProvenOptimalDeterministicObjective => "PROVEN_OPTIMAL_DETERMINISTIC_OBJECTIVE",
});

tokens!(PlannerRunUseToken {
    NewRun => "NEW_RUN",
    ReusedRun => "REUSED_RUN",
});

tokens!(LineageTransitionReasonToken {
    ContextChange => "CONTEXT_CHANGE",
    Replan => "REPLAN",
    NewGeneratorRun => "NEW_GENERATOR_RUN",
    NewVisiblePolicyApplication => "NEW_VISIBLE_POLICY_APPLICATION",
});

tokens!(ArtifactClassToken {
    PlannerAlgorithm => "PLANNER_ALGORITHM",
    GeneratorPolicy => "GENERATOR_POLICY",
    PlannerConfiguration => "PLANNER_CONFIGURATION",
    ProductEligibilityPolicy => "PRODUCT_ELIGIBILITY_POLICY",
    RankingPolicy => "RANKING_POLICY",
    DisplayPolicy => "DISPLAY_POLICY",
    Model => "MODEL",
    SourceSnapshot => "SOURCE_SNAPSHOT",
    SyntheticFixture => "SYNTHETIC_FIXTURE",
});

tokens!(ArtifactRoleToken {
    PrimarySource => "PRIMARY_SOURCE",
    DependencySource => "DEPENDENCY_SOURCE",
    Config => "CONFIG",
    Lockfile => "LOCKFILE",
    Schema => "SCHEMA",
    CompiledRuntime => "COMPILED_RUNTIME",
    ModelWeights => "MODEL_WEIGHTS",
    ModelConfig => "MODEL_CONFIG",
    Vocabulary => "VOCABULARY",
    SnapshotData => "SNAPSHOT_DATA",
    TestFixture => "TEST_FIXTURE",
});

tokens!(DataScope {
    Synthetic => "SYNTHETIC",
});

tokens!(RevisionEntityTypeV1 {
    PlanningObject => "PLANNING_OBJECT",
    TemporalSeries => "TEMPORAL_SERIES",
});

tokens!(IdentityTypeV1 {
    PlanningObjectId => "PLANNING_OBJECT_ID",
    FactId => "FACT_ID",
    CalendarId => "CALENDAR_ID",
    IntegrationId => "INTEGRATION_ID",
    ResourceId => "RESOURCE_ID",
    SeriesId => "SERIES_ID",
});

tokens!(GraphVertexTypeToken {
    Object => "OBJECT",
    Fact => "FACT",
    Calendar => "CALENDAR",
    Integration => "INTEGRATION",
    Resource => "RESOURCE",
    Series => "SERIES",
});

tokens!(GraphEdgeTypeToken {
    DependsOnObject => "DEPENDS_ON_OBJECT",
    FactSubject => "FACT_SUBJECT",
    ObjectCalendar => "OBJECT_CALENDAR",
    ObjectIntegration => "OBJECT_INTEGRATION",
    ObjectResource => "OBJECT_RESOURCE",
    EvidenceFactReference => "EVIDENCE_FACT_REFERENCE",
});

tokens!(EvidenceRoleToken {
    ObjectTimeEvidence => "OBJECT_TIME_EVIDENCE",
    SeriesEvidence => "SERIES_EVIDENCE",
});

tokens!(ReplayResolutionState {
    ResolvableAndVerified => "RESOLVABLE_AND_VERIFIED",
    Unresolvable => "UNRESOLVABLE",
    DigestMismatch => "DIGEST_MISMATCH",
    UnsupportedReference => "UNSUPPORTED_REFERENCE",
});

tokens!(InteractionKindToken {
    Selected => "SELECTED",
    Corrected => "CORRECTED",
    Rejected => "REJECTED",
    Dismissed => "DISMISSED",
    ReplanRequested => "REPLAN_REQUESTED",
    NoChoiceConfirmed => "NO_CHOICE_CONFIRMED",
});

tokens!(RejectionScopeToken {
    Candidate => "CANDIDATE",
    Slate => "SLATE",
});

tokens!(CorrectionKindToken {
    Preference => "PREFERENCE",
    NewFact => "NEW_FACT",
    NewHardConstraint => "NEW_HARD_CONSTRAINT",
    ContextChange => "CONTEXT_CHANGE",
    UiOrSystem => "UI_OR_SYSTEM",
    OtherOrUnknown => "OTHER_OR_UNKNOWN",
});

tokens!(TerminalStateToken {
    Open => "OPEN",
    ClosedSelected => "CLOSED_SELECTED",
    ClosedRejectedSlate => "CLOSED_REJECTED_SLATE",
    ClosedDismissed => "CLOSED_DISMISSED",
    ClosedReplan => "CLOSED_REPLAN",
    ClosedNoChoice => "CLOSED_NO_CHOICE",
    ClosedNotAdmitted => "CLOSED_NOT_ADMITTED",
    ClosedNotExposed => "CLOSED_NOT_EXPOSED",
    ClosedSuperseded => "CLOSED_SUPERSEDED",
});

tokens!(ProductDecisionStateToken {
    Applied => "APPLIED",
    NotApplicable => "NOT_APPLICABLE",
    Exposed => "EXPOSED",
    NotExposed => "NOT_EXPOSED",
});

tokens!(ProductDecisionReasonToken {
    PlanningNotAdmitted => "PLANNING_NOT_ADMITTED",
});

tokens!(CaptureIncompleteReason {
    ExposureUnavailableOrNotInstrumented => "EXPOSURE_UNAVAILABLE_OR_NOT_INSTRUMENTED",
    ClientOrSessionEndedUnexpectedly => "CLIENT_OR_SESSION_ENDED_UNEXPECTEDLY",
    PersistenceFailure => "PERSISTENCE_FAILURE",
    InteractionStreamTruncated => "INTERACTION_STREAM_TRUNCATED",
    OutcomeUnavailable => "OUTCOME_UNAVAILABLE",
    UnsupportedInstrumentation => "UNSUPPORTED_INSTRUMENTATION",
});

tokens!(PlannerAdmissionIssueCodeToken {
    UnsupportedSchema => "UNSUPPORTED_SCHEMA",
    UnsupportedOperation => "UNSUPPORTED_OPERATION",
    UnsupportedObject => "UNSUPPORTED_OBJECT",
    RequiredDurationMissing => "REQUIRED_DURATION_MISSING",
    RequiredDurationUnknown => "REQUIRED_DURATION_UNKNOWN",
    RequiredDurationUncertain => "REQUIRED_DURATION_UNCERTAIN",
    RequiredDurationAmbiguous => "REQUIRED_DURATION_AMBIGUOUS",
    RequiredDurationUnresolved => "REQUIRED_DURATION_UNRESOLVED",
    RequiredTimeMissing => "REQUIRED_TIME_MISSING",
    RequiredTimeUnknown => "REQUIRED_TIME_UNKNOWN",
    RequiredTimeUncertain => "REQUIRED_TIME_UNCERTAIN",
    RequiredTimeAmbiguous => "REQUIRED_TIME_AMBIGUOUS",
    RequiredTimeUnresolved => "REQUIRED_TIME_UNRESOLVED",
    DuplicateObject => "DUPLICATE_OBJECT",
    DuplicateFact => "DUPLICATE_FACT",
    DuplicateTarget => "DUPLICATE_TARGET",
    UnknownObject => "UNKNOWN_OBJECT",
    ContradictoryFact => "CONTRADICTORY_FACT",
    InvalidFactSource => "INVALID_FACT_SOURCE",
    ScopeViolation => "SCOPE_VIOLATION",
    PlanningPermissionDenied => "PLANNING_PERMISSION_DENIED",
    MissingPlanningCapability => "MISSING_PLANNING_CAPABILITY",
    PolicyDenied => "POLICY_DENIED",
    InvalidTargetState => "INVALID_TARGET_STATE",
    NoTargets => "NO_TARGETS",
    UnsupportedSearchBudget => "UNSUPPORTED_SEARCH_BUDGET",
    UnsupportedTargetCount => "UNSUPPORTED_TARGET_COUNT",
    InputLimit => "INPUT_LIMIT",
    CompilationHorizonMismatch => "COMPILATION_HORIZON_MISMATCH",
    CompilationInputLimit => "COMPILATION_INPUT_LIMIT",
    CompilationDuplicateSeries => "COMPILATION_DUPLICATE_SERIES",
    CompilationIdentityCollision => "COMPILATION_IDENTITY_COLLISION",
    CompilationProspectiveSeries => "COMPILATION_PROSPECTIVE_SERIES",
    CompilationInvalidProvenance => "COMPILATION_INVALID_PROVENANCE",
    CompilationUnknownObjectTime => "COMPILATION_UNKNOWN_OBJECT_TIME",
    CompilationTemporalInvalidRange => "COMPILATION_TEMPORAL_INVALID_RANGE",
    CompilationTemporalInvalidDuration => "COMPILATION_TEMPORAL_INVALID_DURATION",
    CompilationTemporalOverflow => "COMPILATION_TEMPORAL_OVERFLOW",
    CompilationTemporalAmbiguousLocalTime => "COMPILATION_TEMPORAL_AMBIGUOUS_LOCAL_TIME",
    CompilationTemporalNonexistentLocalTime => "COMPILATION_TEMPORAL_NONEXISTENT_LOCAL_TIME",
    CompilationTemporalInvalidRecurrence => "COMPILATION_TEMPORAL_INVALID_RECURRENCE",
    CompilationTemporalInputLimit => "COMPILATION_TEMPORAL_INPUT_LIMIT",
    CompilationTemporalOccurrenceLimitExceeded => "COMPILATION_TEMPORAL_OCCURRENCE_LIMIT_EXCEEDED",
    CompilationTemporalDateLimitExceeded => "COMPILATION_TEMPORAL_DATE_LIMIT_EXCEEDED",
    IncompleteCoverage => "INCOMPLETE_COVERAGE",
    DependencyInputLimit => "DEPENDENCY_INPUT_LIMIT",
    DependencyMissingReference => "DEPENDENCY_MISSING_REFERENCE",
    DependencyCycle => "DEPENDENCY_CYCLE",
});

/// Independent manifest schema authority; incompatible versions reject at decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactManifestSchemaVersion {
    #[serde(rename = "0.1")]
    V0_1,
}
