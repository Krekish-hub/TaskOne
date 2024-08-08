use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

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

fn topological_sort(dependencies: &HashMap<String, Vec<String>>) -> Result<Vec<String>, String>
{
    let mut in_degree = HashMap::new();
    let mut zero_in_degree = VecDeque::new();

    for (file, deps) in dependencies
    {
        for dep in deps
        {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
        in_degree.entry(file.clone()).or_insert(0);
    }

    for (file, &degree) in &in_degree
    {
        if degree == 0
        {
            zero_in_degree.push_back(file.clone());
        }
    }

    let mut sorted_files = Vec::new();
    while let Some(file) = zero_in_degree.pop_front()
    {
        sorted_files.push(file.clone());
        if let Some(deps) = dependencies.get(&file)
        {
            for dep in deps
            {
                if let Some(degree) = in_degree.get_mut(dep)
                {
                    *degree -= 1;
                    if *degree == 0
                    {
                        zero_in_degree.push_back(dep.clone());
                    }
                }
            }
        }
    }

    if sorted_files.len() == in_degree.len()
    {
        Ok(sorted_files)
    }
    else { Err("Цикличная зависимость".to_string()) }
}

fn main() -> io::Result<()> {
    let mut files_content = HashMap::new();
    let mut dependencies = HashMap::new();
    read_dir_recursive(Path::new("./Folder"), &mut files_content, &mut dependencies)?;

    match topological_sort(&dependencies)
    {
        Ok(sorted_files) =>
            {
                let mut output = String::new();
                for file in sorted_files
                {
                    output.push_str(&files_content[&file]);
                    output.push('\n');
                }
                let mut output_file = fs::File::create("output.txt")?;
                output_file.write_all(output.as_bytes())?;
            }
        Err(err) =>
            {
                println!("Ошибка: {}", err)
            }
    }
    Ok(())
}
