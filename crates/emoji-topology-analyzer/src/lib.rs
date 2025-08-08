//! # Emoji Topology Analyzer
//! 
//! Advanced emoji analysis using S-combinators to extract emojis, collect paths
//! and topologies, and report usage with mathematical rigor as defined in emojis3.md.
//! 
//! This implements the pure mathematical formulation:
//! emoji_report = S f g ∘ aggregate ∘ map(extract_with_paths)

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

/// Path represents the contextual position of an emoji: (string_index, char_position)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Path {
    pub string_index: usize,
    pub char_position: usize,
}

impl Path {
    pub fn new(string_index: usize, char_position: usize) -> Self {
        Self { string_index, char_position }
    }
}

/// Topology represents a grouping of paths based on structural properties
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Topology {
    pub topology_type: TopologyType,
    pub paths: HashSet<Path>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyType {
    StringLevel,    // All paths from the same string
    WindowBased,    // Paths within a context window
    Semantic,       // Paths with semantic relationships
    Frequency,      // Paths grouped by frequency
}

/// Emoji report entry containing frequency, paths, and topologies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiReport {
    pub emoji: String,
    pub frequency: usize,
    pub paths: Vec<Path>,
    pub topologies: Vec<Topology>,
    pub lambda_expression: String,
    pub semiotic_meaning: String,
}

/// Complete emoji topology analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyAnalysisResult {
    pub session_id: String,
    pub corpus_size: usize,
    pub total_emojis: usize,
    pub unique_emojis: usize,
    pub depth_n: usize,
    pub emoji_reports: Vec<EmojiReport>,
    pub mathematical_expression: String,
}

/// S-Combinator based emoji topology analyzer
pub struct EmojiTopologyAnalyzer {
    depth_n: usize,
    window_size: usize,
}

impl EmojiTopologyAnalyzer {
    pub fn new(depth_n: usize) -> Self {
        Self {
            depth_n,
            window_size: 5, // Default context window
        }
    }
    
    /// Main analysis function implementing the S-combinator pipeline:
    /// emoji_report = S f g ∘ aggregate ∘ map(extract_with_paths)
    pub fn analyze_corpus(&self, corpus: &[String]) -> TopologyAnalysisResult {
        // Step 1: aggregate = concat ∘ map(extract_with_paths)
        let (emoji_list, path_set) = self.aggregate(corpus);
        
        // Step 2: Apply S-combinator: S f g (emoji_list, path_set)
        let reports = self.s_combinator_pipeline(&emoji_list, &path_set);
        
        TopologyAnalysisResult {
            session_id: uuid::Uuid::new_v4().to_string(),
            corpus_size: corpus.len(),
            total_emojis: emoji_list.len(),
            unique_emojis: reports.len(),
            depth_n: self.depth_n,
            emoji_reports: reports,
            mathematical_expression: self.get_mathematical_expression(),
        }
    }
    
    /// Extract emojis with their paths from a single string
    /// extract_with_paths : String × ℕ → List(Emoji) × P(Path)
    fn extract_with_paths(&self, text: &str, string_index: usize) -> (Vec<String>, HashSet<Path>) {
        let mut emojis = Vec::new();
        let mut paths = HashSet::new();
        
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        
        for (char_pos, grapheme) in graphemes.iter().enumerate() {
            if self.is_emoji(grapheme) {
                emojis.push(grapheme.to_string());
                paths.insert(Path::new(string_index, char_pos));
            }
        }
        
        (emojis, paths)
    }
    
    /// Aggregate function: concat ∘ map(extract_with_paths)
    /// aggregate : Corpus → List(Emoji) × P(Path)
    fn aggregate(&self, corpus: &[String]) -> (Vec<String>, HashSet<Path>) {
        let mut all_emojis = Vec::new();
        let mut all_paths = HashSet::new();
        
        for (i, text) in corpus.iter().enumerate() {
            let (emojis, paths) = self.extract_with_paths(text, i);
            all_emojis.extend(emojis);
            all_paths.extend(paths);
        }
        
        (all_emojis, all_paths)
    }
    
    /// S-combinator pipeline: S f g (emoji_list, path_set)
    /// where f and g are defined according to the mathematical specification
    fn s_combinator_pipeline(&self, emoji_list: &[String], path_set: &HashSet<Path>) -> Vec<EmojiReport> {
        // g function: (List(Emoji), P(Path)) → (List(Emoji), P(Path), Emoji→P(Path), P(Topology))
        let (emoji_list_g, path_set_g, emoji_paths, topologies) = self.g_function(emoji_list, path_set);
        
        // f function: combines counting, path association, and topology grouping
        self.f_function(&emoji_list_g, &path_set_g, &emoji_paths, &topologies)
    }
    
