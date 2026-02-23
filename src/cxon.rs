use core::panic;
use std::{fs, path::{Path, PathBuf}, sync::{LazyLock, RwLock}};

use serde::{Deserialize, Serialize};

use crate::{cli::arg::{self, get_args}, toolchain::{TargetType, ToolChain, ToolChainTrait}};
use crate::utils;

static CONFIG: LazyLock<RwLock<CxonConfig>> = LazyLock::new(|| {
    RwLock::new({
        let arg = arg::get_args();
        let path = arg.project_dir;

        CxonConfig::new(path.as_path())
    })
});

pub fn get_cxon_config() -> &'static RwLock<CxonConfig> {
    &CONFIG
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CxonConfig {
    pub project: String,

    // build settings
    target_name: Option<String>,
    #[serde(default = "default_target_type")]
    target_type: String, // "executable", "static_lib", "shared_lib"

    // toolchain settings
    pub toolchain: String,
    pub cc:  Option<String>,
    pub cxx: Option<String>,

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

impl CxonConfig {
    pub fn new(path: &Path) -> CxonConfig {
        let file_path = if path.is_dir() {
            path.join("cxon.json")
        } else {
            path.to_path_buf()
        };

        let content = fs::read_to_string(&file_path).expect(
            format!(
                "Failed to read cxon.json file from {}",
                file_path.to_string_lossy()
            )
            .as_str(),
        );

        let mut cxon: CxonConfig = serde_json::from_str(&content)
            .expect("Failed to parse cxon configuration");

        // Target name check
        if cxon.target_name.is_none() {
            cxon.target_name = Some(cxon.project.clone())
        }

        // build target type check
        let target_type = cxon.target_type.clone().to_lowercase();
        if !["executable", "static_lib", "shared_lib"].contains(&target_type.as_str()) {
            panic!("Unsupported target type: {}. Supported target types are: executable, static_lib, shared_lib", cxon.target_type);
        }

        // Source file check
        if cxon.sources.is_none() || cxon.sources.as_ref().unwrap().is_empty() {
            panic!("No source files specified in cxon configuration");
        }

        // Toolchain check
        let supported_toolchains = ["gnu", "llvm", "msvc"];
        if !supported_toolchains.contains(&cxon.toolchain.as_str()) {
            panic!("Unsupported toolchain: {}. Supported toolchains are: {:?}", cxon.toolchain, supported_toolchains);
        }

        cxon.resolve_paths()
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
        let mut cxon = self;

        // Create build and output directories if they don't exist
        cxon.build_dir  = Self::init_dir(cxon.build_dir, true);
        cxon.output_dir = Self::init_dir(cxon.output_dir, true);

        if let Some(sources) = cxon.sources {
            cxon.sources = Some(Self::init_dirs(sources, false));
        }
        if let Some(includes) = cxon.include {
            cxon.include = Some(Self::init_dirs(includes, false));
        }
        if let Some(links) = cxon.link {
            cxon.link    = Some(Self::init_dirs(links, false));
        }
    
        cxon
    }

    pub fn get_target_name(&self) -> String {
        self.target_name.clone().unwrap()
    }

    pub fn get_target_type(&self) -> TargetType {
        match self.target_type.to_lowercase().as_str() {
            "object_lib" => TargetType::ObjectLib,
            "executable" => TargetType::Executable,
            "static_lib" => TargetType::StaticLib,
            "shared_lib" => TargetType::SharedLib,
            _ => panic!("Unsupported target type: {}. Supported target types are: executable, static_lib, shared_lib", self.target_type),
        }
    }

    pub fn get_toolchain(&self) -> ToolChain {
        match self.toolchain.to_lowercase().as_str() {
            "gnu"  => ToolChain::GNU(),
            "llvm" => ToolChain::LLVM(),
            "msvc" => ToolChain::MSVC(),
            _ => panic!("Unsupported toolchain: {}. Supported toolchains are: gnu, llvm, msvc", self.toolchain),
        }
    }

    fn get_compiler_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        if let Some(f) = &self.flags {
            flags.extend(f.clone());
        }

        flags
    }

    pub fn get_cflags(&self) -> Vec<String> {
        let mut flags = self.get_compiler_flags();

        if let Some(f) = &self.cflags {
            flags.extend(f.clone());
        }

        flags
    }

    pub fn get_cxxflags(&self) -> Vec<String> {
        let mut flags = self.get_compiler_flags();

        if let Some(f) = &self.cxxflags {
            flags.extend(f.clone());
        }

        flags
    }

    pub fn get_define_args<T: ToolChainTrait>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(defines) = &self.defines else {
            return args;
        };

        for define in defines {
            args.push(format!("{}{}", T::DEFINE_FLAG_PREFIX, define));
        }

        args
    }

    pub fn get_include_dir_args<T: ToolChainTrait>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(include_dirs) = &self.include else {
            return args;
        };

        for include_dir in include_dirs {
            args.push(format!("{}{}", T::INCLUDE_FLAG_PREFIX, include_dir.to_str().unwrap().to_string()));
        }

        args
    }

    pub fn get_link_dir_args<T: ToolChainTrait>(&self) -> Vec<String> {
        let mut args = Vec::new();
        let Some(link_dirs) = &self.link else {
            return args;
        };

        for link_dir in link_dirs {
            args.push(format!("{}{}", T::LINK_DIR_FLAG_PREFIX, link_dir.to_str().unwrap().to_string()));
        }

        args
    }

    pub fn get_lib_args<T: ToolChainTrait>(&self) -> Vec<String> {
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

fn default_target_type() -> String {
    "execuable".to_string()
}

fn default_build_dir() -> PathBuf {
    PathBuf::from("./build")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./output")
}

#[test]
fn test_cxon() {
    let config = CxonConfig::new("./cxon.json".as_ref());
    println!("Project: {:?}", config);
}
