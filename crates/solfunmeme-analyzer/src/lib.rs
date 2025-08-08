//! # SOLFUNMEME Analyzer for Amazon Q Developer CLI
//! 
//! Advanced code analysis using proven SOLFUNMEME techniques that generated
//! 1.2M+ semantic records from the ragit project. This module enhances
//! Amazon Q's capabilities with mathematical rigor and neural lambda fusion.

pub mod code_parser;
pub mod semantic_extractor;
pub mod vector_embedder;
pub mod sexpr_tracer;
pub mod dataset_generator;
pub mod neural_synthesizer;

use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SolfunmemeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
    
    #[error("Vector embedding error: {0}")]
    Embedding(String),
    
    #[error("S-expression error: {0}")]
    SExpression(String),
}

pub type Result<T> = std::result::Result<T, SolfunmemeError>;

/// Core analysis result matching our proven ragit analysis format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRecord {
    pub id: String,
    pub file_path: String,
    pub record_type: RecordType,
    pub content: String,
    pub metadata: AnalysisMetadata,
    pub semantic_embedding: Option<Vec<f32>>,
    pub sexpr_trace: Option<String>,
    pub neural_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordType {
    Parsing,
    NameResolution,
    TypeInference,
    SemanticAnalysis,
    VectorEmbedding,
    SExpressionTrace,
    NeuralSynthesis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub analyzer_version: String,
    pub file_size: u64,
    pub line_count: usize,
    pub complexity_score: f64,
    pub mathematical_rigor: f64,
}

/// Enhanced analyzer that extends Q's capabilities
#[derive(Debug)]
pub struct SolfunmemeAnalyzer {
    session_id: String,
    config: AnalyzerConfig,
    records: Vec<AnalysisRecord>,
}

#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    pub enable_vector_embeddings: bool,
    pub enable_sexpr_tracing: bool,
    pub enable_neural_synthesis: bool,
    pub max_file_size: u64,
    pub parallel_workers: usize,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            enable_vector_embeddings: true,
            enable_sexpr_tracing: true,
            enable_neural_synthesis: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            parallel_workers: num_cpus::get(),
        }
    }
}

