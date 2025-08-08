use crate::{AnalysisRecord, RecordType, Result, SolfunmemeError};
use candle_core::{Device, Tensor, DType};
use std::collections::HashMap;

/// Vector embedder for semantic code search
pub struct VectorEmbedder {
    device: Device,
    embedding_dim: usize,
    vocab: HashMap<String, usize>,
}

impl VectorEmbedder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            device: Device::Cpu,
            embedding_dim: 384, // Standard embedding dimension
            vocab: HashMap::new(),
        })
    }
    
    /// Generate embeddings for analysis records
    pub async fn embed_records(&self, records: &[AnalysisRecord]) -> Result<Vec<AnalysisRecord>> {
        let mut embedded_records = Vec::new();
        
        for record in records {
            let mut new_record = record.clone();
            
            // Generate embedding based on content
            let embedding = self.generate_embedding(&record.content)?;
            new_record.semantic_embedding = Some(embedding);
            new_record.record_type = RecordType::VectorEmbedding;
            
            embedded_records.push(new_record);
        }
        
        Ok(embedded_records)
    }
    
    /// Generate embedding for text content
    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Simple embedding generation (in production, use a proper model)
        let mut embedding = vec![0.0f32; self.embedding_dim];
        
        // Hash-based embedding for demonstration
        let hash = self.simple_hash(text);
        for i in 0..self.embedding_dim {
            embedding[i] = ((hash.wrapping_mul(i as u64 + 1)) % 1000) as f32 / 1000.0;
        }
        
        // Normalize the embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }
        
        Ok(embedding)
    }
    
    /// Simple hash function for demonstration
    fn simple_hash(&self, text: &str) -> u64 {
        let mut hash = 5381u64;
        for byte in text.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }
    
    /// Search for similar records using cosine similarity
    pub async fn search_similar(
        &self,
        query: &str,
        records: &[AnalysisRecord],
        limit: usize,
    ) -> Result<Vec<&AnalysisRecord>> {
        let query_embedding = self.generate_embedding(query)?;
        
        let mut similarities: Vec<(f32, &AnalysisRecord)> = Vec::new();
        
        for record in records {
            if let Some(ref embedding) = record.semantic_embedding {
                let similarity = self.cosine_similarity(&query_embedding, embedding);
                similarities.push((similarity, record));
            }
        }
        
        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(similarities
            .into_iter()
            .take(limit)
            .map(|(_, record)| record)
            .collect())
    }
    
    /// Calculate cosine similarity between two embeddings
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
    
    /// Generate embeddings using Candle tensors (for future enhancement)
    #[allow(dead_code)]
    fn generate_tensor_embedding(&self, text: &str) -> Result<Tensor> {
        // Tokenize text (simplified)
        let tokens: Vec<f32> = text
            .chars()
            .take(self.embedding_dim)
            .map(|c| c as u32 as f32 / 1000.0)
            .collect();
        
        let mut padded_tokens = tokens;
        padded_tokens.resize(self.embedding_dim, 0.0);
        
        Tensor::from_vec(padded_tokens, (1, self.embedding_dim), &self.device)
            .map_err(|e| SolfunmemeError::Embedding(format!("Tensor creation failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisMetadata, RecordType};
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_embedding_generation() {
        let embedder = VectorEmbedder::new().unwrap();
        
        let embedding = embedder.generate_embedding("hello world").unwrap();
        assert_eq!(embedding.len(), 384);
        
        // Check normalization
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }
    
    #[tokio::test]
    async fn test_similarity_search() {
        let embedder = VectorEmbedder::new().unwrap();
        
        let records = vec![
            AnalysisRecord {
                id: Uuid::new_v4().to_string(),
                file_path: "test1.rs".to_string(),
                record_type: RecordType::Parsing,
                content: "function hello world".to_string(),
                metadata: AnalysisMetadata {
                    timestamp: chrono::Utc::now(),
                    analyzer_version: "1.0.0".to_string(),
                    file_size: 100,
                    line_count: 10,
                    complexity_score: 0.5,
                    mathematical_rigor: 0.8,
                },
                semantic_embedding: Some(embedder.generate_embedding("function hello world").unwrap()),
                sexpr_trace: None,
                neural_signature: None,
            },
            AnalysisRecord {
                id: Uuid::new_v4().to_string(),
                file_path: "test2.rs".to_string(),
                record_type: RecordType::Parsing,
                content: "struct data type".to_string(),
                metadata: AnalysisMetadata {
                    timestamp: chrono::Utc::now(),
                    analyzer_version: "1.0.0".to_string(),
                    file_size: 200,
                    line_count: 20,
                    complexity_score: 0.3,
                    mathematical_rigor: 0.9,
                },
                semantic_embedding: Some(embedder.generate_embedding("struct data type").unwrap()),
                sexpr_trace: None,
                neural_signature: None,
            },
        ];
        
        let results = embedder.search_similar("hello function", &records, 1).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("hello"));
    }
}
