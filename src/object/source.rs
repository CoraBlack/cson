use std::{path::{Path, PathBuf}, time::SystemTime};

use crate::cli::arg;

pub struct Source {
    src_dir: PathBuf,
    pub modified: Option<SystemTime>
}

impl Source {
    pub fn new(src_path: &Path) -> Self {
        let src_path = if src_path.is_relative() {
            let project_dir = &arg::get_args().project_dir.clone();
            let src_path = project_dir.join(src_path);
            src_path.canonicalize()
                .expect(format!("Invalid source file {}", src_path.display()).as_str())
        } else {
            src_path.to_path_buf()
        };

        let extension = src_path.extension()
            .expect(format!("Invalid source file {}", src_path.display()).as_str())
            .to_str().unwrap();

        if !src_path.exists() {
            eprintln!("Source file {} is not exist", src_path.display());
            std::process::exit(-1);
        }

        if extension == "c" || extension == "cpp" || 
            extension == "cxx" || extension == "cc" ||
            extension == "h" || extension == "hpp" ||
            extension == "hh" || extension == "hxx" {
            // Valid source file extension
        } else {
            // TODO: Error
            eprintln!("Invalid source file {}", src_path.display());
            std::process::exit(-1);
        }

        let mut src = Self { 
            src_dir: src_path.to_path_buf(), 
            modified: None
        };

        let metadata = src_path.metadata();
        match metadata {
            Ok(metadata) => {
                src.modified = match metadata.modified()  {
                    Ok(modified) => Some(modified),
                    Err(_) => None
                };
            }
            Err(_) => {}
        }

        return src;
    }

    pub fn get_path(&self) -> &Path {
        &self.src_dir
    }
}