use clap::{Parser, Subcommand};
use std::path::PathBuf;
use unified_knowledge::{GitLogCollector, InteractiveQuizSession, Result};

#[derive(Parser)]
#[command(name = "unified-knowledge")]
#[command(about = "🚀 Unified Knowledge System - AI-powered git log analysis with RL feedback")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive quiz session with AI providing reactions
    Quiz {
        /// Repository path to analyze
        #[arg(short, long, default_value = ".")]
        repo_path: PathBuf,
        
        /// Number of pages to process
        #[arg(short, long, default_value = "5")]
        pages: usize,
        
        /// Page size (entries per page)
        #[arg(short, long, default_value = "10")]
        size: usize,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Collect git logs from all submodules
    CollectLogs {
        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo_path: PathBuf,
        
        /// Include submodules recursively
        #[arg(long)]
        submodules_recursive: bool,
    },
    
    /// Process a specific page of results
    ProcessPage {
        /// Page number to process
        #[arg(short, long)]
        page: usize,
        
        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo_path: PathBuf,
    },
    
    /// Continue from a specific timestamp
    ContinueFrom {
        /// Timestamp to continue from (ISO 8601 format)
        #[arg(short, long)]
        timestamp: String,
        
        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Quiz { repo_path, pages, size, verbose } => {
            run_interactive_quiz(repo_path, pages, size, verbose).await
        },
        Commands::CollectLogs { repo_path, submodules_recursive } => {
            collect_logs_command(repo_path, submodules_recursive).await
        },
        Commands::ProcessPage { page, repo_path } => {
            process_page_command(repo_path, page).await
        },
        Commands::ContinueFrom { timestamp, repo_path } => {
            continue_from_command(repo_path, timestamp).await
        },
    }
}

