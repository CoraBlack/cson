//! CLI argument parsing.
//!
//! Supported forms:
//! - `cxon` (use current directory)
//! - `cxon <project_dir>`
//! - `cxon <path/to/cxon.json>`

use std::{
    env::current_dir,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use crate::{
    error::{fail, fail_option, fail_result},
    utils,
};

static ARGS: LazyLock<Mutex<CliArgs>> = LazyLock::new(|| Mutex::new(CliArgs::new()));

pub fn get_args() -> CliArgs {
    match ARGS.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => fail("failed to access CLI arguments state"),
    }
}

#[derive(Clone)]
pub struct CliArgs {
    pub project_dir: PathBuf,
}

impl CliArgs {
    /// Parse command-line arguments once and normalize target project directory.
    pub fn new() -> Self {
        let arg_col: Vec<String> = std::env::args().collect();

        if arg_col.len() <= 1 {
            let mut project_dir = fail_result(
                current_dir(),
                "failed to get current project directory automatically",
            );
            project_dir = utils::normalize_and_canonicalize_path(project_dir);

            return Self { project_dir };
        }

        let project_dir = PathBuf::from(arg_col[1].clone());
        if !project_dir.exists() {
            fail(format!(
                "cxon project dir is not available: {}",
                project_dir.display()
            ));
        }

        // remove cxon.json if it's included in the path
        let project_dir = if project_dir.is_file()
            && fail_option(
                project_dir.file_name(),
                "invalid project path without file name",
            ) == "cxon.json"
        {
            fail_option(
                project_dir.parent(),
                "invalid cxon.json path without parent directory",
            )
            .to_path_buf()
        } else {
            project_dir
        };

        let project_dir = if project_dir.is_absolute() {
            project_dir
        } else {
            fail_result(
                project_dir.canonicalize(),
                format!("failed to canonicalize path {}", project_dir.display()),
            )
        };

        Self {
            project_dir: utils::normalize_and_canonicalize_path(project_dir),
        }
    }
}
