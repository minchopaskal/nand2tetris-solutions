use std::{path::{Path, PathBuf}, fs};

pub fn get_files(path: &Path, res: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if let Some("jack") = ext.to_str() {
                res.push(path.to_owned());
            }
        }
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            get_files(&path, res)?;
        }
    }
        
    Ok(())
}