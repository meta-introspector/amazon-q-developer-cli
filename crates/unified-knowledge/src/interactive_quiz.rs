use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::process::Command;

use crate::{LogEntry, Reaction, ReactionType, HotTake, KnowledgeFragment, Page, Result, UnifiedKnowledgeError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSession {
    pub session_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub pages_processed: usize,
    pub target_pages: usize,
    pub reactions_generated: Vec<Reaction>,
    pub hot_takes_generated: Vec<HotTake>,
    pub quiz_responses: Vec<QuizResponse>,
    pub learning_metrics: LearningMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResponse {
    pub question_id: String,
    pub question_text: String,
    pub ai_response: String,
    pub confidence: f64,
    pub reasoning: String,
    pub related_concepts: Vec<String>,
    pub emoji_context: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetrics {
    pub concepts_learned: usize,
    pub connections_made: usize,
    pub insights_generated: usize,
    pub glossary_updates: usize,
    pub quality_improvements: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIReactionGenerator {
    pub personality_traits: Vec<String>,
    pub focus_areas: Vec<String>,
    pub reaction_patterns: BTreeMap<String, f64>,
}

impl AIReactionGenerator {
    pub fn new() -> Self {
        Self {
            personality_traits: vec![
                "analytical".to_string(),
                "pattern-seeking".to_string(),
                "connection-making".to_string(),
                "optimization-focused".to_string(),
                "emoji-aware".to_string(),
            ],
            focus_areas: vec![
                "architecture".to_string(),
                "performance".to_string(),
                "code_quality".to_string(),
                "documentation".to_string(),
                "patterns".to_string(),
                "emoji_semantics".to_string(),
            ],
            reaction_patterns: BTreeMap::new(),
        }
    }
    
    pub fn generate_reaction_to_page(&self, page: &Page<LogEntry>) -> Vec<Reaction> {
        let mut reactions = Vec::new();
        
        // Analyze the page content and generate reactions
        for (idx, log_entry) in page.items.iter().enumerate() {
            // Generate different types of reactions based on content analysis
            
            // 1. Technical Insight Reactions
            if self.contains_technical_content(&log_entry.message) {
                reactions.push(Reaction {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    page_number: page.page_number,
                    reaction_type: ReactionType::Insight,
                    content: self.generate_technical_insight(&log_entry.message),
                    confidence: 0.8,
                    emoji_context: self.extract_relevant_emojis(&log_entry.message),
                    target_fragment_id: Some(log_entry.id),
                });
            }
            
            // 2. Pattern Recognition Reactions
            if self.detects_pattern(&log_entry.message) {
                reactions.push(Reaction {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    page_number: page.page_number,
                    reaction_type: ReactionType::Connection,
                    content: self.generate_pattern_observation(&log_entry.message),
                    confidence: 0.7,
                    emoji_context: vec!["🔍".to_string(), "🔗".to_string()],
                    target_fragment_id: Some(log_entry.id),
                });
            }
            
            // 3. Question Generation
            if self.needs_clarification(&log_entry.message) {
                reactions.push(Reaction {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    page_number: page.page_number,
                    reaction_type: ReactionType::Question,
                    content: self.generate_clarifying_question(&log_entry.message),
                    confidence: 0.6,
                    emoji_context: vec!["❓".to_string(), "🤔".to_string()],
                    target_fragment_id: Some(log_entry.id),
                });
            }
            
            // 4. Hot Take Generation
            if self.triggers_hot_take(&log_entry.message) {
                reactions.push(Reaction {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    page_number: page.page_number,
                    reaction_type: ReactionType::HotTake,
                    content: self.generate_hot_take(&log_entry.message),
                    confidence: 0.9,
                    emoji_context: vec!["🔥".to_string(), "💡".to_string()],
                    target_fragment_id: Some(log_entry.id),
                });
            }
        }
        
        reactions
    }
    
    fn contains_technical_content(&self, message: &str) -> bool {
        let technical_keywords = [
            "impl", "struct", "fn", "cargo", "rust", "optimization", 
            "performance", "algorithm", "architecture", "refactor",
            "bug", "fix", "feature", "api", "interface"
        ];
        
        technical_keywords.iter().any(|&keyword| 
            message.to_lowercase().contains(keyword)
        )
    }
    
    fn generate_technical_insight(&self, message: &str) -> String {
        // Analyze the commit message and generate a technical insight
        if message.contains("optimization") || message.contains("performance") {
            format!("🚀 This optimization commit suggests a focus on performance improvements. The approach taken here could be applied to similar bottlenecks in the codebase. Consider benchmarking the impact and documenting the optimization pattern for future reference.")
        } else if message.contains("refactor") {
            format!("🔧 This refactoring indicates architectural evolution. The structural changes here likely improve maintainability and could serve as a template for similar code improvements. Worth analyzing the before/after complexity metrics.")
        } else if message.contains("bug") || message.contains("fix") {
            format!("🐛 This bug fix reveals important system behavior. The root cause analysis here could help prevent similar issues. Consider adding this pattern to the testing strategy and error handling guidelines.")
        } else {
            format!("💡 This technical change shows interesting development patterns. The implementation approach demonstrates good engineering practices that could be documented and shared across the team.")
        }
    }
    
    fn detects_pattern(&self, message: &str) -> bool {
        // Look for patterns in commit messages
        message.contains("similar to") || 
        message.contains("like") ||
        message.contains("pattern") ||
        message.contains("consistent") ||
        message.len() > 100 // Longer messages often contain pattern descriptions
    }
    
    fn generate_pattern_observation(&self, message: &str) -> String {
        format!("🔍 Pattern detected: This commit follows a recognizable development pattern. The approach used here connects to broader architectural decisions and could be part of a systematic improvement strategy. Worth cross-referencing with similar changes in the codebase.")
    }
    
    fn needs_clarification(&self, message: &str) -> bool {
        message.len() < 20 || // Very short messages
        message.contains("TODO") ||
        message.contains("WIP") ||
        message.contains("temp") ||
        message.contains("quick")
    }
    
    fn generate_clarifying_question(&self, message: &str) -> String {
        if message.len() < 20 {
            format!("❓ This commit message is quite brief. What was the specific motivation behind this change? Understanding the context would help connect this to the broader development narrative.")
        } else if message.contains("TODO") {
            format!("🤔 This TODO indicates incomplete work. What are the next steps planned? How does this fit into the overall feature development timeline?")
        } else {
            format!("❓ This change raises interesting questions about the implementation approach. What alternatives were considered? How does this decision impact the overall system architecture?")
        }
    }
    
    fn triggers_hot_take(&self, message: &str) -> bool {
        message.contains("breakthrough") ||
        message.contains("major") ||
        message.contains("significant") ||
        message.contains("revolutionary") ||
        message.contains("game-changing") ||
        self.contains_emoji_significance(message)
    }
    
    fn contains_emoji_significance(&self, message: &str) -> bool {
        // Check for universe emojis or high emoji density
        let universe_emojis = ["🧮", "🔢", "✨", "💫", "🔥", "🌊", "📊", "🎯", "💎", "🕳️", "📱", "🌙", "⭐", "🌌", "🚀", "🪐"];
        universe_emojis.iter().any(|&emoji| message.contains(emoji))
    }
    
    fn generate_hot_take(&self, message: &str) -> String {
        if self.contains_emoji_significance(message) {
            format!("🔥 HOT TAKE: This commit contains universe emoji significance! The emoji patterns here suggest deep semantic meaning in the development process. This could be a breakthrough moment in the emoji-driven development methodology. The mathematical implications through Clifford algebra representation could revolutionize how we understand code semantics!")
        } else if message.contains("breakthrough") {
            format!("🚀 HOT TAKE: This breakthrough commit could be a pivotal moment in the project's evolution! The approach taken here might fundamentally change how we think about this problem domain. This deserves deep analysis and could spawn new research directions!")
        } else {
            format!("💡 HOT TAKE: This seemingly routine commit actually reveals profound insights about the development process. The patterns emerging here could be the key to understanding the deeper architectural philosophy driving this project!")
        }
    }
    
    fn extract_relevant_emojis(&self, message: &str) -> Vec<String> {
        let mut emojis = Vec::new();
        
        // Simple emoji detection - look for common emoji patterns
        let emoji_chars = ['🧮', '🔢', '✨', '💫', '🔥', '🌊', '📊', '🎯', '💎', '🕳', '📱', '🌙', '⭐', '🌌', '🚀', '🪐'];
        for emoji_char in emoji_chars {
            if message.contains(emoji_char) {
                emojis.push(emoji_char.to_string());
            }
        }
        
        // Add contextual emojis based on content
        if message.contains("performance") || message.contains("optimization") {
            emojis.push("🚀".to_string());
        }
        if message.contains("bug") || message.contains("fix") {
            emojis.push("🐛".to_string());
        }
        if message.contains("feature") || message.contains("new") {
            emojis.push("✨".to_string());
        }
        if message.contains("refactor") || message.contains("clean") {
            emojis.push("🔧".to_string());
        }
        
        emojis
    }
    
    pub fn take_quiz_on_content(&self, content: &str) -> Result<QuizResponse> {
        // Simulate taking a quiz using ragit's term_quiz_master
        let quiz_result = self.run_term_quiz_master_quiz(content)?;
        
        // Generate AI response to the quiz
        let response = QuizResponse {
            question_id: Uuid::new_v4().to_string(),
            question_text: format!("Analyze this content: {}", content.chars().take(100).collect::<String>()),
            ai_response: self.generate_quiz_response(content),
            confidence: self.calculate_confidence(content),
            reasoning: self.generate_reasoning(content),
            related_concepts: self.extract_concepts(content),
            emoji_context: self.extract_relevant_emojis(content),
            timestamp: Utc::now(),
        };
        
        Ok(response)
    }
    
    fn run_term_quiz_master_quiz(&self, content: &str) -> Result<String> {
        // Run the actual ragit term_quiz_master tool
        let output = Command::new("term_quiz_master")
            .arg("--quiz")
            .arg("--content")
            .arg(content)
            .arg("--ai-mode")
            .output()
            .map_err(|e| UnifiedKnowledgeError::KnowledgeError(format!("Quiz failed: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            // Fallback to internal quiz generation if tool not available
            Ok(self.generate_internal_quiz(content))
        }
    }
    
    fn generate_internal_quiz(&self, content: &str) -> String {
        format!("Internal quiz analysis of: {}", content.chars().take(50).collect::<String>())
    }
    
    fn generate_quiz_response(&self, content: &str) -> String {
        format!("Based on my analysis, this content demonstrates {} patterns with {} significance. The technical depth suggests {} level implementation with {} architectural implications.", 
            self.analyze_patterns(content),
            self.assess_significance(content),
            self.determine_technical_level(content),
            self.evaluate_architectural_impact(content)
        )
    }
    
    fn analyze_patterns(&self, content: &str) -> &str {
        if content.contains("impl") || content.contains("struct") { "object-oriented" }
        else if content.contains("fn") || content.contains("lambda") { "functional" }
        else if content.contains("async") || content.contains("await") { "asynchronous" }
        else { "procedural" }
    }
    
    fn assess_significance(&self, content: &str) -> &str {
        if self.contains_emoji_significance(content) { "high emoji-semantic" }
        else if content.len() > 200 { "substantial" }
        else if self.contains_technical_content(content) { "technical" }
        else { "moderate" }
    }
    
    fn determine_technical_level(&self, content: &str) -> &str {
        let technical_keywords = ["impl", "struct", "fn", "async", "trait"];
        let technical_indicators = technical_keywords.iter()
            .filter(|&keyword| content.contains(keyword))
            .count();
        match technical_indicators {
            0..=1 => "basic",
            2..=4 => "intermediate",
            _ => "advanced"
        }
    }
    
    fn evaluate_architectural_impact(&self, content: &str) -> &str {
        if content.contains("architecture") || content.contains("design") { "significant" }
        else if content.contains("refactor") || content.contains("restructure") { "moderate" }
        else { "minimal" }
    }
    
    fn calculate_confidence(&self, content: &str) -> f64 {
        let mut confidence: f64 = 0.5; // Base confidence
        
        if self.contains_technical_content(content) { confidence += 0.2; }
        if content.len() > 100 { confidence += 0.1; }
        if self.contains_emoji_significance(content) { confidence += 0.2; }
        
        confidence.min(1.0)
    }
    
    fn generate_reasoning(&self, content: &str) -> String {
        format!("My reasoning is based on: 1) Technical content analysis showing {} complexity, 2) Pattern recognition identifying {} structures, 3) Semantic analysis revealing {} significance, 4) Emoji context providing {} dimensional understanding.",
            if self.contains_technical_content(content) { "high" } else { "low" },
            self.analyze_patterns(content),
            self.assess_significance(content),
            if self.contains_emoji_significance(content) { "multi" } else { "single" }
        )
    }
    
    fn extract_concepts(&self, content: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        
        let concept_keywords = [
            ("rust", "programming_language"),
            ("cargo", "build_system"),
            ("impl", "implementation"),
            ("struct", "data_structure"),
            ("fn", "function"),
            ("async", "asynchronous_programming"),
            ("performance", "optimization"),
            ("refactor", "code_improvement"),
            ("bug", "error_handling"),
            ("feature", "functionality"),
        ];
        
        for (keyword, concept) in concept_keywords.iter() {
            if content.to_lowercase().contains(keyword) {
                concepts.push(concept.to_string());
            }
        }
        
        concepts
    }
}

pub struct InteractiveQuizSession {
    pub session: QuizSession,
    pub ai_reactor: AIReactionGenerator,
}

impl InteractiveQuizSession {
    pub fn new(target_pages: usize) -> Self {
        Self {
            session: QuizSession {
                session_id: Uuid::new_v4(),
                start_time: Utc::now(),
                pages_processed: 0,
                target_pages,
                reactions_generated: Vec::new(),
                hot_takes_generated: Vec::new(),
                quiz_responses: Vec::new(),
                learning_metrics: LearningMetrics {
                    concepts_learned: 0,
                    connections_made: 0,
                    insights_generated: 0,
                    glossary_updates: 0,
                    quality_improvements: 0.0,
                },
            },
            ai_reactor: AIReactionGenerator::new(),
        }
    }
    
    pub fn process_page(&mut self, page: &Page<LogEntry>) -> Result<Vec<Reaction>> {
        println!("🎯 AI Processing Page {} of {}", page.page_number, self.session.target_pages);
        println!("📊 Page contains {} log entries", page.items.len());
        
        // Generate reactions to this page
        let reactions = self.ai_reactor.generate_reaction_to_page(page);
        
        println!("💭 Generated {} reactions:", reactions.len());
        for reaction in &reactions {
            println!("  {} {}: {}", 
                match reaction.reaction_type {
                    ReactionType::Insight => "💡",
                    ReactionType::Question => "❓",
                    ReactionType::Connection => "🔗",
                    ReactionType::HotTake => "🔥",
                    _ => "📝",
                },
                format!("{:?}", reaction.reaction_type),
                reaction.content.chars().take(100).collect::<String>()
            );
        }
        
        // Take quiz on each log entry
        for log_entry in &page.items {
            if let Ok(quiz_response) = self.ai_reactor.take_quiz_on_content(&log_entry.message) {
                self.session.quiz_responses.push(quiz_response);
            }
        }
        
        // Update session metrics
        self.session.pages_processed += 1;
        self.session.learning_metrics.insights_generated += reactions.iter()
            .filter(|r| matches!(r.reaction_type, ReactionType::Insight))
            .count();
        self.session.learning_metrics.connections_made += reactions.iter()
            .filter(|r| matches!(r.reaction_type, ReactionType::Connection))
            .count();
        
        // Store reactions
        self.session.reactions_generated.extend(reactions.clone());
        
        Ok(reactions)
    }
    
    pub fn should_continue(&self) -> bool {
        self.session.pages_processed < self.session.target_pages
    }
    
    pub fn get_session_summary(&self) -> String {
        format!(
            "🎉 Quiz Session Complete!\n\
             📊 Pages Processed: {}/{}\n\
             💭 Total Reactions: {}\n\
             🔥 Hot Takes: {}\n\
             💡 Insights: {}\n\
             🔗 Connections: {}\n\
             ❓ Questions: {}\n\
             📚 Concepts Learned: {}\n\
             ⏱️  Session Duration: {:?}",
            self.session.pages_processed,
            self.session.target_pages,
            self.session.reactions_generated.len(),
            self.session.reactions_generated.iter().filter(|r| matches!(r.reaction_type, ReactionType::HotTake)).count(),
            self.session.reactions_generated.iter().filter(|r| matches!(r.reaction_type, ReactionType::Insight)).count(),
            self.session.reactions_generated.iter().filter(|r| matches!(r.reaction_type, ReactionType::Connection)).count(),
            self.session.reactions_generated.iter().filter(|r| matches!(r.reaction_type, ReactionType::Question)).count(),
            self.session.learning_metrics.concepts_learned,
            Utc::now() - self.session.start_time
        )
    }
}
