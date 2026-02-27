use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

use crate::{cli::arg::get_args, cxon::get_cxon_config, object::source::Source};

static COMPILE_COMMANDS_LIST: LazyLock<Mutex<Vec<CompileCommand>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn add_compile_command(command: CompileCommand) {
    COMPILE_COMMANDS_LIST.lock().unwrap().push(command);
}

pub fn generate_compile_commands_json() -> Result<(), Box<dyn std::error::Error>> {
    let commands = COMPILE_COMMANDS_LIST.lock().unwrap().clone();
    let compile_commands_json = serde_json::to_string_pretty(&commands)?;

    let path = match get_cxon_config()
        .read()
        .unwrap()
        .export_compile_commands_path
        .clone()
    {
        Some(mut path) => {
            if !path.ends_with("compile_commands.json") {
                path = path.join("compile_commands.json");
            }

            path
        }
        // default path is `build_dir`/compile_commands.json
        None => get_cxon_config()
            .read()
            .unwrap()
            .build_dir
            .clone()
            .join("compile_commands.json"),
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
    pub fn from_source(source: Source) -> Self {
        let mut command = Self::default();
        command.file = source.get_path().to_string_lossy().to_string();
        command
    }
}


impl Default for CompileCommand {
    fn default() -> Self {
        Self {
            directory: get_args().project_dir.to_string_lossy().to_string(),
            command: String::new(),
            file: String::new(),
        }
    }
}
