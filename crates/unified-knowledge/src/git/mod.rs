use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::{DateTime, Utc};
use git2::{Repository, Oid, Commit, DiffOptions};
use uuid::Uuid;
use tracing::{info, warn, debug};

use crate::{LogEntry, SubmoduleInfo, DiffStats, Page, PageNavigation, PageMetadata, Result, UnifiedKnowledgeError};

pub struct GitLogCollector {
    pub root_path: PathBuf,
    pub submodules: Vec<SubmoduleInfo>,
    pub timestamp_ordering: BTreeMap<DateTime<Utc>, LogEntry>,
    pub current_page: usize,
    pub page_size: usize,
    pub ragit_integration: bool,
}

impl GitLogCollector {
    pub fn new<P: AsRef<Path>>(root_path: P, page_size: usize) -> Result<Self> {
        let root_path = root_path.as_ref().to_path_buf();
        
        info!("Initializing GitLogCollector at {:?}", root_path);
        
        let mut collector = Self {
            root_path,
            submodules: Vec::new(),
            timestamp_ordering: BTreeMap::new(),
            current_page: 0,
            page_size,
            ragit_integration: Self::check_ragit_availability(),
        };
        
        // Discover submodules
        collector.discover_submodules()?;
        
        Ok(collector)
    }
    
    fn check_ragit_availability() -> bool {
        // Check if ragit tools are available
        let ragit_check = Command::new("which")
            .arg("term_quiz_master")
            .output();
            
        match ragit_check {
            Ok(output) => {
                let available = output.status.success();
                if available {
                    info!("🎯 ragit term_quiz_master detected - enabling enhanced processing");
                } else {
                    warn!("⚠️ ragit term_quiz_master not found - using basic git processing");
                }
                available
            },
            Err(_) => {
                warn!("⚠️ Could not check for ragit tools - using basic git processing");
                false
            }
        }
    }
    
    pub fn discover_submodules(&mut self) -> Result<()> {
        info!("🔍 Discovering submodules...");
        
        let repo = Repository::open(&self.root_path)?;
        
        // Get submodules from .gitmodules
        if let Ok(submodules) = repo.submodules() {
            for submodule in submodules {
                let name = submodule.name().unwrap_or("unknown").to_string();
                let path = submodule.path().to_string_lossy().to_string();
                let url = submodule.url().unwrap_or("unknown").to_string();
                
                // Get the current commit hash for the submodule
                let commit_hash = if let Some(oid) = submodule.head_id() {
                    oid.to_string()
                } else {
                    "unknown".to_string()
                };
                
                let submodule_info = SubmoduleInfo {
                    name: name.clone(),
                    path: path.clone(),
                    url,
                    commit_hash,
                    last_updated: Utc::now(), // Will be updated when processing logs
                };
                
                self.submodules.push(submodule_info);
                info!("📁 Found submodule: {} at {}", name, path);
            }
        }
        
        // Also add the root repository as a "submodule"
        self.submodules.push(SubmoduleInfo {
            name: "root".to_string(),
            path: ".".to_string(),
            url: "local".to_string(),
            commit_hash: "HEAD".to_string(),
            last_updated: Utc::now(),
        });
        
        info!("✅ Discovered {} submodules (including root)", self.submodules.len());
        Ok(())
    }
    
    pub fn collect_all_submodule_logs(&mut self) -> Result<Vec<LogEntry>> {
        info!("📚 Collecting logs from all submodules...");
        
        let mut all_logs = Vec::new();
        
        for submodule in &self.submodules {
            info!("🔍 Processing submodule: {}", submodule.name);
            
            let submodule_path = self.root_path.join(&submodule.path);
            
            if !submodule_path.exists() {
                warn!("⚠️ Submodule path does not exist: {:?}", submodule_path);
                continue;
            }
            
            match self.collect_logs_from_path(&submodule_path, &submodule.name) {
                Ok(mut logs) => {
                    info!("✅ Collected {} logs from {}", logs.len(), submodule.name);
                    all_logs.append(&mut logs);
                },
                Err(e) => {
                    warn!("⚠️ Failed to collect logs from {}: {}", submodule.name, e);
                }
            }
        }
        
        info!("📊 Total logs collected: {}", all_logs.len());
        
        // Apply ragit processing if available
        if self.ragit_integration {
            all_logs = self.enhance_with_ragit_analysis(all_logs)?;
        }
        
        Ok(all_logs)
    }
    
    fn collect_logs_from_path(&self, path: &Path, submodule_name: &str) -> Result<Vec<LogEntry>> {
        let repo = Repository::open(path)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        
        let mut logs = Vec::new();
        
        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = repo.find_commit(oid)?;
            
            let log_entry = self.create_log_entry_from_commit(&repo, &commit, submodule_name)?;
            logs.push(log_entry);
        }
        
