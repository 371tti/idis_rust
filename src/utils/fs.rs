use std::{path::PathBuf, io, fs};

use log::error;

pub fn get_file_path(relative_path: &str) -> Result<PathBuf, io::Error> {
    let binding = std::env::current_exe()?;
    let exe_dir = binding.parent().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get executable directory"))?;
    Ok(exe_dir.join(relative_path))
}

pub fn get_file_string(relative_path: &str) -> Result<String, io::Error> {
    let path = match get_file_path(relative_path) {
        Ok(p) => p,
        Err(e) => {
            error!("cannot get file path: {}", e);
            return Err(e);
        },
    };
    let display_path = path.display().to_string();
    match fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => {
            error!("cannot read file: {}", display_path);
            Err(e)
        }
        
    }
}