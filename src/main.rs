use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn read_dir_recursive(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_dir_recursive(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                println!("Name: {:?}", path);
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    read_dir_recursive(Path::new("./Folder"))?;
    Ok(())
}
