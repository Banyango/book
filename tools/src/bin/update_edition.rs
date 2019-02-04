use std::env;
use std::io;
use std::io::{Read, Write};
use std::process::exit;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

fn match_files(source_dir: &Path, target_dir: &Path) -> Vec<(PathBuf, PathBuf)> {
    read_dir(source_dir)
        .expect("Unable to read source directory")
        .filter_map(|maybe_entry| maybe_entry.ok())
        .filter_map(|entry| {
            let source_filename = entry.file_name();
            let source_filename = &source_filename.to_string_lossy().into_owned();
                        
            let mut target_path = PathBuf::from(&target_dir);
            
            target_path.push(source_filename);

            if target_path.exists() && entry.file_type().expect("file type ex").is_file() {
                return Some((entry.path(), target_path));
            }

            None
        })
        .collect()
}

fn update_edition_links(source_dir:&str, target_dir:&str) -> io::Result<()> {

    let source_dir = Path::new(source_dir);
    let target_dir = Path::new(target_dir);

    source_dir.metadata().expect("Source Dir does not exist");
    target_dir.metadata().expect("Target Dir does not exist");

    let matched_files = match_files(source_dir, target_dir);

    for file in matched_files { 

        println!("Updating file => {}", file.1.display());

        let mut src = File::open(file.1.as_path())?;
        let mut data = String::new();
        
        src.read_to_string(&mut data)?;  
        drop(src);

        let new_data = data.replace("../index.html", &["../", file.1.file_stem().unwrap().to_str().unwrap(), ".html"].concat());

        let mut dst = File::create(file.1)?;
        dst.write(new_data.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <src-dir> <target-dir>", args[0]);
        exit(1);
    }

    update_edition_links(&args[1], &args[2]).unwrap();
}