//! Utilities for collecting and exporting compile_commands.json.

use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::{cxon::CxonConfig, object::source::Source};

static COMPILE_COMMANDS_LIST: LazyLock<Mutex<Vec<CompileCommand>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn add_compile_command(command: CompileCommand) {
    COMPILE_COMMANDS_LIST.lock().unwrap().push(command);
}

/// Export compile_commands.json to configured output location.
pub fn generate_compile_commands_json(cxon: &CxonConfig) -> Result<(), Box<dyn std::error::Error>> {
    let commands = COMPILE_COMMANDS_LIST.lock().unwrap().clone();
    let compile_commands_json = serde_json::to_string_pretty(&commands)?;

    let path = match cxon.export_compile_commands_path.clone() {
        Some(mut path) => {
            if !path.ends_with("compile_commands.json") {
                path = path.join("compile_commands.json");
            }

            path
        }
        // default path is `build_dir`/compile_commands.json
        None => cxon.build_dir.clone().join("compile_commands.json"),
    };

    std::fs::write(path, compile_commands_json)?;

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileCommand {
    pub directory: String,
    pub command: String,
    pub file: String,
}

impl CompileCommand {
    /// Create an empty compile command entry for one project directory.
    pub fn default(project_dir: &Path) -> Self {
        Self {
            directory: project_dir.to_string_lossy().to_string(),
            command: String::new(),
            file: String::new(),
        }
    }

    /// Initialize command entry from source file path.
    pub fn from_source(source: Source, project_dir: &Path) -> Self {
        let mut command = Self::default(project_dir);
        command.file = source.get_path().to_string_lossy().to_string();
        command
    }
}
