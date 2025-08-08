use std::path::Path;
use walkdir::WalkDir;
use syn::{parse_file, Item};
use uuid::Uuid;
use crate::{AnalysisRecord, RecordType, AnalysisMetadata, Result, SolfunmemeError};

/// Code parser using proven techniques from our ragit analysis
pub struct CodeParser {
    supported_extensions: Vec<String>,
}

impl CodeParser {
    pub fn new() -> Self {
        Self {
            supported_extensions: vec![
                "rs".to_string(),
                "py".to_string(), 
                "js".to_string(),
                "ts".to_string(),
                "java".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "go".to_string(),
                "rb".to_string(),
                "php".to_string(),
            ],
        }
    }
    
    /// Parse entire directory recursively (like our ragit analysis)
    pub async fn parse_directory<P: AsRef<Path>>(&self, path: P) -> Result<Vec<AnalysisRecord>> {
        let mut records = Vec::new();
        
        for entry in WalkDir::new(path.as_ref())
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if self.supported_extensions.contains(&ext_str.to_lowercase()) {
                            match self.parse_file(entry.path()).await {
                                Ok(mut file_records) => records.append(&mut file_records),
                                Err(e) => {
                                    eprintln!("Warning: Failed to parse {}: {}", entry.path().display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(records)
    }
    
    /// Parse individual file
    pub async fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<AnalysisRecord>> {
        let content = tokio::fs::read_to_string(&path).await?;
        let file_path = path.as_ref().to_string_lossy().to_string();
        
        let metadata = std::fs::metadata(&path)?;
        let file_size = metadata.len();
        let line_count = content.lines().count();
        
        let mut records = Vec::new();
        
        // Determine file type and parse accordingly
        if file_path.ends_with(".rs") {
            records.extend(self.parse_rust_file(&file_path, &content, file_size, line_count)?);
        } else {
            // Generic parsing for other languages
            records.push(self.create_generic_record(&file_path, &content, file_size, line_count));
        }
        
        Ok(records)
    }
    
    /// Parse Rust file using syn (our specialty)
    fn parse_rust_file(
        &self, 
        file_path: &str, 
        content: &str, 
        file_size: u64, 
        line_count: usize
    ) -> Result<Vec<AnalysisRecord>> {
        let mut records = Vec::new();
        
        // Parse the Rust syntax tree
        match parse_file(content) {
            Ok(syntax_tree) => {
                // Create parsing record
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::Parsing,
                    content: format!("Parsed {} items", syntax_tree.items.len()),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: self.calculate_complexity(&syntax_tree),
                        mathematical_rigor: 0.8, // Rust gets high rigor score
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
                
                // Analyze each item in the syntax tree
                for item in &syntax_tree.items {
                    records.extend(self.analyze_rust_item(file_path, item, file_size, line_count)?);
                }
            }
            Err(e) => {
                return Err(SolfunmemeError::Parse(format!("Failed to parse Rust file {}: {}", file_path, e)));
            }
        }
        
        Ok(records)
    }
    
    /// Analyze individual Rust syntax item
    fn analyze_rust_item(
        &self,
        file_path: &str,
        item: &Item,
        file_size: u64,
        line_count: usize,
    ) -> Result<Vec<AnalysisRecord>> {
        let mut records = Vec::new();
        
        match item {
            Item::Fn(func) => {
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::NameResolution,
                    content: format!("Function: {}", func.sig.ident),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: self.calculate_function_complexity(func),
                        mathematical_rigor: 0.9, // Functions get high rigor
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
            }
            Item::Struct(struct_item) => {
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::TypeInference,
                    content: format!("Struct: {}", struct_item.ident),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: self.calculate_struct_complexity(struct_item),
                        mathematical_rigor: 0.85,
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
            }
            Item::Enum(enum_item) => {
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::TypeInference,
                    content: format!("Enum: {}", enum_item.ident),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: self.calculate_enum_complexity(enum_item),
                        mathematical_rigor: 0.9, // Enums are mathematically rigorous
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
            }
            Item::Impl(impl_item) => {
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::SemanticAnalysis,
                    content: format!("Impl block with {} items", impl_item.items.len()),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: impl_item.items.len() as f64 * 0.1,
                        mathematical_rigor: 0.8,
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
            }
            _ => {
                // Handle other item types generically
                records.push(AnalysisRecord {
                    id: Uuid::new_v4().to_string(),
                    file_path: file_path.to_string(),
                    record_type: RecordType::Parsing,
                    content: format!("Other item: {:?}", std::mem::discriminant(item)),
                    metadata: AnalysisMetadata {
                        timestamp: chrono::Utc::now(),
                        analyzer_version: "1.0.0".to_string(),
                        file_size,
                        line_count,
                        complexity_score: 0.1,
                        mathematical_rigor: 0.5,
                    },
                    semantic_embedding: None,
                    sexpr_trace: None,
                    neural_signature: None,
                });
            }
        }
        
        Ok(records)
    }
    
    /// Create generic record for non-Rust files
    fn create_generic_record(
        &self,
        file_path: &str,
        content: &str,
        file_size: u64,
        line_count: usize,
    ) -> AnalysisRecord {
        AnalysisRecord {
            id: Uuid::new_v4().to_string(),
            file_path: file_path.to_string(),
            record_type: RecordType::Parsing,
            content: format!("Generic file with {} lines", line_count),
            metadata: AnalysisMetadata {
                timestamp: chrono::Utc::now(),
                analyzer_version: "1.0.0".to_string(),
                file_size,
                line_count,
                complexity_score: (line_count as f64).log10(),
                mathematical_rigor: 0.3, // Lower rigor for non-Rust
            },
            semantic_embedding: None,
            sexpr_trace: None,
            neural_signature: None,
        }
    }
    
    /// Calculate complexity score for syntax tree
    fn calculate_complexity(&self, syntax_tree: &syn::File) -> f64 {
        syntax_tree.items.len() as f64 * 0.1
    }
    
    /// Calculate function complexity
    fn calculate_function_complexity(&self, func: &syn::ItemFn) -> f64 {
        // Simple complexity based on parameter count and body presence
        let param_count = func.sig.inputs.len() as f64;
        let has_body = func.block.stmts.len() as f64;
        (param_count * 0.1) + (has_body * 0.05)
    }
    
    /// Calculate struct complexity
    fn calculate_struct_complexity(&self, struct_item: &syn::ItemStruct) -> f64 {
        match &struct_item.fields {
            syn::Fields::Named(fields) => fields.named.len() as f64 * 0.1,
            syn::Fields::Unnamed(fields) => fields.unnamed.len() as f64 * 0.1,
            syn::Fields::Unit => 0.1,
        }
    }
    
    /// Calculate enum complexity
    fn calculate_enum_complexity(&self, enum_item: &syn::ItemEnum) -> f64 {
        enum_item.variants.len() as f64 * 0.15
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    
    #[tokio::test]
    async fn test_rust_file_parsing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        
        let rust_code = r#"
            fn hello_world() {
                println!("Hello, world!");
            }
            
            struct Point {
                x: f64,
                y: f64,
            }
            
            enum Color {
                Red,
                Green,
                Blue,
            }
        "#;
        
        fs::write(&file_path, rust_code).await.unwrap();
        
        let parser = CodeParser::new();
        let records = parser.parse_file(&file_path).await.unwrap();
        
        assert!(!records.is_empty());
        
        // Should have parsing record + function + struct + enum
        assert!(records.len() >= 4);
        
        // Check that we have different record types
        let record_types: std::collections::HashSet<_> = records
            .iter()
            .map(|r| format!("{:?}", r.record_type))
            .collect();
        
        assert!(record_types.len() > 1);
    }
}
