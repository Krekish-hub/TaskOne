use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, read_to_string};
use std::path::{Path, PathBuf};

fn read_dir_recursive(
    dir: &Path,
    files_content: &mut HashMap<String, String>,
    dependencies: &mut HashMap<String, Vec<String>>,
) -> io::Result<()>
{
    if dir.is_dir()
    {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_dir_recursive(&path, files_content, dependencies)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let mut content = String::new();
                fs::File::open(&path)?.read_to_string(&mut content)?;

                let file_path = path.strip_prefix("./").unwrap().to_str().unwrap().to_string();
                files_content.insert(file_path.clone(), content.clone());

                let mut deps = Vec::new();
                for line in content.lines()
                {
                    if line.starts_with("*require '") && line.ends_with("'*")
                    {
                        let dep = line[10..line.len()-2].to_string();
                        deps.push(dep);
                    }
                }
                dependencies.insert(file_path, deps);
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut files_content = HashMap::new();
    let mut dependencies = HashMap::new();
    read_dir_recursive(Path::new("./Folder"), &mut files_content, &mut dependencies)?;

    for (file, deps) in &dependencies
    {
        println!("Name: {}, Dependencies: {:?}", file, deps);
    }
    Ok(())
}
