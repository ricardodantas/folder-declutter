use std::fs;
use std::path::Path;
use std::time::SystemTime;

// Recursively process the directory and delete old files
pub fn process_dir(dir: &Path, cutoff: SystemTime) {
    let mut removed_files = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_dir(&path, cutoff);
            } else if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff {
                        match fs::remove_file(&path) {
                            Ok(_) => {
                                removed_files += 1;
                                println!("\n 🗑️ File deleted: {:?}", path)
                            }
                            Err(e) => eprintln!("Error deleting {:?}: {}", path, e),
                        }
                    }
                }
            }
        }
    }
    if removed_files == 0 {
        println!("\n✅ Done! No files to delete.");
        return;
    }
    println!("\n✅ Done! {} files removed.", removed_files);
}