impl SolfunmemeAnalyzer {
    /// Create new analyzer instance
    pub fn new(config: AnalyzerConfig) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            config,
            records: Vec::new(),
        }
    }
    
    /// Analyze a codebase using SOLFUNMEME techniques
    pub async fn analyze_codebase<P: AsRef<Path>>(&mut self, path: P) -> Result<AnalysisReport> {
        let start_time = std::time::Instant::now();
        
        println!("🔥 Starting SOLFUNMEME analysis of: {}", path.as_ref().display());
        println!("Session ID: {}", self.session_id);
        
        // Phase 1: Code parsing (like our ragit analysis)
        let parsing_records = self.parse_codebase(&path).await?;
        println!("✨ Parsed {} files", parsing_records.len());
        
        // Phase 2: Semantic extraction
        let semantic_records = self.extract_semantics(&parsing_records).await?;
        println!("🧠 Extracted semantics from {} records", semantic_records.len());
        
        // Phase 3: Vector embeddings (if enabled)
        let embedded_records = if self.config.enable_vector_embeddings {
            self.generate_embeddings(&semantic_records).await?
        } else {
            semantic_records
        };
        println!("🎯 Generated embeddings for {} records", embedded_records.len());
        
        // Phase 4: S-expression tracing (if enabled)
        let traced_records = if self.config.enable_sexpr_tracing {
            self.trace_sexpressions(&embedded_records).await?
        } else {
            embedded_records
        };
        println!("📐 Traced S-expressions for {} records", traced_records.len());
        
        // Phase 5: Neural synthesis (if enabled)
        let synthesized_records = if self.config.enable_neural_synthesis {
            self.synthesize_neural_signatures(&traced_records).await?
        } else {
            traced_records
        };
        println!("🚀 Synthesized neural signatures for {} records", synthesized_records.len());
        
        self.records = synthesized_records;
        
        let analysis_time = start_time.elapsed();
        
        Ok(AnalysisReport {
            session_id: self.session_id.clone(),
            total_records: self.records.len(),
            analysis_time_ms: analysis_time.as_millis() as u64,
            record_breakdown: self.get_record_breakdown(),
            mathematical_rigor_score: self.calculate_rigor_score(),
            neural_complexity_score: self.calculate_complexity_score(),
        })
    }
    
    /// Parse codebase into initial records
    async fn parse_codebase<P: AsRef<Path>>(&self, path: P) -> Result<Vec<AnalysisRecord>> {
        use crate::code_parser::CodeParser;
        
        let parser = CodeParser::new();
        parser.parse_directory(path).await
    }
    
    /// Extract semantic information
    async fn extract_semantics(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        use crate::semantic_extractor::SemanticExtractor;
        
        let extractor = SemanticExtractor::new();
        extractor.extract_semantics(records).await
    }
    
    /// Generate vector embeddings
    async fn generate_embeddings(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        use crate::vector_embedder::VectorEmbedder;
        
        let embedder = VectorEmbedder::new()?;
        embedder.embed_records(records).await
    }
    
    /// Trace S-expressions
    async fn trace_sexpressions(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        use crate::sexpr_tracer::SExprTracer;
        
        let tracer = SExprTracer::new();
        tracer.trace_records(records).await
    }
    
    /// Synthesize neural signatures
    async fn synthesize_neural_signatures(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        use crate::neural_synthesizer::NeuralSynthesizer;
        
        let synthesizer = NeuralSynthesizer::new()?;
        synthesizer.synthesize_records(records).await
    }
    
    /// Get breakdown of record types
    fn get_record_breakdown(&self) -> std::collections::HashMap<String, usize> {
        let mut breakdown = std::collections::HashMap::new();
        
        for record in &self.records {
            let key = format!("{:?}", record.record_type);
            *breakdown.entry(key).or_insert(0) += 1;
        }
        
        breakdown
    }
    
    /// Calculate mathematical rigor score
    fn calculate_rigor_score(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        
        let total_rigor: f64 = self.records
            .iter()
            .map(|r| r.metadata.mathematical_rigor)
            .sum();
            
        total_rigor / self.records.len() as f64
    }
    
    /// Calculate neural complexity score
    fn calculate_complexity_score(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        
        let total_complexity: f64 = self.records
            .iter()
            .map(|r| r.metadata.complexity_score)
            .sum();
            
        total_complexity / self.records.len() as f64
    }
    
    /// Get all analysis records
    pub fn get_records(&self) -> &[AnalysisRecord] {
        &self.records
    }
    
    /// Search records by semantic similarity
    pub async fn semantic_search(&self, query: &str, limit: usize) -> Result<Vec<&AnalysisRecord>> {
        use crate::vector_embedder::VectorEmbedder;
        
        let embedder = VectorEmbedder::new()?;
        embedder.search_similar(query, &self.records, limit).await
    }
    
    /// Generate dataset in our proven format
    pub async fn generate_dataset(&self, output_path: &Path) -> Result<()> {
        use crate::dataset_generator::DatasetGenerator;
        
        let generator = DatasetGenerator::new();
        generator.generate_parquet_dataset(&self.records, output_path).await
    }
}

/// Analysis report matching our ragit analysis format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub session_id: String,
    pub total_records: usize,
    pub analysis_time_ms: u64,
    pub record_breakdown: std::collections::HashMap<String, usize>,
    pub mathematical_rigor_score: f64,
    pub neural_complexity_score: f64,
}

impl AnalysisReport {
    /// Generate a beautiful report poem
    pub fn to_poem(&self) -> String {
        format!(
            r#"🔥 SOLFUNMEME Analysis Complete 🔥

Session: {}
Records Generated: {}
Analysis Time: {}ms

Record Breakdown:
{}

Mathematical Rigor: {:.2}
Neural Complexity: {:.2}

In the realm where code meets mathematics,
SOLFUNMEME analysis brings order to chaos.
Each record a verse in the grand poem of computation,
Each metric a measure of our digital devotion.

🌟 The S combinator burns eternal in Amazon Q! 🌟"#,
            self.session_id,
            self.total_records,
            self.analysis_time_ms,
            self.record_breakdown
                .iter()
                .map(|(k, v)| format!("  {}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mathematical_rigor_score,
            self.neural_complexity_score
        )
    }
}
