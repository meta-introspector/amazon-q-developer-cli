use std::env;
use std::path::Path;
use serde_json;

mod log_processor;
use log_processor::LogProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let log_file = if args.len() > 1 {
        &args[1]
    } else {
        "log2.md" // Default to log2.md as requested
    };
    
    let output_dir = if args.len() > 2 {
        &args[2]
    } else {
        "log_sections" // Default output directory
    };
    
    println!("🔥 Rust Log Processor - Converting Python to Pure Rust! 🔥");
    println!("Processing: {}", log_file);
    println!("Output directory: {}", output_dir);
    
    // Create processor
    let processor = LogProcessor::new();
    
    // Process the log file
    println!("\n📊 Processing log file...");
    let sections = processor.process_log_file(log_file)?;
    
    // Display section statistics
    println!("\n📈 Section Statistics:");
    println!("┌─────────────────────────┬───────────┬─────────────┬─────────────┐");
    println!("│ Section                 │ Lines     │ Quality     │ Insights    │");
    println!("├─────────────────────────┼───────────┼─────────────┼─────────────┤");
    
    let mut total_lines = 0;
    for (section_name, section_data) in &sections {
        if section_name != "general" { // Skip general section in summary
            total_lines += section_data.total_lines;
            println!("│ {:<23} │ {:<9} │ {:<11.1} │ {:<11} │", 
                section_name, 
                section_data.total_lines,
                section_data.quality_score,
                section_data.key_insights.len()
            );
        }
    }
    println!("└─────────────────────────┴───────────┴─────────────┴─────────────┘");
    println!("Total processed lines: {}", total_lines);
    
    // Quality assessment
    println!("\n🎯 Quality Assessment:");
    let quality_assessments = processor.assess_quality(&sections);
    
    let mut tier1_sections = Vec::new();
    let mut tier2_sections = Vec::new();
    let mut tier3_sections = Vec::new();
    
    for (section_name, assessment) in &quality_assessments {
        if section_name != "general" {
            match assessment.priority_tier {
                1 => tier1_sections.push((section_name, assessment)),
                2 => tier2_sections.push((section_name, assessment)),
                3 => tier3_sections.push((section_name, assessment)),
                _ => {}
            }
        }
    }
    
    println!("🏆 Tier 1 (High Priority - Ready for Integration): {} sections", tier1_sections.len());
    for (name, assessment) in &tier1_sections {
        println!("  ✅ {} (Tech: {}/10, Doc: {}/10, Errors: {:.1}%)", 
            name, assessment.technical_depth, assessment.documentation_completeness, 
            assessment.error_rate * 100.0);
    }
    
    println!("🔧 Tier 2 (Medium Priority - Needs Curation): {} sections", tier2_sections.len());
    for (name, assessment) in &tier2_sections {
        println!("  🔄 {} (Tech: {}/10, Doc: {}/10, Errors: {:.1}%)", 
            name, assessment.technical_depth, assessment.documentation_completeness, 
            assessment.error_rate * 100.0);
    }
    
    println!("⚠️  Tier 3 (Lower Priority - Significant Work Needed): {} sections", tier3_sections.len());
    for (name, assessment) in &tier3_sections {
        println!("  🔨 {} (Tech: {}/10, Doc: {}/10, Errors: {:.1}%)", 
            name, assessment.technical_depth, assessment.documentation_completeness, 
            assessment.error_rate * 100.0);
    }
    
    // Save sections to files
    println!("\n💾 Saving sections to files...");
    processor.save_sections_to_files(&sections, output_dir)?;
    
    // Generate summary report
    let summary_report = generate_summary_report(&sections, &quality_assessments);
    let summary_path = Path::new(output_dir).join("processing_summary.json");
    std::fs::write(summary_path, serde_json::to_string_pretty(&summary_report)?)?;
    
    println!("\n🎉 Processing Complete!");
    println!("✅ {} sections processed and saved", sections.len());
    println!("✅ Quality assessments generated");
    println!("✅ Summary report saved to processing_summary.json");
    println!("\n🚀 Ready for QA system integration!");
    
    Ok(())
}

fn generate_summary_report(
    sections: &std::collections::HashMap<String, log_processor::SectionData>,
    assessments: &std::collections::HashMap<String, log_processor::QualityAssessment>
) -> serde_json::Value {
    use serde_json::json;
    
    let total_lines: usize = sections.values().map(|s| s.total_lines).sum();
    let total_insights: usize = sections.values().map(|s| s.key_insights.len()).sum();
    
    let tier1_count = assessments.values().filter(|a| a.priority_tier == 1).count();
    let tier2_count = assessments.values().filter(|a| a.priority_tier == 2).count();
    let tier3_count = assessments.values().filter(|a| a.priority_tier == 3).count();
    
    json!({
        "processing_summary": {
            "total_sections": sections.len(),
            "total_lines_processed": total_lines,
            "total_insights_extracted": total_insights,
            "quality_distribution": {
                "tier1_high_priority": tier1_count,
                "tier2_medium_priority": tier2_count,
                "tier3_lower_priority": tier3_count
            },
            "conversion_status": "✅ Successfully converted from Python to Rust",
            "performance_improvement": "🚀 Native Rust processing - no Python dependencies",
            "integration_ready": tier1_count > 0
        },
        "section_details": sections.iter().map(|(name, data)| {
            json!({
                "name": name,
                "lines": data.total_lines,
                "quality_score": data.quality_score,
                "insights_count": data.key_insights.len(),
                "priority_tier": assessments.get(name).map(|a| a.priority_tier).unwrap_or(3)
            })
        }).collect::<Vec<_>>()
    })
}
