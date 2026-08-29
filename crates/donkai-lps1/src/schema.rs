use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthoringMode {
    #[serde(rename = "human-authored")]
    HumanAuthored,
    #[serde(rename = "human-with-accessibility-aid")]
    HumanWithAccessibilityAid,
    #[serde(rename = "human-dictated-transcription")]
    HumanDictatedTranscription,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateCertainty {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "approximate")]
    Approximate,
    #[serde(rename = "era-estimated")]
    EraEstimated,
    #[serde(rename = "uncertain")]
    Uncertain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDateRange {
    pub start: String,
    pub end: String,
    pub certainty: DateCertainty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationPrecision {
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "neighborhood")]
    Neighborhood,
    #[serde(rename = "city")]
    City,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "global")]
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocationDescriptor {
    pub label: String,
    pub precision: LocationPrecision,
}

/// 1. RemembranceStatement
/// Original human-authored narrative, preserving original phrasing, dialect, and nuance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemembranceStatement {
    #[serde(rename = "type")]
    pub record_type: String,
    pub language: String,
    #[serde(rename = "authoringMode")]
    pub authoring_mode: AuthoringMode,
    pub narrative: String,
    #[serde(rename = "eventDate")]
    pub event_date: EventDateRange,
    pub location: LocationDescriptor,
    #[serde(rename = "culturalContext")]
    pub cultural_context: Vec<String>,
    #[serde(rename = "authorAttestation")]
    pub author_attestation: String,
}

impl RemembranceStatement {
    pub fn new_human_authored(
        language: impl Into<String>,
        narrative: impl Into<String>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
        location_label: impl Into<String>,
        location_precision: LocationPrecision,
        cultural_tags: Vec<String>,
        attestation: impl Into<String>,
    ) -> Self {
        Self {
            record_type: "donkai.remembrance.v1".to_string(),
            language: language.into(),
            authoring_mode: AuthoringMode::HumanAuthored,
            narrative: narrative.into(),
            event_date: EventDateRange {
                start: start_date.into(),
                end: end_date.into(),
                certainty: DateCertainty::Approximate,
            },
            location: LocationDescriptor {
                label: location_label.into(),
                precision: location_precision,
            },
            cultural_context: cultural_tags,
            author_attestation: attestation.into(),
        }
    }
}

/// 2. ContextManifest
/// Metadata describing the context, category, and discovery metadata for blind corroboration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryContext {
    pub category: String,
    pub place: String,
    #[serde(rename = "dateRange")]
    pub date_range: String,
    #[serde(rename = "culturalKeywords")]
    pub cultural_keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextManifest {
    #[serde(rename = "type")]
    pub record_type: String,
    pub category: String,
    #[serde(rename = "eventDate")]
    pub event_date: EventDateRange,
    pub location: LocationDescriptor,
    #[serde(rename = "culturalContext")]
    pub cultural_context: Vec<String>,
    #[serde(rename = "discoveryMetadata")]
    pub discovery_metadata: DiscoveryContext,
    #[serde(rename = "isProtectedSplit")]
    pub is_protected_split: bool,
}

