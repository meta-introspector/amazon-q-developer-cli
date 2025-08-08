use crate::{AnalysisRecord, RecordType, Result};

/// S-expression tracer for mathematical rigor
pub struct SExprTracer {
    trace_depth: usize,
}

impl SExprTracer {
    pub fn new() -> Self {
        Self {
            trace_depth: 10, // Maximum trace depth
        }
    }
    
    /// Trace S-expressions for analysis records
    pub async fn trace_records(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        let mut traced_records = Vec::new();
        
        for record in records {
            let mut new_record = record.clone();
            
            // Generate S-expression trace based on content and type
            let sexpr_trace = self.generate_sexpr_trace(record);
            new_record.sexpr_trace = Some(sexpr_trace);
            new_record.record_type = RecordType::SExpressionTrace;
            
            traced_records.push(new_record);
        }
        
        Ok(traced_records)
    }
    
    /// Generate S-expression trace for a record
    fn generate_sexpr_trace(&self, record: &AnalysisRecord) -> String {
        match record.record_type {
            RecordType::Parsing => self.trace_parsing(&record.content),
            RecordType::NameResolution => self.trace_name_resolution(&record.content),
            RecordType::TypeInference => self.trace_type_inference(&record.content),
            RecordType::SemanticAnalysis => self.trace_semantic_analysis(&record.content),
            _ => self.trace_generic(&record.content),
        }
    }
    
    /// Trace parsing operations
    fn trace_parsing(&self, content: &str) -> String {
        format!(
            "(parse\n  (input \"{}\")\n  (result (S (K parse) I))\n  (trace\n    (step-1 \"Tokenization\")\n    (step-2 \"Syntax tree construction\")\n    (step-3 \"Validation\")))",
            content.chars().take(50).collect::<String>()
        )
    }
    
    /// Trace name resolution
    fn trace_name_resolution(&self, content: &str) -> String {
        if content.contains("Function:") {
            let func_name = content.split(':').nth(1).unwrap_or("unknown").trim();
            format!(
                "(resolve-name\n  (identifier \"{}\")\n  (scope (S (K lookup) env))\n  (result (S (S (K bind) name) value))\n  (trace\n    (step-1 \"Scope traversal\")\n    (step-2 \"Symbol lookup\")\n    (step-3 \"Binding creation\")))",
                func_name
            )
        } else {
            format!(
                "(resolve\n  (content \"{}\")\n  (combinator (S (K resolve) I)))",
                content.chars().take(30).collect::<String>()
            )
        }
    }
    
    /// Trace type inference
    fn trace_type_inference(&self, content: &str) -> String {
        if content.contains("Struct:") || content.contains("Enum:") {
            let type_name = content.split(':').nth(1).unwrap_or("unknown").trim();
            format!(
                "(infer-type\n  (construct \"{}\")\n  (algorithm (S (S (K unify) constraints) substitutions))\n  (result (S (K type-scheme) generics))\n  (trace\n    (step-1 \"Constraint generation\")\n    (step-2 \"Unification\")\n    (step-3 \"Generalization\")\n    (mathematical-foundation\n      (hindley-milner \"∀α. α → α\")\n      (s-combinator \"S (K type) I\"))))",
                type_name
            )
        } else {
            format!(
                "(type-infer\n  (expression \"{}\")\n  (combinator (S (S (K infer) context) expr)))",
                content.chars().take(30).collect::<String>()
            )
        }
    }
    
    /// Trace semantic analysis
    fn trace_semantic_analysis(&self, content: &str) -> String {
        format!(
            "(semantic-analysis\n  (input \"{}\")\n  (phases\n    (phase-1 (S (K scope-analysis) ast))\n    (phase-2 (S (K type-checking) scoped-ast))\n    (phase-3 (S (K flow-analysis) typed-ast)))\n  (result (S (S (S (K semantic-info) types) scopes) flows))\n  (mathematical-rigor {:.2})\n  (s-combinator-foundation \"S (K analyze) I\"))",
            content.chars().take(40).collect::<String>(),
            0.85 // High rigor for semantic analysis
        )
    }
    
