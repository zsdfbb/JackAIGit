use std::error::Error;
use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    // subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Show the diff between the working tree and the index
    Diff {
        /// specify the commit index such as HEAD^
        index: String,
        /// explain the diff between the working tree and the index
        #[arg(short, long)] explain: bool,
    },
    /// Commit the current changes
    Commit {
        /// specify the commit message
        #[arg(short, long)] message: Option<String>,
        /// generate commit message
        #[arg(short, long)] explain: bool,
    },
    /// List all commits
    List {
        /// number of commits
        #[arg(short, long)] number: Option<u32>,
        /// explain selected commit
        #[arg(short, long)] explain: bool,
    },
}

pub fn handle() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Diff {index, explain} => {
            
        }
        Commands::Commit { message, explain } => {
            
        }
        Commands::List { number, explain } => {
           
        }
    }

    Ok(())
}


/*
 * ===========================================================
 * test code
 */
#[cfg(feature = "test")]
fn test_options() {
    println!("command: test options");
}

#[cfg(feature = "test")]
fn test_parse_subcommand() {
    println!("command: test subcommand");

    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { index , explain} => {
            println!("Diff index: {}, explain: {:?}", index, explain);
        }
        Commands::Commit { message, explain } => {
            println!("Commit specify message: {:?}, explain: {:?}", message, explain);
        },
        Commands::List { number, explain } => {
            println!("List num: {:?}, explain: {:?}", number, explain);
        },
    }
}

#[cfg(feature = "test")]
pub fn test() {
    test_options();
    test_parse_subcommand();
}