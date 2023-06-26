use clap::{Parser, Subcommand};

/// League of Legends Item Analyzer CLI
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Loliac {
    /// The version of items to use
    #[arg(short, long, default_value_t = String::from("latest"))]
    data_version: String,
    /// Command to execute
    #[command(subcommand)]
    command: LoliacCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoliacCommand {
    /// Get an item by item name
    Get { item_name: String },
    GetId { item_id: String },
    Search { text: String }
}
