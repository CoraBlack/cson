use std::{env::current_dir, path::PathBuf, sync::{LazyLock, Mutex}};

static ARGS: LazyLock<Mutex<CliArgs>> = LazyLock::new(|| {
    Mutex::new(CliArgs::new())
});

pub fn get_args() -> CliArgs {
    ARGS.lock().unwrap().clone()
}

#[derive(Clone)]
pub struct CliArgs {
    pub project_dir: PathBuf,
}

impl CliArgs {
    pub fn new() -> Self {
        let arg_col: Vec<String> = std::env::args().collect();

        if arg_col.len() <= 1 {
            return Self {
                project_dir: current_dir()
                    .expect("Failed to get project directory automatically"),
            };
        }

        let project_dir = PathBuf::from(arg_col[1].clone());
        if !project_dir.exists() {
            Err(()).expect("cson project dir is not available")
        }

        // remove cson.json if it's included in the path
        let project_dir = if project_dir.is_file() && project_dir.file_name().unwrap() == "cson.json" {
            project_dir.parent().unwrap().to_path_buf()
        } else {
            project_dir
        };

        let project_dir = 
            if project_dir.is_absolute() { project_dir } else { project_dir.canonicalize().unwrap() };

        Self {
            project_dir: project_dir,
        }
    }
}