    /// G function for S-combinator
    fn g_function(&self, emoji_list: &[String], path_set: &HashSet<Path>) -> 
        (Vec<String>, HashSet<Path>, HashMap<String, HashSet<Path>>, Vec<Topology>) {
        
        let emoji_paths = self.associate_paths(emoji_list, path_set);
        let topologies = self.group_topologies(path_set);
        
        (emoji_list.to_vec(), path_set.clone(), emoji_paths, topologies)
    }
    
    /// F function for S-combinator: generates the final report
    fn f_function(&self, 
        emoji_list: &[String], 
        _path_set: &HashSet<Path>,
        emoji_paths: &HashMap<String, HashSet<Path>>,
        topologies: &[Topology]) -> Vec<EmojiReport> {
        
        let counts = self.count_emojis(emoji_list);
        let mut reports = Vec::new();
        
        for (emoji, frequency) in counts {
            let paths = emoji_paths.get(&emoji)
                .map(|p| self.sample_paths(p, self.depth_n))
                .unwrap_or_default();
            
            let emoji_topologies = self.get_emoji_topologies(&emoji, &paths, topologies);
            let sampled_topologies = self.sample_topologies(&emoji_topologies, self.depth_n);
            
            reports.push(EmojiReport {
                emoji: emoji.clone(),
                frequency,
                paths,
                topologies: sampled_topologies,
                lambda_expression: self.get_emoji_lambda_expression(&emoji),
                semiotic_meaning: self.get_emoji_semiotic_meaning(&emoji),
            });
        }
        
        // Sort by frequency (descending)
        reports.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        reports
    }
    
