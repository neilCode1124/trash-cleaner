use jwalk::WalkDir;
use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

// Define struct for zombie directories
#[derive(Debug, Clone)]
pub struct ZombieDir {
    pub path: PathBuf,
    pub size: u64,
}

// Implement Display trait so inquire can display items in the menu
impl fmt::Display for ZombieDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert bytes to MB, keep 2 decimal places
        let size_mb = self.size as f64 / 1024.0 / 1024.0;
        // Display format: path (size MB)
        write!(f, "{} ({:.2} MB)", self.path.display(), size_mb)
    }
}

// Scan directory for target directories older than specified days
pub fn scan_dir(root: &Path, days: u64, target_dir_name: &str) -> Vec<ZombieDir> {
    // Initialize loading spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}") 
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "])
    );
    spinner.set_message(format!("Searching for '{}' ...", target_dir_name));
    spinner.enable_steady_tick(Duration::from_millis(80));

    let mut zombies = Vec::new(); 
    let mut found_paths = HashSet::new(); // Track found directory paths to avoid scanning nested directories

    // Parallel scanning with jwalk
    for entry in WalkDir::new(root)
        .skip_hidden(false)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        
        // Check if current path is inside an already found directory
        let is_inside_found_dir = found_paths.iter().any(|found_path| {
            path.starts_with(found_path) && &path != found_path
        });
        
        if is_inside_found_dir {
            continue; // Skip scanning inside already found directories
        }
        
        if entry.file_type().is_dir() && entry.file_name() == target_dir_name {
            // Check last modified time
            if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = modified.elapsed() {
                        let days_elapsed = elapsed.as_secs() / 86400;
                        if days_elapsed >= days {
                            // Update loading message dynamically
                            spinner.set_message(format!("Found: {:?}", path.file_name().unwrap()));
                            
                            let size = get_dir_size(&path);
                            zombies.push(ZombieDir {
                                path: path.clone(),
                                size: size,
                            });
                            
                            // Record this path to skip scanning its contents later
                            found_paths.insert(path);
                        }
                    }
                }
            }
        }
    }

    spinner.finish_and_clear();
    zombies
}

// Helper function to calculate directory size
fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}
