use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod git;
pub mod interactive_quiz;

// Re-export main components
pub use git::GitLogCollector;
pub use interactive_quiz::{InteractiveQuizSession, AIReactionGenerator, QuizSession, QuizResponse};

/// Core data structures for the unified knowledge system

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub commit_hash: String,
    pub author: String,
    pub message: String,
    pub submodule_path: String,
    pub files_changed: Vec<String>,
    pub diff_stats: DiffStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStats {
    pub insertions: usize,
    pub deletions: usize,
    pub files_changed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub commit_hash: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeFragment {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source_log: LogEntry,
    pub extracted_concepts: Vec<Concept>,
    pub emoji_analysis: EmojiAnalysis,
    pub bert_embedding: Vec<f32>,
    pub clifford_multivector: Multivector,
    pub ttl_mappings: Vec<TTLMapping>,
    pub reactions: Vec<Reaction>,
    pub hot_takes: Vec<HotTake>,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub confidence: f64,
    pub category: ConceptCategory,
    pub related_emojis: Vec<String>,
    pub ontology_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptCategory {
    Technical,
    Architectural,
    Process,
    Tool,
    Language,
    Framework,
    Pattern,
    Bug,
    Feature,
    Documentation,
    Performance,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiAnalysis {
    pub emojis_found: Vec<String>,
    pub universe_emojis: Vec<String>, // From the 16 core universe emojis
    pub emoji_count: usize,
    pub semantic_density: f64,
    pub multivector_coefficients: [f64; 8], // 8D multivector from conversation summary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multivector {
    pub coefficients: [f64; 8], // 8D Clifford algebra representation
    pub magnitude: f64,
    pub geometric_interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTLMapping {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
    pub ontology_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub page_number: usize,
    pub reaction_type: ReactionType,
    pub content: String,
    pub confidence: f64,
    pub emoji_context: Vec<String>,
    pub target_fragment_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReactionType {
    Insight,
    Question,
    Correction,
    Enhancement,
    Connection,
    HotTake,
    Bookmark,
    Flag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotTake {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub related_concepts: Vec<String>,
    pub confidence: f64,
    pub impact_score: f64,
    pub emoji_signature: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub page_number: usize,
    pub total_pages: usize,
    pub items: Vec<T>,
    pub timestamp_range: (DateTime<Utc>, DateTime<Utc>),
    pub navigation: PageNavigation,
    pub metadata: PageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNavigation {
    pub previous_timestamp: Option<DateTime<Utc>>,
    pub next_timestamp: Option<DateTime<Utc>>,
    pub can_continue: bool,
    pub can_go_back: bool,
    pub bookmark_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub total_concepts: usize,
    pub emoji_density: f64,
    pub quality_score: f64,
    pub reaction_count: usize,
    pub hot_take_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedLogEntry {
    pub log_entry: LogEntry,
    pub knowledge_fragment: KnowledgeFragment,
    pub related_entries: Vec<Uuid>,
    pub similarity_scores: BTreeMap<Uuid, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedQuery {
    pub text_query: Option<String>,
    pub emoji_pattern: Option<String>,
    pub semantic_concepts: Vec<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub submodule_filter: Option<String>,
    pub reaction_type_filter: Option<ReactionType>,
    pub quality_threshold: Option<f64>,
    pub bert_similarity_threshold: Option<f64>,
    pub clifford_distance_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub results: Vec<EnrichedLogEntry>,
    pub total_matches: usize,
    pub query_time_ms: u128,
    pub aggregations: QueryAggregations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAggregations {
    pub concept_frequency: BTreeMap<String, usize>,
    pub emoji_frequency: BTreeMap<String, usize>,
    pub submodule_distribution: BTreeMap<String, usize>,
    pub reaction_type_distribution: BTreeMap<ReactionType, usize>,
    pub quality_distribution: BTreeMap<String, usize>, // "high", "medium", "low"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryEntry {
    pub term: String,
    pub definition: String,
    pub category: ConceptCategory,
    pub related_terms: Vec<String>,
    pub emoji_representation: Vec<String>,
    pub usage_frequency: usize,
    pub last_updated: DateTime<Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryUpdate {
    pub new_entries: Vec<GlossaryEntry>,
    pub updated_entries: Vec<GlossaryEntry>,
    pub deprecated_entries: Vec<String>,
    pub confidence_adjustments: BTreeMap<String, f64>,
}

// Universe emoji system from conversation summary
pub const UNIVERSE_EMOJIS: &[&str] = &[
    "🧮", "🔢", "✨", "💫", "🔥", "🌊", "📊", "🎯", 
    "💎", "🕳️", "📱", "🌙", "⭐", "🌌", "🚀", "🪐"
];

// Error types
#[derive(Debug, thiserror::Error)]
pub enum UnifiedKnowledgeError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parquet error: {0}")]
    ParquetError(String),
    
    #[error("Knowledge extraction error: {0}")]
    KnowledgeError(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Reaction processing error: {0}")]
    ReactionError(String),
}

pub type Result<T> = std::result::Result<T, UnifiedKnowledgeError>;