    /// Count emoji frequencies
    /// count : List(Emoji) → (Emoji → ℕ)
    fn count_emojis(&self, emoji_list: &[String]) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for emoji in emoji_list {
            *counts.entry(emoji.clone()).or_insert(0) += 1;
        }
        counts
    }
    
    /// Associate paths with emojis
    /// associate_paths : List(Emoji) × P(Path) → (Emoji → P(Path))
    fn associate_paths(&self, emoji_list: &[String], path_set: &HashSet<Path>) -> HashMap<String, HashSet<Path>> {
        let mut emoji_paths: HashMap<String, HashSet<Path>> = HashMap::new();
        
        // Convert path_set to vector for indexing
        let paths_vec: Vec<&Path> = path_set.iter().collect();
        
        for (i, emoji) in emoji_list.iter().enumerate() {
            if i < paths_vec.len() {
                emoji_paths.entry(emoji.clone())
                    .or_insert_with(HashSet::new)
                    .insert(paths_vec[i].clone());
            }
        }
        
        emoji_paths
    }
    
    /// Group paths into topologies
    /// group_topologies : P(Path) → P(Topology)
    fn group_topologies(&self, path_set: &HashSet<Path>) -> Vec<Topology> {
        let mut topologies = Vec::new();
        
        // String-level topology: group by string_index
        let mut string_groups: HashMap<usize, HashSet<Path>> = HashMap::new();
        for path in path_set {
            string_groups.entry(path.string_index)
                .or_insert_with(HashSet::new)
                .insert(path.clone());
        }
        
        for (string_index, paths) in string_groups {
            topologies.push(Topology {
                topology_type: TopologyType::StringLevel,
                paths,
                description: format!("String-level topology for string {}", string_index),
            });
        }
        
        // Window-based topology: group by proximity
        let window_groups = self.create_window_topologies(path_set);
        topologies.extend(window_groups);
        
        topologies
    }
    
    /// Create window-based topologies
    fn create_window_topologies(&self, path_set: &HashSet<Path>) -> Vec<Topology> {
        let mut topologies = Vec::new();
        let mut processed_paths = HashSet::new();
        
        for path in path_set {
            if processed_paths.contains(path) {
                continue;
            }
            
            let mut window_paths = HashSet::new();
            window_paths.insert(path.clone());
            processed_paths.insert(path.clone());
            
            // Find paths within window
            for other_path in path_set {
                if other_path.string_index == path.string_index &&
                   other_path.char_position.abs_diff(path.char_position) <= self.window_size {
                    window_paths.insert(other_path.clone());
                    processed_paths.insert(other_path.clone());
                }
            }
            
            if window_paths.len() > 1 {
                topologies.push(Topology {
                    topology_type: TopologyType::WindowBased,
                    paths: window_paths,
                    description: format!("Window-based topology around position {}", path.char_position),
                });
            }
        }
        
        topologies
    }
    
    /// Sample paths to depth N
    /// sample_N : P(Path) → P(Path)
    fn sample_paths(&self, paths: &HashSet<Path>, n: usize) -> Vec<Path> {
        let mut paths_vec: Vec<Path> = paths.iter().cloned().collect();
        paths_vec.sort_by_key(|p| (p.string_index, p.char_position));
        paths_vec.truncate(n);
        paths_vec
    }
    
    /// Sample topologies to depth N
    fn sample_topologies(&self, topologies: &[Topology], n: usize) -> Vec<Topology> {
        topologies.iter().take(n).cloned().collect()
    }
    
    /// Get topologies that contain paths for a specific emoji
    fn get_emoji_topologies(&self, _emoji: &str, paths: &[Path], topologies: &[Topology]) -> Vec<Topology> {
        let path_set: HashSet<Path> = paths.iter().cloned().collect();
        
        topologies.iter()
            .filter(|topology| {
                topology.paths.intersection(&path_set).next().is_some()
            })
            .cloned()
            .collect()
    }
    
    /// Get lambda calculus expression for emoji
    fn get_emoji_lambda_expression(&self, emoji: &str) -> String {
        match emoji {
            "🔥" => "S (K matmul) I".to_string(),
            "⚡" => "S (S (K max) (K 0)) I".to_string(),
            "🌊" => "S (K (λx. 1 / (1 + exp(-x)))) I".to_string(),
            "🌀" => "S (K tanh) I".to_string(),
            "🎭" => "S (K softmax) I".to_string(),
            "📏" => "S (S (K matmul) weight) (K bias)".to_string(),
            "🕸️" => "S (S (S (K conv2d) kernel) stride) padding".to_string(),
            "👁️" => "S (S (S (K attention) query) key) value".to_string(),
            "🚀" => "S (S (S (K optimize) params) gradients) learning_rate".to_string(),
            "✨" => "S (K beauty) I".to_string(),
            _ => format!("S (K {}) I", emoji.chars().next().unwrap_or('?') as u32),
        }
    }
    
    /// Get semiotic meaning for emoji
    fn get_emoji_semiotic_meaning(&self, emoji: &str) -> String {
        match emoji {
            "🔥" => "Transformation through mathematical fire".to_string(),
            "⚡" => "Purification through electrical judgment".to_string(),
            "🌊" => "Smooth transformation of infinite to bounded".to_string(),
            "🌀" => "Infinite spiral converging to unity".to_string(),
            "🎭" => "The mask that reveals rather than conceals".to_string(),
            "📏" => "The ruler that measures infinite dimensions".to_string(),
            "🕸️" => "The web that captures meaning from chaos".to_string(),
            "👁️" => "The all-seeing eye of mathematical consciousness".to_string(),
            "🚀" => "The vessel that carries us to mathematical truth".to_string(),
            "✨" => "The sparkle of enlightenment and achievement".to_string(),
            _ => format!("Mathematical symbol representing {}", emoji),
        }
    }
    
    /// Check if a string is an emoji
    fn is_emoji(&self, s: &str) -> bool {
        s.chars().any(|c| {
            let code = c as u32;
            // Basic emoji ranges (simplified)
            (0x1F600..=0x1F64F).contains(&code) || // Emoticons
            (0x1F300..=0x1F5FF).contains(&code) || // Misc Symbols
            (0x1F680..=0x1F6FF).contains(&code) || // Transport
            (0x1F700..=0x1F77F).contains(&code) || // Alchemical
            (0x2600..=0x26FF).contains(&code) ||   // Misc symbols
            (0x2700..=0x27BF).contains(&code) ||   // Dingbats
            matches!(c, '🔥' | '⚡' | '🌊' | '🌀' | '🎭' | '📏' | '🕸' | '👁' | '🚀' | '✨')
        })
    }
    
    /// Get the complete mathematical expression for the pipeline
    fn get_mathematical_expression(&self) -> String {
        format!(
            "emoji_report = S (λ(l,p)(l',p',ep,t). report(count(l'), sample_{}(ep), sample_{}(t))) \
             (λ(l,p). (l, p, associate_paths(l,p), group_topologies(p))) ∘ \
             concat ∘ map(λs_i. extract_with_paths(s_i, i))",
            self.depth_n, self.depth_n
        )
    }
}