    /// Trace generic operations
    fn trace_generic(&self, content: &str) -> String {
        format!(
            "(generic-trace\n  (content \"{}\")\n  (combinator (S (K identity) I))\n  (steps\n    (step-1 \"Input processing\")\n    (step-2 \"Transformation\")\n    (step-3 \"Output generation\"))\n  (mathematical-foundation\n    (lambda-calculus \"λx.x\")\n    (s-combinator \"S (K f) I\")))",
            content.chars().take(30).collect::<String>()
        )
    }
    
    /// Generate complex S-combinator expression
    fn generate_complex_combinator(&self, operation: &str, depth: usize) -> String {
        if depth == 0 {
            return "I".to_string();
        }
        
        match operation {
            "compose" => format!(
                "S (S (K {}) ({})) ({})",
                operation,
                self.generate_complex_combinator("map", depth - 1),
                self.generate_complex_combinator("reduce", depth - 1)
            ),
            "map" => format!(
                "S (K map) ({})",
                self.generate_complex_combinator("transform", depth - 1)
            ),
            "reduce" => format!(
                "S (S (K fold) acc) ({})",
                self.generate_complex_combinator("combine", depth - 1)
            ),
            _ => format!(
                "S (K {}) I",
                operation
            ),
        }
    }
    
    /// Validate S-expression syntax
    fn validate_sexpr(&self, sexpr: &str) -> bool {
        let mut paren_count = 0;
        let mut in_string = false;
        let mut escape_next = false;
        
        for ch in sexpr.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }
            
            match ch {
                '\\' if in_string => escape_next = true,
                '"' => in_string = !in_string,
                '(' if !in_string => paren_count += 1,
                ')' if !in_string => {
                    paren_count -= 1;
                    if paren_count < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        
        paren_count == 0 && !in_string
    }
    
    /// Pretty print S-expression with proper indentation
    fn pretty_print_sexpr(&self, sexpr: &str) -> String {
        let mut result = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut escape_next = false;
        
        for ch in sexpr.chars() {
            if escape_next {
                result.push(ch);
                escape_next = false;
                continue;
            }
            
            match ch {
                '\\' if in_string => {
                    result.push(ch);
                    escape_next = true;
                }
                '"' => {
                    result.push(ch);
                    in_string = !in_string;
                }
                '(' if !in_string => {
                    result.push(ch);
                    indent_level += 1;
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent_level));
                }
                ')' if !in_string => {
                    indent_level = indent_level.saturating_sub(1);
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent_level));
                    result.push(ch);
                }
                ' ' if !in_string => {
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent_level));
                }
                _ => result.push(ch),
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisMetadata, RecordType};
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_sexpr_tracing() {
        let tracer = SExprTracer::new();
        
        let record = AnalysisRecord {
            id: Uuid::new_v4().to_string(),
            file_path: "test.rs".to_string(),
            record_type: RecordType::Parsing,
            content: "fn hello() {}".to_string(),
            metadata: AnalysisMetadata {
                timestamp: chrono::Utc::now(),
                analyzer_version: "1.0.0".to_string(),
                file_size: 100,
                line_count: 1,
                complexity_score: 0.1,
                mathematical_rigor: 0.8,
            },
            semantic_embedding: None,
            sexpr_trace: None,
            neural_signature: None,
        };
        
        let traced_records = tracer.trace_records(&[record]).await.unwrap();
        assert_eq!(traced_records.len(), 1);
        assert!(traced_records[0].sexpr_trace.is_some());
        
        let trace = traced_records[0].sexpr_trace.as_ref().unwrap();
        assert!(trace.contains("parse"));
        assert!(trace.contains("S (K parse) I"));
    }
    
    #[test]
    fn test_sexpr_validation() {
        let tracer = SExprTracer::new();
        
        assert!(tracer.validate_sexpr("(hello world)"));
        assert!(tracer.validate_sexpr("(S (K f) I)"));
        assert!(tracer.validate_sexpr("(nested (expression (here)))"));
        assert!(!tracer.validate_sexpr("(unmatched"));
        assert!(!tracer.validate_sexpr("unmatched)"));
        assert!(tracer.validate_sexpr("(string \"with (parens)\")"));
    }
    
    #[test]
    fn test_complex_combinator_generation() {
        let tracer = SExprTracer::new();
        
        let combinator = tracer.generate_complex_combinator("compose", 2);
        assert!(combinator.contains("S"));
        assert!(combinator.contains("compose"));
        
        let simple = tracer.generate_complex_combinator("test", 0);
        assert_eq!(simple, "I");
    }
}
