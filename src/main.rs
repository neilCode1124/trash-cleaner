use clap::Parser;
use inquire::MultiSelect;
use std::fmt;
use std::path::PathBuf;
use std::process;

mod cleaner;
mod scanner;
mod utils;

// Wrapper enum for selection items
#[derive(Debug, Clone)]
enum SelectItem {
    SelectAll,
    Directory(scanner::ZombieDir),
}

impl fmt::Display for SelectItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectItem::SelectAll => write!(f, "✓ Select All"),
            SelectItem::Directory(zombie) => write!(f, "{}", zombie),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, default_value_t = 60)]
    days: u64,

    #[arg(short, long, default_value = "node")]
    type_: String, // "node" or "python"

    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let root_path = PathBuf::from(&args.path);

    if !root_path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path);
        process::exit(1);
    }

    let target_name = match args.type_.as_str() {
        "node" | "npm" => "node_modules",
        "cargo" | "rust" => "target",
        "python" => "__pycache__",
        _ => {
            eprintln!("Unsupported type: {}", args.type_);
            eprintln!("Supported types: node, cargo, python");
            process::exit(1);
        }
    };

    println!("🚀 Scanning (multi-threaded mode)...");

    // Call scanner
    let zombie_list = scanner::scan_dir(&root_path, args.days, target_name);

    if zombie_list.is_empty() {
        println!("✨ No junk directories found! Your system is clean.");
        return;
    }

    // Calculate total size
    let total_size: u64 = zombie_list.iter().map(|z| z.size).sum();
    println!(
        "📦 Found {} candidate directories | Total: {}",
        zombie_list.len(),
        utils::format_size(total_size)
    );

    // If dry-run, display and exit
    if args.dry_run {
        cleaner::delete_targets(zombie_list, true);
        return;
    }

    // Create selection list with "Select All" option at the beginning
    let mut select_items: Vec<SelectItem> = vec![SelectItem::SelectAll];
    select_items.extend(zombie_list.iter().cloned().map(SelectItem::Directory));

    // Interactive selection
    let selection = MultiSelect::new("Select directories to clean:", select_items)
        .with_page_size(10)
        .with_help_message("Space: toggle | Enter: confirm")
        .prompt();

    match selection {
        Ok(selected_items) => {
            // Check if "Select All" was selected
            let has_select_all = selected_items
                .iter()
                .any(|item| matches!(item, SelectItem::SelectAll));

            let directories_to_delete = if has_select_all {
                // If "Select All" is selected, delete all directories
                zombie_list
            } else {
                // Otherwise, delete only selected directories
                selected_items
                    .into_iter()
                    .filter_map(|item| {
                        if let SelectItem::Directory(zombie) = item {
                            Some(zombie)
                        } else {
                            None
                        }
                    })
                    .collect()
            };

            if !directories_to_delete.is_empty() {
                cleaner::delete_targets(directories_to_delete, false);
            } else {
                println!("No directories selected.");
            }
        }
        Err(_) => println!("Operation cancelled."),
    }
}
