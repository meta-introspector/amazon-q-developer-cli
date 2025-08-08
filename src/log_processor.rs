use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub line_number: usize,
    pub content: String,
    pub context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionData {
    pub entries: Vec<LogEntry>,
    pub total_lines: usize,
    pub quality_score: f64,
    pub key_insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetadata {
    pub processed_at: DateTime<Utc>,
    pub source_file: String,
    pub total_lines: usize,
    pub sections_found: usize,
    pub processing_time_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    pub technical_depth: u8,      // 1-10 scale
    pub documentation_completeness: u8, // 1-10 scale
    pub error_rate: f64,          // percentage
    pub actionable_insights: usize,
    pub priority_tier: u8,        // 1=High, 2=Medium, 3=Lower
}

pub struct LogProcessor {
    patterns: HashMap<String, Regex>,
}

impl LogProcessor {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // From conversation summary: 11 structured sections
        patterns.insert("git_operations".to_string(), 
            Regex::new(r"git\s+(add|commit|push|pull|clone|status|branch|remote|init)").unwrap());
        patterns.insert("cargo_operations".to_string(), 
            Regex::new(r"cargo\s+(run|build|test|clippy|fmt|install)").unwrap());
        patterns.insert("emoji_analysis".to_string(), 
            Regex::new(r"emoji|🧮|🔢|✨|💫|🔥|🌊|📊|🎯|💎|🕳️|📱|🌙|⭐|🌌|🚀|🪐").unwrap());
        patterns.insert("ragit_work".to_string(), 
            Regex::new(r"ragit|solfunmeme|clifford|multivector|term_quiz_master").unwrap());
        patterns.insert("dataset_generation".to_string(), 
            Regex::new(r"dataset|jsonl|records|validation|parquet|hugging.*face").unwrap());
        patterns.insert("technical_discussions".to_string(), 
            Regex::new(r"⠋ Thinking|🛠️|Using tool|✓ Successfully|⠦ Thinking|⠇ Thinking").unwrap());
        patterns.insert("file_operations".to_string(), 
            Regex::new(r"fs_read|fs_write|Reading directory|Creating file|mkdir|touch").unwrap());
        patterns.insert("error_handling".to_string(), 
            Regex::new(r"error:|Error|failed|Failed|exception|panic|unwrap").unwrap());
        patterns.insert("results_summaries".to_string(), 
            Regex::new(r"### .*Results|## .*Summary|📊|🏆|✅|SUMMARY|Results:").unwrap());
        patterns.insert("code_snippets".to_string(), 
            Regex::new(r"```|fn |struct |impl |use |mod |pub |let |match").unwrap());
        patterns.insert("general".to_string(), 
            Regex::new(r".*").unwrap()); // Catch-all
            
        Self { patterns }
    }

    pub fn process_log_file<P: AsRef<Path>>(&self, log_path: P) -> Result<HashMap<String, SectionData>, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        
        // Read file content
        let content = fs::read_to_string(&log_path)?;
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        
        println!("Processing {} lines from {:?}", total_lines, log_path.as_ref());
        
        // Initialize sections
        let mut sections: HashMap<String, Vec<LogEntry>> = HashMap::new();
        for section_name in self.patterns.keys() {
            sections.insert(section_name.clone(), Vec::new());
        }
        
        // Process each line
        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;
            
            // Get context (2 lines before and after)
            let context_start = if i >= 2 { i - 2 } else { 0 };
            let context_end = std::cmp::min(lines.len(), i + 3);
            let context: Vec<String> = lines[context_start..context_end]
                .iter()
                .map(|s| s.to_string())
                .collect();
            
            let entry = LogEntry {
                line_number,
                content: line.to_string(),
                context,
            };
            
            // Check which sections this line matches
            let mut matched_any = false;
            for (section_name, pattern) in &self.patterns {
                if section_name != "general" && pattern.is_match(line) {
                    sections.get_mut(section_name).unwrap().push(entry.clone());
                    matched_any = true;
                }
            }
            
            // Always add to general section
            sections.get_mut("general").unwrap().push(entry);
        }
        
        // Convert to SectionData with quality assessment
        let mut result = HashMap::new();
        for (section_name, entries) in sections {
            let quality_score = self.calculate_quality_score(&entries);
            let key_insights = self.extract_key_insights(&section_name, &entries);
            
            let section_data = SectionData {
                total_lines: entries.len(),
                entries,
                quality_score,
                key_insights,
            };
            
            result.insert(section_name, section_data);
        }
        
        let processing_time = start_time.elapsed().as_millis();
        println!("Processing completed in {}ms", processing_time);
        println!("Found {} sections with content", result.len());
        
        Ok(result)
    }
    
    fn calculate_quality_score(&self, entries: &[LogEntry]) -> f64 {
        if entries.is_empty() {
            return 0.0;
        }
        
        let mut score = 0.0;
        let mut total_indicators = 0;
        
        for entry in entries {
            let line = &entry.content;
            
            // Technical content indicators (+3 points from summary)
            if line.contains("impl ") || line.contains("struct ") || line.contains("fn ") {
                score += 3.0;
                total_indicators += 1;
            }
            
            // Achievement indicators (+2 points from summary)
            if line.contains("✅") || line.contains("Successfully") || line.contains("completed") {
                score += 2.0;
                total_indicators += 1;
            }
            
            // Code implementation indicators (+2 points from summary)
            if line.contains("```") || line.contains("cargo run") || line.contains("git commit") {
                score += 2.0;
                total_indicators += 1;
            }
            
            // Error indicators (-1 point)
            if line.contains("error:") || line.contains("failed") || line.contains("Error") {
                score -= 1.0;
                total_indicators += 1;
            }
        }
        
        if total_indicators == 0 {
            return 1.0; // Neutral score for sections without indicators
        }
        
        // Normalize to 0-10 scale
        let normalized = (score / total_indicators as f64) * 2.0 + 5.0;
        normalized.max(0.0).min(10.0)
    }
    
    fn extract_key_insights(&self, section_name: &str, entries: &[LogEntry]) -> Vec<String> {
        let mut insights = Vec::new();
        
        match section_name {
            "emoji_analysis" => {
                // Look for emoji discoveries and universe system mentions
                for entry in entries {
                    if entry.content.contains("universe") && entry.content.contains("emoji") {
                        insights.push(format!("Line {}: Universe emoji system reference", entry.line_number));
                    }
                    if entry.content.contains("17,817") || entry.content.contains("unique emojis") {
                        insights.push(format!("Line {}: Large emoji dataset reference", entry.line_number));
                    }
                }
            },
            "dataset_generation" => {
                // Look for dataset statistics and achievements
                for entry in entries {
                    if entry.content.contains("22GB") || entry.content.contains("size reduction") {
                        insights.push(format!("Line {}: Large dataset processing achievement", entry.line_number));
                    }
                    if entry.content.contains("parquet") || entry.content.contains("10MB") {
                        insights.push(format!("Line {}: Parquet optimization reference", entry.line_number));
                    }
                }
            },
            "ragit_work" => {
                // Look for ragit integration and solfunmeme references
                for entry in entries {
                    if entry.content.contains("solfunmeme_clifford") || entry.content.contains("term_quiz_master") {
                        insights.push(format!("Line {}: Ragit tool integration", entry.line_number));
                    }
                    if entry.content.contains("multivector") || entry.content.contains("clifford") {
                        insights.push(format!("Line {}: Mathematical framework reference", entry.line_number));
                    }
                }
            },
            _ => {
                // Generic insights for other sections
                let significant_lines: Vec<_> = entries.iter()
                    .filter(|e| e.content.len() > 100 || e.content.contains("✅") || e.content.contains("Successfully"))
                    .take(3)
                    .collect();
                
                for entry in significant_lines {
                    insights.push(format!("Line {}: {}", entry.line_number, 
                        if entry.content.len() > 80 { 
                            format!("{}...", &entry.content[..80]) 
                        } else { 
                            entry.content.clone() 
                        }));
                }
            }
        }
        
        insights
    }
    
    pub fn assess_quality(&self, sections: &HashMap<String, SectionData>) -> HashMap<String, QualityAssessment> {
        let mut assessments = HashMap::new();
        
        for (section_name, section_data) in sections {
            let technical_depth = (section_data.quality_score).round() as u8;
            let documentation_completeness = self.calculate_documentation_completeness(section_data);
            let error_rate = self.calculate_error_rate(section_data);
            let actionable_insights = section_data.key_insights.len();
            
            // Determine priority tier based on conversation summary criteria
            let priority_tier = if technical_depth >= 8 && documentation_completeness >= 7 && error_rate < 0.2 {
                1 // High priority - ready for immediate integration
            } else if technical_depth >= 6 && documentation_completeness >= 5 && error_rate < 0.4 {
                2 // Medium priority - needs some curation
            } else {
                3 // Lower priority - requires significant work
            };
            
            let assessment = QualityAssessment {
                technical_depth,
                documentation_completeness,
                error_rate,
                actionable_insights,
                priority_tier,
            };
            
            assessments.insert(section_name.clone(), assessment);
        }
        
        assessments
    }
    
    fn calculate_documentation_completeness(&self, section_data: &SectionData) -> u8 {
        let mut completeness_score = 0;
        let total_entries = section_data.entries.len();
        
        if total_entries == 0 {
            return 0;
        }
        
        let documented_entries = section_data.entries.iter()
            .filter(|e| e.content.contains("###") || e.content.contains("##") || 
                       e.content.contains("//") || e.content.contains("/*"))
            .count();
        
        let ratio = documented_entries as f64 / total_entries as f64;
        ((ratio * 10.0).round() as u8).min(10)
    }
    
    fn calculate_error_rate(&self, section_data: &SectionData) -> f64 {
        let total_entries = section_data.entries.len();
        
        if total_entries == 0 {
            return 0.0;
        }
        
        let error_entries = section_data.entries.iter()
            .filter(|e| e.content.contains("error:") || e.content.contains("Error") || 
                       e.content.contains("failed") || e.content.contains("Failed"))
            .count();
        
        error_entries as f64 / total_entries as f64
    }
    
    pub fn save_sections_to_files<P: AsRef<Path>>(&self, sections: &HashMap<String, SectionData>, output_dir: P) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = output_dir.as_ref();
        std::fs::create_dir_all(output_path)?;
        
        for (section_name, section_data) in sections {
            let filename = format!("{}.json", section_name);
            let file_path = output_path.join(filename);
            
            let json_content = serde_json::to_string_pretty(section_data)?;
            std::fs::write(file_path, json_content)?;
            
            println!("Saved {} entries to {}.json", section_data.entries.len(), section_name);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_processor_creation() {
        let processor = LogProcessor::new();
        assert!(processor.patterns.contains_key("emoji_analysis"));
        assert!(processor.patterns.contains_key("ragit_work"));
    }
    
    #[test]
    fn test_quality_score_calculation() {
        let processor = LogProcessor::new();
        let entries = vec![
            LogEntry {
                line_number: 1,
                content: "impl EmojiProcessor { fn new() -> Self".to_string(),
                context: vec![],
            },
            LogEntry {
                line_number: 2,
                content: "✅ Successfully processed dataset".to_string(),
                context: vec![],
            },
        ];
        
        let score = processor.calculate_quality_score(&entries);
        assert!(score > 5.0); // Should be above neutral due to positive indicators
    }
}