/// 3. ConsentManifest
/// Privacy, visibility, pseudonymity, translation, and retention rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisibilityMode {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "reviewer-only")]
    ReviewerOnly,
    #[serde(rename = "trusted-circle")]
    TrustedCircle,
    #[serde(rename = "delayed-public")]
    DelayedPublic,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "aggregate-only")]
    AggregateOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityMode {
    #[serde(rename = "attributable")]
    Attributable,
    #[serde(rename = "pseudonymous")]
    Pseudonymous,
    #[serde(rename = "anonymous-with-credential")]
    AnonymousWithCredential,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentManifest {
    #[serde(rename = "type")]
    pub record_type: String,
    pub visibility: VisibilityMode,
    #[serde(rename = "identityMode")]
    pub identity_mode: IdentityMode,
    #[serde(rename = "allowAggregateResearch")]
    pub allow_aggregate_research: bool,
    #[serde(rename = "allowPublicExcerpt")]
    pub allow_public_excerpt: bool,
    #[serde(rename = "allowTranslation")]
    pub allow_translation: bool,
    #[serde(rename = "allowIndependentCorroboration")]
    pub allow_independent_corroboration: bool,
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: String,
    #[serde(rename = "sensitiveContentFlags")]
    pub sensitive_content_flags: Vec<String>,
}

/// 4. EvidenceBundle & Items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceClass {
    #[serde(rename = "author-provided")]
    AuthorProvided,
    #[serde(rename = "third-party-witness")]
    ThirdPartyWitness,
    #[serde(rename = "institutional-archive")]
    InstitutionalArchive,
    #[serde(rename = "public-record")]
    PublicRecord,
    #[serde(rename = "media-broadcast")]
    MediaBroadcast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceTier {
    #[serde(rename = "contemporaneous-artifact")]
    ContemporaneousArtifact,
    #[serde(rename = "subsequent-documentation")]
    SubsequentDocumentation,
    #[serde(rename = "recollection-sketch")]
    RecollectionSketch,
    #[serde(rename = "derivative-analysis")]
    DerivativeAnalysis,
    #[serde(rename = "unverified-lead")]
    UnverifiedLead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiDisclosure {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "restoration-filter-applied")]
    RestorationFilterApplied,
    #[serde(rename = "ocr-transcription-assisted")]
    OcrTranscriptionAssisted,
    #[serde(rename = "ai-generated-illustration-reference")]
    AiGeneratedIllustrationReference,
    #[serde(rename = "translation-assisted")]
    TranslationAssisted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceItem {
    #[serde(rename = "contentHash")]
    pub content_hash: String,
    #[serde(rename = "storageRef")]
    pub storage_ref: String,
    #[serde(rename = "sourceClass")]
    pub source_class: SourceClass,
    #[serde(rename = "evidenceTier")]
    pub evidence_tier: EvidenceTier,
    #[serde(rename = "claimedCreatedAt")]
    pub claimed_created_at: String,
    pub access: String,
    #[serde(rename = "aiDisclosure")]
    pub ai_disclosure: AiDisclosure,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(rename = "memoryId")]
    pub memory_id: String,
    pub items: Vec<EvidenceItem>,
}

/// 5. CorroborationCommitment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorroborationCommitment {
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(rename = "memoryId")]
    pub memory_id: String,
    #[serde(rename = "discoveryContext")]
    pub discovery_context: DiscoveryContext,
    #[serde(rename = "sealedRecallRoot")]
    pub sealed_recall_root: String,
    #[serde(rename = "commitmentTime")]
    pub commitment_time: String,
    pub visibility: VisibilityMode,
    #[serde(rename = "eligibilityCredentialRef")]
    pub eligibility_credential_ref: Option<String>,
}

/// 6. ReviewAssessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportClassification {
    #[serde(rename = "historically-supported")]
    HistoricallySupported,
    #[serde(rename = "partially-supported")]
    PartiallySupported,
    #[serde(rename = "conflicting-evidence")]
    ConflictingEvidence,
    #[serde(rename = "unresolved")]
    Unresolved,
    #[serde(rename = "insufficient-evidence")]
    InsufficientEvidence,
    #[serde(rename = "retracted-by-author")]
    RetractedByAuthor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAssessment {
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(rename = "memoryId")]
    pub memory_id: String,
    #[serde(rename = "methodologyVersion")]
    pub methodology_version: String,
    #[serde(rename = "reviewerRole")]
    pub reviewer_role: String,
    #[serde(rename = "conflictDisclosure")]
    pub conflict_disclosure: String,
    #[serde(rename = "evidenceRootsConsidered")]
    pub evidence_roots_considered: Vec<String>,
    #[serde(rename = "boundedClassification")]
    pub bounded_classification: SupportClassification,
    #[serde(rename = "writtenRationaleRoot")]
    pub written_rationale_root: String,
    #[serde(rename = "appealWindowEnd")]
    pub appeal_window_end: String,
    #[serde(rename = "appealStatus")]
    pub appeal_status: String,
}

/// 7. VersionGraph & Amendments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmendmentRelation {
    #[serde(rename = "clarification")]
    Clarification,
    #[serde(rename = "correction")]
    Correction,
    #[serde(rename = "redaction")]
    Redaction,
    #[serde(rename = "retraction")]
    Retraction,
    #[serde(rename = "evidence-supplement")]
    EvidenceSupplement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionGraph {
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(rename = "parentRecordRoot")]
    pub parent_record_root: Option<String>,
    #[serde(rename = "amendmentRelation")]
    pub amendment_relation: Option<AmendmentRelation>,
    #[serde(rename = "effectiveTimestamp")]
    pub effective_timestamp: String,
    #[serde(rename = "signedEventHistory")]
    pub signed_event_history: Vec<String>,
}

/// Full Unified Memory Record (LPS-1 Master Document)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub statement: RemembranceStatement,
    pub context: ContextManifest,
    pub consent: ConsentManifest,
    pub evidence: Option<EvidenceBundle>,
    pub corroboration: Option<CorroborationCommitment>,
    pub review: Option<ReviewAssessment>,
    pub version: VersionGraph,
}
