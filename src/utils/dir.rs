use anyhow::Result;
use include_dir;
use lazy_static::lazy_static;
use std::{collections::HashSet, fs, path::PathBuf};

// Junk files without pattern
const JUNK_FILES: [&str; 15] = [
    ".DS_Store",
    ".Trash",
    ".Spotlight-V100",
    ".fseventsd",
    ".AppleDouble",
    ".AppleDB",
    ".Trashes",
    ".TemporaryItems",
    "Thumbs.db",
    "Desktop.ini",
    "$RECYCLE.BIN",
    "System Volume Information",
    "pagefile.sys",
    "hiberfil.sys",
    "swapfile.sys"
];

lazy_static! {
    static ref JUNK_FILES_SET: HashSet<String> = JUNK_FILES.iter().map(|s| s.to_string()).collect();
}

// Walk through a directory
pub fn walk_dir<F, T>(dir: &PathBuf, mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf) -> T,
{
    for entry in fs::read_dir(dir)? {
        let _ = callback(entry?.path());
    }

    Ok(())
}

// Compare the contents of two directories
pub fn compare_dir(dir: &PathBuf, other: &PathBuf, optional_files: &Vec<String>) -> Result<bool> {
    let mut queue: Vec<(PathBuf, PathBuf)> = Vec::new();
    queue.push((dir.clone(), other.clone()));

    let mut optional_files: HashSet<PathBuf> = optional_files.iter().map(PathBuf::from).collect();

    while queue.len() > 0 {
        if let Some((current_dir, current_other)) = queue.pop() {
            // Folder and file names in the current directory
            let mut sub_dir: HashSet<PathBuf> = HashSet::new();
            let mut sub_other: HashSet<PathBuf> = HashSet::new();

            // Base
            walk_dir(&current_dir, |current| -> Result<()> {
                // Ignore junk files
                if let Some(name) = current.file_name().and_then(|s| s.to_str()) {
                    if JUNK_FILES_SET.contains(name) {
                        return Ok(());
                    }
                }

                // Get relative path
                if let Ok(new_current) = current.strip_prefix(dir) {
                    let new_current = new_current.to_path_buf();

                    // If it is optional, it is not considered in the set
                    if optional_files.contains(&new_current) {
                        optional_files.remove(&new_current);
                    } else {
                        sub_dir.insert(new_current);
                    }
                }

                Ok(())
            })?;

            // Muestra
            walk_dir(&current_other, |current| {
                if let Ok(new_current) = current.strip_prefix(other) {
                    sub_other.insert(new_current.to_path_buf());
                }
            })?;

            // Check if the current level is the same
            if sub_dir.is_subset(&sub_other) {
                for path in sub_dir {
                    let new_dir = current_dir.join(&path);
                    let new_other = current_other.join(&path);

                    if new_dir.is_dir() {
                        queue.push((new_dir, new_other));
                    }
                }
            } else {
                return Ok(false);
            }
        }
    }

    return Ok(true);
}

// Deeply rebuild a directory found in the binary
pub fn rebuild_dir(from: &include_dir::Dir, to: &PathBuf) -> Result<()> {
    for entry in from.entries() {
        let path = to.join(entry.path());

        match entry {
            include_dir::DirEntry::Dir(dir) => {
                if !path.exists() {
                    fs::create_dir_all(&path)?;
                }

                rebuild_dir(dir, &to)?;
            }
            include_dir::DirEntry::File(file) => {
                let path = to.join(file.path());

                if !path.exists() {
                    fs::write(&path, file.contents())?;
                }
            }
        }
    }

    Ok(())
}

// Move between project folders
pub fn up<F, T>(home: &PathBuf, current_dir: &PathBuf, callback: &mut F) -> Option<T>
where
    F: FnMut(PathBuf) -> Option<T>,
{
    let mut current = Some(current_dir.as_path());

    loop {
        if let Some(path) = current {
            // The project must be inside HOME
            if path == home {
                break;
            }

            if let Some(result) = callback(path.to_path_buf()) {
                return Some(result);
            } // If it is None, continue

            current = path.parent();
        } else {
            break;
        }
    }

    return None;
}