        Ok(logs)
    }
    
    fn create_log_entry_from_commit(&self, repo: &Repository, commit: &Commit, submodule_name: &str) -> Result<LogEntry> {
        let timestamp = DateTime::from_timestamp(commit.time().seconds(), 0)
            .unwrap_or_else(|| Utc::now());
        
        let author = commit.author().name().unwrap_or("unknown").to_string();
        let message = commit.message().unwrap_or("").to_string();
        let commit_hash = commit.id().to_string();
        
        // Get diff statistics
        let diff_stats = self.calculate_diff_stats(repo, commit)?;
        
        // Get changed files
        let files_changed = self.get_changed_files(repo, commit)?;
        
        Ok(LogEntry {
            id: Uuid::new_v4(),
            timestamp,
            commit_hash,
            author,
            message,
            submodule_path: submodule_name.to_string(),
            files_changed,
            diff_stats,
        })
    }
    
    fn calculate_diff_stats(&self, repo: &Repository, commit: &Commit) -> Result<DiffStats> {
        let tree = commit.tree()?;
        
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };
        
        let mut diff_opts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;
        
        let stats = diff.stats()?;
        
        Ok(DiffStats {
            insertions: stats.insertions(),
            deletions: stats.deletions(),
            files_changed: stats.files_changed(),
        })
    }
    
    fn get_changed_files(&self, repo: &Repository, commit: &Commit) -> Result<Vec<String>> {
        let tree = commit.tree()?;
        
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };
        
        let mut diff_opts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;
        
        let mut files = Vec::new();
        
        diff.foreach(
            &mut |delta, _progress| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )?;
        
        Ok(files)
    }
    
    fn enhance_with_ragit_analysis(&self, mut logs: Vec<LogEntry>) -> Result<Vec<LogEntry>> {
        info!("🎯 Enhancing logs with ragit term_quiz_master analysis...");
        
        // For each log entry, run term_quiz_master to extract additional insights
        for log_entry in &mut logs {
            if let Ok(enhanced_data) = self.run_term_quiz_master(&log_entry.message) {
                debug!("Enhanced log {} with ragit data", log_entry.id);
                // The enhancement will be stored in the knowledge extraction phase
                // Here we just mark that ragit processing is available
            }
        }
        
        info!("✅ Ragit enhancement complete");
        Ok(logs)
    }
    
    fn run_term_quiz_master(&self, content: &str) -> Result<String> {
        // Run ragit's term_quiz_master on the commit message
        let output = Command::new("term_quiz_master")
            .arg("--analyze")
            .arg("--input")
            .arg(content)
            .arg("--format")
            .arg("json")
            .output()
            .map_err(|e| UnifiedKnowledgeError::KnowledgeError(format!("Failed to run term_quiz_master: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(UnifiedKnowledgeError::KnowledgeError(
                format!("term_quiz_master failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }
    
    pub fn order_by_timestamp(&mut self, logs: &[LogEntry]) -> &BTreeMap<DateTime<Utc>, LogEntry> {
        info!("📅 Ordering {} logs by timestamp...", logs.len());
        
        // Clear existing ordering
        self.timestamp_ordering.clear();
        
        // Add all logs to timestamp ordering
        for log_entry in logs {
            self.timestamp_ordering.insert(log_entry.timestamp, log_entry.clone());
        }
        
        info!("✅ Ordered {} logs by timestamp", self.timestamp_ordering.len());
        &self.timestamp_ordering
    }
    
    pub fn paginate(&self, page: usize) -> Page<LogEntry> {
        let total_entries = self.timestamp_ordering.len();
        let total_pages = (total_entries + self.page_size - 1) / self.page_size;
        
        let start_idx = page.saturating_sub(1) * self.page_size;
        let end_idx = std::cmp::min(start_idx + self.page_size, total_entries);
        
        let items: Vec<LogEntry> = self.timestamp_ordering
            .values()
            .skip(start_idx)
            .take(self.page_size)
            .cloned()
            .collect();
        
        let (start_time, end_time) = if !items.is_empty() {
            (items.first().unwrap().timestamp, items.last().unwrap().timestamp)
        } else {
            (Utc::now(), Utc::now())
        };
        
        let navigation = PageNavigation {
            previous_timestamp: if page > 1 { 
                self.timestamp_ordering.values().nth(start_idx.saturating_sub(1)).map(|e| e.timestamp)
            } else { 
                None 
            },
            next_timestamp: if end_idx < total_entries {
                self.timestamp_ordering.values().nth(end_idx).map(|e| e.timestamp)
            } else {
                None
            },
            can_continue: end_idx < total_entries,
            can_go_back: page > 1,
            bookmark_id: None,
        };
        
        let metadata = PageMetadata {
            total_concepts: 0, // Will be filled by knowledge extraction
            emoji_density: 0.0,
            quality_score: 0.0,
            reaction_count: 0,
            hot_take_count: 0,
        };
        
        Page {
            page_number: page,
            total_pages,
            items,
            timestamp_range: (start_time, end_time),
            navigation,
            metadata,
        }
    }
    
    pub fn continue_from_timestamp(&self, timestamp: DateTime<Utc>) -> Page<LogEntry> {
        // Find the position of the timestamp and create a page from there
        let position = self.timestamp_ordering
            .keys()
            .position(|&t| t >= timestamp)
            .unwrap_or(0);
        
        let page_number = (position / self.page_size) + 1;
        self.paginate(page_number)
    }
    
    pub fn get_submodule_stats(&self) -> BTreeMap<String, usize> {
        let mut stats = BTreeMap::new();
        
        for entry in self.timestamp_ordering.values() {
            *stats.entry(entry.submodule_path.clone()).or_insert(0) += 1;
        }
        
        stats
    }
}
