use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

pub struct FileWatcher {
    dir: PathBuf,
    state: HashMap<String, SystemTime>,
    changed_dirs: Vec<String>,
}

impl FileWatcher {
    pub fn new(dir: PathBuf, state: Option<HashMap<String, SystemTime>>) -> Self {
        Self {
            dir: dir.clone(),
            state: state.unwrap_or(HashMap::new()),
            changed_dirs: Vec::new(),
        }
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn add(&mut self, path: &str, modified: SystemTime) {
        self.state.insert(path.to_string(), modified);
    }

    pub fn remove(&mut self, path: &str) {
        self.state.remove(path);
    }

    pub fn scan_each(&mut self, changes: &Vec<String>) {
        changes.iter().for_each(|dir| {
            self.scan(Some(PathBuf::from(dir)));
        });
    }

    pub fn scan(&mut self, dir: Option<PathBuf>) {
        self.state = HashMap::new();
        let dir = dir.unwrap_or(self.dir.clone());

        // Iterate over the files in the current directory (skip subdirectories)
        for entry in WalkDir::new(dir)
            .max_depth(1) // Only look at the current directory, not its subdirectories
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path().to_string_lossy().to_string();

                // Check if file is missing, added, or has a different modification time
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    if let Ok(modified) = metadata.modified() {
                        self.state.insert(file_path, modified);
                    }
                }
            }
        }
        ();
    }

    /// get changes ONCE and update state
    pub fn changes(&mut self) -> Vec<String> {
        self.changed_dirs = Vec::new();

        // Iterate over directories, stopping further recursion when a change is found
        for entry in WalkDir::new(self.dir.to_path_buf())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
        // Only look at directories
        {
            let dir_path = entry.path().to_string_lossy().to_string();

            // If a change is detected in the current directory, add it to the list and skip its subdirectories
            if self.has_changes_in_directory(&entry) {
                self.changed_dirs.push(dir_path);
                // Skip recursion into this directory's subdirectories (early exit)
                entry.into_path(); // Discard entry
            }
        }

        self.changed_dirs.clone()
    }

    /// Checks if there are any changes within the directory's immediate files (no recursion).
    pub fn has_changes_in_directory(&mut self, dir: &DirEntry) -> bool {
        // list of all files that where supposed to be in the directory
        let mut expected_files = self
            .state
            .iter()
            .filter(|(k, _)| k.starts_with(&dir.path().to_string_lossy().to_string()))
            .collect::<HashMap<_, _>>();

        // Iterate over the files in the current directory (skip subdirectories)
        for entry in WalkDir::new(dir.path())
            .max_depth(1) // Only look at the current directory, not its subdirectories
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path().to_string_lossy().to_string();

                // Check if file is missing, added, or has a different modification time
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    if let Ok(modified) = metadata.modified() {
                        if let Some(last_mod_time) = self.state.get(&file_path) {
                            if &modified != last_mod_time {
                                // File has changed
                                println!("File changed: {}", file_path);
                                return true;
                            } else {
                                // File has not changed
                                expected_files.remove(&file_path);
                            }
                        } else {
                            // New file added
                            println!("New file detected: {}", file_path);
                            return true;
                        }
                    }
                }
            }
        }

        // Check if any expected files are missing
        for (file_path, _) in expected_files.iter() {
            if !fs::metadata(file_path).is_ok() {
                println!("File missing: {}", file_path);
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_file_watcher() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        // Create a file in the temporary directory
        let file_path = temp_dir_path.join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Initial watch
        let mut watcher = FileWatcher::new(temp_dir_path.to_path_buf(), None);
        watcher.scan(None);
        assert_eq!(watcher.changes().len(), 0);

        // Wait for 1 second
        thread::sleep(Duration::from_secs(1));
        // Modify the file and check for changes once
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world! Modified").unwrap();
        let mut changes = watcher.changes();
        assert_eq!(changes.len(), 1);
        watcher.scan_each(&changes);
        changes = watcher.changes();
        assert_eq!(changes.len(), 0);

        // Wait for 1 second
        thread::sleep(Duration::from_secs(1));
        // Remove the file
        fs::remove_file(&file_path).unwrap();
        changes = watcher.changes();
        assert_eq!(changes.len(), 1);
        watcher.scan_each(&changes);
        // no changes after scan
        assert_eq!(watcher.changes().len(), 0);

        // make a dir in the temp dir and add a file
        let dir_path = temp_dir_path.join("test2_dir");
        fs::create_dir(&dir_path).unwrap();
        let file_path = dir_path.join("test-2.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world! 2").unwrap();
        changes = watcher.changes();
        assert_eq!(changes.len(), 1);
        watcher.scan_each(&changes); // 0 changes

        // remove the dir with all files
        fs::remove_dir_all(&dir_path).unwrap();
        assert_eq!(watcher.changes().len(), 1);

        // Cleanup
        temp_dir.close().unwrap();
    }
}