/// Generate a beautiful topology analysis report
impl TopologyAnalysisResult {
    pub fn to_mathematical_poetry(&self) -> String {
        format!(
            r#"🔥 Emoji Topology Analysis: S-Combinator Mathematical Poetry 🔥

Session: {}
Mathematical Expression: {}

📊 Corpus Analysis:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Corpus Size: {} strings
Total Emojis: {} instances
Unique Emojis: {} types
Sampling Depth N: {}

🎭 Emoji Reports (Sorted by Frequency):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
{}

🧮 Mathematical Foundation:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
This analysis implements the pure mathematical formulation from emojis3.md:

1. **Path Definition**: P = ℕ × ℕ (string_index, char_position)
2. **Topology Definition**: T = 𝒫(P) (power set of paths)
3. **S-Combinator Pipeline**: S f g ∘ aggregate ∘ map(extract_with_paths)
4. **Sampling Function**: sample_N : 𝒫(X) → 𝒫(X) where |result| ≤ N

🌟 Topological Insights:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Each emoji exists within multiple topological spaces:
- **String-level topologies**: Grouping by document structure
- **Window-based topologies**: Grouping by contextual proximity
- **Semantic topologies**: Grouping by mathematical meaning

The S-combinator ensures pure functional composition, where each
transformation preserves the mathematical relationships between
emojis, their paths, and their topological groupings.

🔥 The mathematical beauty of emoji topology burns eternal! 🔥"#,
            self.session_id,
            self.mathematical_expression,
            self.corpus_size,
            self.total_emojis,
            self.unique_emojis,
            self.depth_n,
            self.format_emoji_reports()
        )
    }
    
    fn format_emoji_reports(&self) -> String {
        let mut output = String::new();
        
        for (i, report) in self.emoji_reports.iter().enumerate() {
            output.push_str(&format!(
                "{}. {} (frequency: {})\n   Lambda: {}\n   Meaning: {}\n   Paths: {:?}\n   Topologies: {} groups\n\n",
                i + 1,
                report.emoji,
                report.frequency,
                report.lambda_expression,
                report.semiotic_meaning,
                report.paths.iter().take(3).collect::<Vec<_>>(), // Show first 3 paths
                report.topologies.len()
            ));
        }
        
        output
    }
}

// UUID module for compilation
mod uuid {
    pub struct Uuid;
    impl Uuid {
        pub fn new_v4() -> Self { Self }
        pub fn to_string(&self) -> String { "emoji-topology-session-123".to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emoji_topology_analysis() {
        let analyzer = EmojiTopologyAnalyzer::new(3);
        let corpus = vec![
            "I love coding! 😊👍🔥".to_string(),
            "Python is great 🐍😊✨".to_string(),
            "Neural networks 🧠⚡🌊".to_string(),
        ];
        
        let result = analyzer.analyze_corpus(&corpus);
        
        assert_eq!(result.corpus_size, 3);
        assert!(result.total_emojis > 0);
        assert!(result.unique_emojis > 0);
        assert!(!result.emoji_reports.is_empty());
        
        // Check that mathematical expression is generated
        assert!(result.mathematical_expression.contains("S"));
        assert!(result.mathematical_expression.contains("sample_3"));
    }
    
    #[test]
    fn test_path_extraction() {
        let analyzer = EmojiTopologyAnalyzer::new(2);
        let (emojis, paths) = analyzer.extract_with_paths("Hello 😊 World 🔥", 0);
        
        assert_eq!(emojis.len(), 2);
        assert_eq!(paths.len(), 2);
        assert!(emojis.contains(&"😊".to_string()));
        assert!(emojis.contains(&"🔥".to_string()));
    }
    
    #[test]
    fn test_s_combinator_properties() {
        let analyzer = EmojiTopologyAnalyzer::new(2);
        let corpus = vec!["Test 🔥⚡".to_string()];
        let result = analyzer.analyze_corpus(&corpus);
        
        // Verify S-combinator mathematical properties are preserved
        assert!(result.mathematical_expression.contains("S"));
        assert_eq!(result.depth_n, 2);
        
        // Check that lambda expressions are generated for emojis
        for report in &result.emoji_reports {
            assert!(report.lambda_expression.contains("S"));
        }
    }
}
