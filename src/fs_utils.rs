use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::PathBuf;
use walkdir::WalkDir;

pub trait ClearContents {
    fn delete_recursively(&self);
}

impl ClearContents for PathBuf {
    fn delete_recursively(&self) {
        if self.exists() {
            for entry in read_dir(&self).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    let _ = remove_dir_all(path);
                } else {
                    let _ = remove_file(path);
                }
            }
        }
    }
}

pub fn copy_recursively(root_dir: &PathBuf, source: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let source_path = entry.path();

        let relative = source_path
            .strip_prefix(root_dir)
            .expect("source_path should be under root_dir");

        let dest_path = destination.join(relative);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&dest_path)?;
        } else {
            print!("Copying {} to {}…", source_path.display(), dest_path.display());
            std::fs::copy(source_path, &dest_path)?;
            println!(" Done");
        }
    }

    println!();
    Ok(())
}