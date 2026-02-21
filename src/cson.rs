use core::panic;
use std::{fs, path::{Path, PathBuf}, sync::{LazyLock, RwLock}};

use serde::{Deserialize, Serialize};

use crate::{cli::arg::{self, get_args}, toolchain::compiler::Compiler};
use crate::utils;

static CONFIG: LazyLock<RwLock<CsonConfig>> = LazyLock::new(|| {
    RwLock::new({
        let arg = arg::get_args();
        let path = arg.project_dir;

        CsonConfig::new(path.as_path())
    })
});

pub fn get_cson_config() -> &'static RwLock<CsonConfig> {
    &CONFIG
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsonConfig {
    // project settings
    pub project: String,
    pub target_name: String,
    pub toolchain: String,

    // building settings
    pub threads: Option<usize>,

    // temp directory
    #[serde(default = "default_build_dir")]
    pub build_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    // compiler flags
    flags:    Option<Vec<String>>,
    cflags:   Option<Vec<String>>,
    cxxflags: Option<Vec<String>>,

    // source files
    pub sources: Option<Vec<PathBuf>>,

    // compiler defines and includes
    defines: Option<Vec<String>>,
    include: Option<Vec<PathBuf>>,
    link:    Option<Vec<PathBuf>>,
    libs:    Option<Vec<String>>,
}

impl CsonConfig {
    pub fn new(path: &Path) -> CsonConfig {
        let file_path = if path.is_dir() {
            path.join("cson.json")
        } else {
            path.to_path_buf()
        };

        let content = fs::read_to_string(&file_path).expect(
            format!(
                "Failed to read cson.json file from {}",
                file_path.to_string_lossy()
            )
            .as_str(),
        );

        let cson: CsonConfig = serde_json::from_str(&content)
            .expect("Failed to parse cson configuration");

        // Source file check
        if cson.sources.is_none() || cson.sources.as_ref().unwrap().is_empty() {
            panic!("No source files specified in cson configuration");
        }

        // Toolchain check
        let supported_toolchains = ["gnu", "llvm", "msvc"];
        if !supported_toolchains.contains(&cson.toolchain.as_str()) {
            panic!("Unsupported toolchain: {}. Supported toolchains are: {:?}", cson.toolchain, supported_toolchains);
        }

        cson.resolve_paths()
    }

    fn init_dir(path: PathBuf, cda: bool) -> PathBuf {
        let path = if !path.is_absolute() {
            get_args().project_dir.join(path)
        } else {
            path
        };

        if !path.exists() {
            if !cda {
                panic!("Directory {} does not exist", path.to_string_lossy());
            }

            fs::create_dir_all(&path).expect(format!("Failed to create {}", path.to_string_lossy()).as_str());
        }

        utils::normalize_and_canonicalize_path(path)
    }

    fn init_dirs(paths: Vec<PathBuf>, cda: bool) -> Vec<PathBuf> {
        paths.into_iter().map(|path| Self::init_dir(path, cda)).collect()
    }

    fn resolve_paths(self) -> Self {
        let mut cson = self;

        // Create build and output directories if they don't exist
        cson.build_dir  = Self::init_dir(cson.build_dir, true);
        cson.output_dir = Self::init_dir(cson.output_dir, true);

        if let Some(sources) = cson.sources {
            cson.sources = Some(Self::init_dirs(sources, false));
        }
        if let Some(includes) = cson.include {
            cson.include = Some(Self::init_dirs(includes, false));
        }
        if let Some(links) = cson.link {
            cson.link    = Some(Self::init_dirs(links, false));
        }
    
        cson
    }

    pub fn get_define_args<T: Compiler>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(defines) = &self.defines else {
            return args;
        };

        for define in defines {
            args.push(format!("{}{}", T::DEFINE_FLAG_PREFIX, define));
        }

        args
    }

    pub fn get_include_dir_args<T: Compiler>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(include_dirs) = &self.include else {
            return args;
        };

        for include_dir in include_dirs {
            args.push(format!("{}{}", T::INCLUDE_FLAG_PREFIX, include_dir.to_str().unwrap().to_string()));
        }

        args
    }

    pub fn get_link_dir_args<T: Compiler>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(link_dirs) = &self.link else {
            return args;
        };

        for link_dir in link_dirs {
            args.push(format!("{}{}", T::LINK_DIR_FLAG_PREFIX, link_dir.to_str().unwrap().to_string()));
        }

        args
    }

    pub fn get_lib_args<T: Compiler>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(libs) = &self.libs else {
            return args;
        };

        for lib in libs {
            args.push(format!("{}{}", T::LINK_LIB_FLAG_PREFIX, lib));
        }

        args
    }
}

fn default_build_dir() -> PathBuf {
    PathBuf::from("./build").canonicalize().expect("Failed to canonicalize build directory")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./output").canonicalize().expect("Failed to canonicalize output directory")
}

#[test]
fn test_cson() {
    let config = CsonConfig::new("./cson.json".as_ref());
    println!("Project: {:?}", config);
}
