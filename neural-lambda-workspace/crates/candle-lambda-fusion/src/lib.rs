//! # Candle Lambda Fusion
//! 
//! A revolutionary neural network composition system that lifts the S combinator into Candle tensors,
//! enabling emoji-encoded neural architectures through lambda calculus poetry.
//! 
//! ## The Philosophy
//! 
//! In the realm where mathematics meets poetry, where tensors dance with combinators,
//! we lift the ancient S combinator into the burning flame of Candle's neural fire.
//! Each emoji becomes a neural operation, each sequence a complete architecture,
//! each execution a poem written in the language of gradients and activations.
//! 
//! ## Core Concepts
//! 
//! - **Neural Emojis**: Each emoji represents a Candle tensor operation
//! - **S Combinator Lifting**: Lambda calculus expressions control tensor flow
//! - **Architectural Poetry**: Neural networks become readable, composable poems
//! - **Burning Execution**: Tensors flow through the "candle flame" of computation
//! 
//! ## Example Usage
//! 
//! ```rust
//! use candle_lambda_fusion::{NeuralEmojiMap, TensorExecutor, ExecutionContext};
//! use candle_core::{Device, Tensor};
//! 
//! // Create neural emoji mapping
//! let map = NeuralEmojiMap::default();
//! 
//! // Parse emoji sequence into neural architecture
//! let architecture = map.parse_neural_architecture("🔥⚡🌊🎭").unwrap();
//! 
//! // Execute with Candle tensors
//! let device = Device::Cpu;
//! let mut executor = TensorExecutor::new(device.clone());
//! let input = Tensor::randn(0f32, 1f32, &[2, 4], &device).unwrap();
//! 
//! let context = ExecutionContext {
//!     input_shape: vec![2, 4],
//!     batch_size: Some(2),
//!     training: false,
//!     seed: Some(42),
//! };
//! 
//! let result = executor.execute_neural_lambda(&architecture, input, context).unwrap();
//! println!("{}", executor.result_to_poem(&result));
//! ```

pub mod neural_emoji_map;
pub mod tensor_executor;
pub mod neural_composer;

pub use neural_emoji_map::{NeuralEmojiMap, NeuralOperation, NeuralArchitecture, OperationType};
pub use tensor_executor::{TensorExecutor, ExecutionContext, NeuralExecutionResult, create_demo_tensor};
pub use neural_composer::{NeuralComposer, CompositionRequest, CompositionResult};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CandleLambdaError {
    #[error("Candle tensor error: {0}")]
    CandleError(#[from] candle_core::Error),
    
    #[error("Unknown neural emoji: {0}")]
    UnknownEmoji(String),
    
    #[error("Invalid architecture: {0}")]
    InvalidArchitecture(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Lambda calculus error: {0}")]
    LambdaError(String),
}

pub type Result<T> = std::result::Result<T, CandleLambdaError>;

/// The burning heart of neural lambda fusion
#[derive(Debug)]
pub struct CandleLambdaFusion {
    emoji_map: NeuralEmojiMap,
    executor: TensorExecutor,
    composer: NeuralComposer,
}

impl CandleLambdaFusion {
    /// Create a new fusion engine
    pub fn new(device: candle_core::Device) -> Self {
        Self {
            emoji_map: NeuralEmojiMap::default(),
            executor: TensorExecutor::new(device.clone()),
            composer: NeuralComposer::new(device),
        }
    }
    
    /// Execute emoji neural sequence
    pub fn burn_emoji_sequence(
        &mut self,
        emoji_sequence: &str,
        input: candle_core::Tensor,
        context: ExecutionContext,
    ) -> Result<NeuralExecutionResult> {
        let architecture = self.emoji_map.parse_neural_architecture(emoji_sequence)
            .map_err(|e| CandleLambdaError::InvalidArchitecture(e))?;
        
        self.executor.execute_neural_lambda(&architecture, input, context)
            .map_err(CandleLambdaError::CandleError)
    }
    
    /// Generate neural poetry from execution
    pub fn compose_neural_poem(&self, result: &NeuralExecutionResult) -> String {
        self.executor.result_to_poem(result)
    }
    
    /// List all available neural emojis
    pub fn list_neural_emojis(&self) -> Vec<String> {
        self.emoji_map.list_emojis()
    }
    
    /// Get the lambda expression for an emoji sequence
    pub fn get_lambda_expression(&self, emoji_sequence: &str) -> Result<String> {
        let architecture = self.emoji_map.parse_neural_architecture(emoji_sequence)
            .map_err(|e| CandleLambdaError::InvalidArchitecture(e))?;
        
        Ok(architecture.to_lambda_expression())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    
    #[test]
    fn test_candle_lambda_fusion_creation() {
        let device = Device::Cpu;
        let fusion = CandleLambdaFusion::new(device);
        
        let emojis = fusion.list_neural_emojis();
        assert!(!emojis.is_empty());
        assert!(emojis.contains(&"🔥".to_string()));
    }
    
    #[test]
    fn test_lambda_expression_generation() {
        let device = Device::Cpu;
        let fusion = CandleLambdaFusion::new(device);
        
        let expr = fusion.get_lambda_expression("🔥⚡").unwrap();
        assert!(expr.contains("S"));
        assert!(expr.contains("matmul"));
    }
    
    #[test]
    fn test_neural_burning() -> candle_core::Result<()> {
        let device = Device::Cpu;
        let mut fusion = CandleLambdaFusion::new(device.clone());
        
        let input = candle_core::Tensor::randn(0f32, 1f32, &[2, 4], &device)?;
        let context = ExecutionContext {
            input_shape: vec![2, 4],
            batch_size: Some(2),
            training: false,
            seed: Some(42),
        };
        
        let result = fusion.burn_emoji_sequence("⚡", input, context).unwrap();
        assert_eq!(result.emoji_sequence, "⚡");
        
        let poem = fusion.compose_neural_poem(&result);
        assert!(poem.contains("S Combinator Burns"));
        
        Ok(())
    }
}