async fn run_interactive_quiz(repo_path: PathBuf, target_pages: usize, page_size: usize, verbose: bool) -> Result<()> {
    println!("🚀 Starting Interactive Quiz Session");
    println!("🎯 Target: {} pages with {} entries each", target_pages, page_size);
    println!("📁 Repository: {:?}", repo_path);
    println!();
    
    // Initialize git log collector
    let mut git_collector = GitLogCollector::new(&repo_path, page_size)?;
    
    // Collect all logs
    println!("📚 Collecting git logs from all submodules...");
    let logs = git_collector.collect_all_submodule_logs()?;
    println!("✅ Collected {} total log entries", logs.len());
    
    // Order by timestamp
    git_collector.order_by_timestamp(&logs);
    
    // Initialize AI quiz session
    let mut quiz_session = InteractiveQuizSession::new(target_pages);
    
    println!("\n🎭 AI Quiz Session Starting!");
    println!("🤖 I will analyze each page and provide reactions, insights, and hot takes");
    println!("📊 This creates RL feedback data to improve the knowledge system");
    println!();
    
    // Process pages one by one
    let mut current_page = 1;
    
    while quiz_session.should_continue() && current_page <= target_pages {
        println!("{}", "=".repeat(60));
        println!("📄 PROCESSING PAGE {} of {}", current_page, target_pages);
        println!("{}", "=".repeat(60));
        
        // Get the page
        let page = git_collector.paginate(current_page);
        
        if page.items.is_empty() {
            println!("⚠️ No more entries available");
            break;
        }
        
        // Show page summary
        println!("📊 Page Summary:");
        println!("  • Entries: {}", page.items.len());
        println!("  • Time Range: {} to {}", 
            page.timestamp_range.0.format("%Y-%m-%d %H:%M"),
            page.timestamp_range.1.format("%Y-%m-%d %H:%M")
        );
        println!("  • Navigation: {} | {}", 
            if page.navigation.can_go_back { "← Previous" } else { "  (start)" },
            if page.navigation.can_continue { "Next →" } else { "(end)  " }
        );
        println!();
        
        if verbose {
            println!("📝 Log Entries on this page:");
            for (idx, entry) in page.items.iter().enumerate() {
                println!("  {}. [{}] {} - {}", 
                    idx + 1,
                    entry.submodule_path,
                    entry.author,
                    entry.message.chars().take(80).collect::<String>()
                );
            }
            println!();
        }
        
        // AI processes the page and generates reactions
        let reactions = quiz_session.process_page(&page)?;
        
        println!("\n🎯 AI Analysis Complete for Page {}", current_page);
        println!("💭 Generated {} reactions", reactions.len());
        
        if verbose {
            println!("\n📋 Detailed Reactions:");
            for (idx, reaction) in reactions.iter().enumerate() {
                println!("  {}. {} [{:?}] (confidence: {:.1})", 
                    idx + 1,
                    match reaction.reaction_type {
                        unified_knowledge::ReactionType::Insight => "💡",
                        unified_knowledge::ReactionType::Question => "❓",
                        unified_knowledge::ReactionType::Connection => "🔗",
                        unified_knowledge::ReactionType::HotTake => "🔥",
                        unified_knowledge::ReactionType::Enhancement => "⚡",
                        unified_knowledge::ReactionType::Correction => "🔧",
                        unified_knowledge::ReactionType::Bookmark => "📌",
                        unified_knowledge::ReactionType::Flag => "🚩",
                    },
                    reaction.reaction_type,
                    reaction.confidence
                );
                println!("     {}", reaction.content);
                if !reaction.emoji_context.is_empty() {
                    println!("     Emojis: {}", reaction.emoji_context.join(" "));
                }
                println!();
            }
        }
        
        // Pause for dramatic effect and readability
        println!("⏸️  Press Enter to continue to next page...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        current_page += 1;
    }
    
    // Show final session summary
    println!("\n{}", "=".repeat(60));
    println!("🎉 INTERACTIVE QUIZ SESSION COMPLETE!");
    println!("{}", "=".repeat(60));
    println!("{}", quiz_session.get_session_summary());
    
    // Save session data
    let session_data = serde_json::to_string_pretty(&quiz_session.session)?;
    let session_file = format!("quiz_session_{}.json", quiz_session.session.session_id);
    std::fs::write(&session_file, session_data)?;
    
    println!("\n💾 Session data saved to: {}", session_file);
    println!("🔄 This data can now be used to update the knowledge system!");
    
    // Show next steps
    println!("\n📋 Next Steps:");
    println!("1. 🧠 Use the reactions to update the knowledge base");
    println!("2. 📚 Integrate hot takes into the glossary");
    println!("3. 🔗 Use connections to improve semantic linking");
    println!("4. ❓ Address questions to improve documentation");
    println!("5. 🎯 Use insights to guide future development");
    
    Ok(())
}

async fn collect_logs_command(repo_path: PathBuf, _submodules_recursive: bool) -> Result<()> {
    println!("📚 Collecting git logs from {:?}", repo_path);
    
    let mut git_collector = GitLogCollector::new(&repo_path, 50)?;
    let logs = git_collector.collect_all_submodule_logs()?;
    
    println!("✅ Collected {} log entries", logs.len());
    
    // Show submodule statistics
    let stats = git_collector.get_submodule_stats();
    println!("\n📊 Submodule Statistics:");
    for (submodule, count) in stats {
        println!("  • {}: {} commits", submodule, count);
    }
    
    Ok(())
}

async fn process_page_command(repo_path: PathBuf, page_num: usize) -> Result<()> {
    println!("📄 Processing page {} from {:?}", page_num, repo_path);
    
    let mut git_collector = GitLogCollector::new(&repo_path, 10)?;
    let logs = git_collector.collect_all_submodule_logs()?;
    git_collector.order_by_timestamp(&logs);
    
    let page = git_collector.paginate(page_num);
    
    println!("📊 Page {} contains {} entries", page_num, page.items.len());
    for (idx, entry) in page.items.iter().enumerate() {
        println!("  {}. [{}] {} - {}", 
            idx + 1,
            entry.submodule_path,
            entry.author,
            entry.message.chars().take(100).collect::<String>()
        );
    }
    
    Ok(())
}

async fn continue_from_command(repo_path: PathBuf, timestamp_str: String) -> Result<()> {
    println!("⏭️ Continuing from timestamp: {}", timestamp_str);
    
    // Parse timestamp
    let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
        .map_err(|e| unified_knowledge::UnifiedKnowledgeError::KnowledgeError(format!("Invalid timestamp: {}", e)))?
        .with_timezone(&chrono::Utc);
    
    let mut git_collector = GitLogCollector::new(&repo_path, 10)?;
    let _logs = git_collector.collect_all_submodule_logs()?;
    git_collector.order_by_timestamp();
    
    let page = git_collector.continue_from_timestamp(timestamp);
    
    println!("📊 Found {} entries from timestamp", page.items.len());
    for (idx, entry) in page.items.iter().enumerate() {
        println!("  {}. [{}] {} - {}", 
            idx + 1,
            entry.submodule_path,
            entry.timestamp.format("%Y-%m-%d %H:%M"),
            entry.message.chars().take(80).collect::<String>()
        );
    }
    
    Ok(())
}
