use crate::scanner::ZombieDir;
use crate::utils::format_size;
use trash; 

pub fn delete_targets(targets: Vec<ZombieDir>, dry_run: bool) {
    let mut freed_space = 0;

    for target in &targets {
        if dry_run {
            println!("🔍 [DRY RUN] Would move to trash: {}", target.path.display());
            freed_space += target.size;
        } else {
            // Use trash::delete to move files to system trash
            match trash::delete(&target.path) {
                Ok(_) => {
                    println!("✅ Moved to trash: {}", target.path.display());
                    freed_space += target.size;
                },
                Err(e) => eprintln!("❌ Failed to move {}: {}", target.path.display(), e),
            }
        }
    }

    if dry_run {
        println!("\n🎉 Dry run completed! Estimated space to free: {}", format_size(freed_space));
    } else {
        println!("\n🎉 Cleanup completed! Freed space: {}", format_size(freed_space));
        println!("💡 (Files are in your system trash. Empty trash to permanently free space)");
    }
}
