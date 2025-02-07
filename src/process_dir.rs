use std::fs;
use std::path::Path;
use std::time::SystemTime;

// Recursively process the directory, delete old files and empty folders (for subdirectories)
pub fn process_dir(dir: &Path, cutoff: SystemTime) {
    let mut removed_files = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_dir(&path, cutoff);
                // After processing, try to remove empty subdirectories.
                if let Ok(mut entries) = fs::read_dir(&path) {
                    if entries.next().is_none() {
                        match fs::remove_dir(&path) {
                            Ok(_) => println!("\n🗑️ Empty folder deleted: {:?}", path),
                            Err(e) => eprintln!("Error deleting folder {:?}: {}", path, e),
                        }
                    }
                }
            } else if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff {
                        match fs::remove_file(&path) {
                            Ok(_) => {
                                removed_files += 1;
                                println!("\n🗑️ File deleted: {:?}", path)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};
    use tempfile::tempdir;

    #[test]
    fn test_removes_old_files() {
        // Create a temporary directory and a file inside it.
        let temp_dir = tempdir().unwrap();
        let file_path: PathBuf = temp_dir.path().join("old_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Some test content").unwrap();

        // Set cutoff to a time in the future so that the file is considered old.
        let cutoff = SystemTime::now() + Duration::from_secs(3600);

        // Process the directory; expected to remove the file.
        process_dir(temp_dir.path(), cutoff);

        // Assert that the file no longer exists.
        assert!(!file_path.exists(), "File should have been deleted");
    }

    #[test]
    fn test_preserves_new_files() {
        // Create a temporary directory and a file inside it.
        let temp_dir = tempdir().unwrap();
        let file_path: PathBuf = temp_dir.path().join("new_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Fresh content").unwrap();

        // Set cutoff to a time in the past so that the file is considered new.
        let cutoff = SystemTime::now() - Duration::from_secs(3600);

        // Process the directory; expected to keep the file.
        process_dir(temp_dir.path(), cutoff);

        // Assert that the file still exists.
        assert!(file_path.exists(), "File should not have been deleted");
    }

    #[test]
    fn test_recursive_processing() {
        // Create a temporary directory with a subdirectory.
        let temp_dir = tempdir().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        // Create files inside the subdirectory.
        let old_file: PathBuf = sub_dir.join("old_file.txt");
        let new_file: PathBuf = sub_dir.join("new_file.txt");
        {
            let mut file = File::create(&old_file).unwrap();
            writeln!(file, "Old content").unwrap();
        }
        {
            let mut file = File::create(&new_file).unwrap();
            writeln!(file, "New content").unwrap();
        }

        // For this test, use a cutoff in the future so that both files (in subdirectory) are deleted.
        let cutoff = SystemTime::now() + Duration::from_secs(3600);
        process_dir(temp_dir.path(), cutoff);

        // Assert that files in the subdirectory have been removed.
        assert!(!old_file.exists(), "old_file should have been deleted recursively");
        assert!(!new_file.exists(), "new_file should have been deleted recursively");

        // Now test preservation in a recursive scenario.
        // Recreate a subdirectory with a file.
        let sub_dir2 = temp_dir.path().join("subdir2");
        fs::create_dir(&sub_dir2).unwrap();
        let preserved_file: PathBuf = sub_dir2.join("preserved.txt");
        {
            let mut file = File::create(&preserved_file).unwrap();
            writeln!(file, "I am still new").unwrap();
        }

        // Use a cutoff in the past to ensure the file is not deleted.
        let cutoff_old = SystemTime::now() - Duration::from_secs(3600);
        process_dir(temp_dir.path(), cutoff_old);

        // Assert preserved_file still exists.
        assert!(preserved_file.exists(), "File should not have been deleted recursively");
    }

    #[test]
    fn test_empty_folder_deletion() {
        // Create a temporary directory with an empty subdirectory.
        let temp_dir = tempdir().unwrap();
        let empty_subdir = temp_dir.path().join("empty_folder");
        fs::create_dir(&empty_subdir).unwrap();

        // Set a cutoff that doesn't affect folders.
        let cutoff = SystemTime::now() + Duration::from_secs(3600);
        process_dir(temp_dir.path(), cutoff);

        // Assert that the empty subdirectory has been deleted.
        assert!(!empty_subdir.exists(), "Empty folder should have been deleted");
    }
}
