use candle_core::{Device, Tensor};
use candle_lambda_fusion::{
    CandleLambdaFusion, ExecutionContext, CompositionRequest, CompositionType, 
    CompositionParameters, NeuralComposer, create_demo_tensor
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Welcome to the Neural Lambda Fusion Demo! 🔥");
    println!("Where S combinators burn through Candle tensors!\n");
    
    // Initialize the fusion engine
    let device = Device::Cpu;
    let mut fusion = CandleLambdaFusion::new(device.clone());
    
    // Show available neural emojis
    println!("📋 Available Neural Emojis:");
    let emojis = fusion.list_neural_emojis();
    for emoji in &emojis {
        println!("  {}", emoji);
    }
    println!();
    
    // Demo 1: Simple emoji sequence execution
    println!("🎯 Demo 1: Simple Neural Sequence");
    println!("Executing: 🔥⚡🌊 (MatMul -> ReLU -> Sigmoid)");
    
    let input = create_demo_tensor(&device, &[2, 4])?;
    let context = ExecutionContext {
        input_shape: vec![2, 4],
        batch_size: Some(2),
        training: false,
        seed: Some(42),
    };
    
    let result = fusion.burn_emoji_sequence("🔥⚡🌊", input, context.clone())?;
    println!("✨ Execution completed in {}ms", result.execution_time_ms);
    println!("📐 Output shape: {:?}", result.output_shape);
    
    // Generate and display the neural poem
    let poem = fusion.compose_neural_poem(&result);
    println!("\n{}", poem);
    
    // Demo 2: Lambda expression generation
    println!("\n🧮 Demo 2: Lambda Expression Generation");
    let lambda_expr = fusion.get_lambda_expression("🔥⚡🌊")?;
    println!("Lambda expression: {}", lambda_expr);
    
    // Demo 3: Advanced composition with NeuralComposer
    println!("\n🎼 Demo 3: Advanced Neural Composition");
    let mut composer = NeuralComposer::new(device.clone());
    
    // Show available patterns
    println!("🎨 Available Composition Patterns:");
    let patterns = composer.list_patterns();
    for pattern in &patterns {
        if let Some(p) = composer.get_pattern(pattern) {
            println!("  {} - {}: {}", pattern, p.emoji_template, p.description);
        }
    }
    
    // Create a sequential composition
    let composition_request = CompositionRequest {
        base_architecture: "⚡🌊".to_string(),
        composition_type: CompositionType::Sequential,
        parameters: CompositionParameters {
            depth: Some(3),
            width: None,
            skip_probability: None,
            mutation_rate: None,
            temperature: None,
        },
        context: context.clone(),
    };
    
    let composition = composer.compose_architecture(composition_request)?;
    println!("\n🏗️ Sequential Composition Result:");
    println!("Emoji sequence: {}", composition.emoji_sequence);
    println!("Estimated parameters: {}", composition.estimated_parameters);
    println!("\n{}", composition.composition_poem);
    
    // Demo 4: Parallel composition
    println!("\n🌐 Demo 4: Parallel Neural Architecture");
    let parallel_request = CompositionRequest {
        base_architecture: "🔥".to_string(),
        composition_type: CompositionType::Parallel,
        parameters: CompositionParameters {
            depth: None,
            width: Some(4),
            skip_probability: None,
            mutation_rate: None,
            temperature: None,
        },
        context: context.clone(),
    };
    
    let parallel_composition = composer.compose_architecture(parallel_request)?;
    println!("Parallel emoji sequence: {}", parallel_composition.emoji_sequence);
    println!("\n{}", parallel_composition.composition_poem);
    
    // Demo 5: Attention-based composition
    println!("\n👁️ Demo 5: Attention Mechanism Composition");
    let attention_request = CompositionRequest {
        base_architecture: "👁️".to_string(),
        composition_type: CompositionType::Attention,
        parameters: CompositionParameters {
            depth: None,
            width: Some(8), // 8 attention heads
            skip_probability: None,
            mutation_rate: None,
            temperature: None,
        },
        context: context.clone(),
    };
    
    let attention_composition = composer.compose_architecture(attention_request)?;
    println!("Attention emoji sequence: {}", attention_composition.emoji_sequence);
    println!("\n{}", attention_composition.composition_poem);
    
    // Demo 6: Execute a composed architecture
    println!("\n🚀 Demo 6: Executing Composed Architecture");
    let input2 = create_demo_tensor(&device, &[2, 4])?;
    let execution_result = composer.execute_composition(&composition, input2, context)?;
    
    println!("Composed execution completed in {}ms", execution_result.execution_time_ms);
    println!("Final output shape: {:?}", execution_result.output_shape);
    
    // Final poem
    println!("\n🎭 Final Neural Poetry:");
    println!("In the realm where mathematics meets art,");
    println!("S combinators and tensors never part.");
    println!("Each emoji a gate, each sequence a song,");
    println!("In Candle's bright flame, where neural nets belong!");
    println!("\n🔥✨ The S combinator has been lifted into the candle! ✨🔥");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo_runs() -> Result<(), Box<dyn std::error::Error>> {
        let device = Device::Cpu;
        let mut fusion = CandleLambdaFusion::new(device.clone());
        
        let input = create_demo_tensor(&device, &[2, 4])?;
        let context = ExecutionContext {
            input_shape: vec![2, 4],
            batch_size: Some(2),
            training: false,
            seed: Some(42),
        };
        
        let result = fusion.burn_emoji_sequence("⚡", input, context)?;
        assert_eq!(result.emoji_sequence, "⚡");
        assert!(!result.lambda_trace.is_empty());
        
        Ok(())
    }
}
