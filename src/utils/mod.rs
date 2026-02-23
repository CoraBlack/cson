use std::path::PathBuf;

use crate::{cli::arg, cxon::get_cxon_config, object::source::Source, toolchain::ToolChainTrait};

pub fn normalize_and_canonicalize_path(path: PathBuf) -> PathBuf {
    let canonicalized_path = if !path.is_absolute() {
        path.canonicalize().expect("Failed to canonicalize path")
    } else {
        path
    };
    normalize_path(canonicalized_path)
}

pub fn normalize_and_canonicalize_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths.into_iter().map(|path| normalize_and_canonicalize_path(path)).collect()
}

fn normalize_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)] {
        // 移除 \\?\ 前缀
        let clean_path = path.to_str().unwrap().trim_start_matches("\\\\?\\");
        PathBuf::from(clean_path)
    }
    #[cfg(not(windows))] {
        path
    }
}

pub fn get_object_target_path<T: ToolChainTrait>(src: &Source) -> Result<PathBuf, String> {
    let src_path = src.get_path();

    let obj_sub_path = pathdiff::diff_paths(&src_path, arg::get_args().project_dir);

    let obj_path = get_cxon_config()
        .read()
        .unwrap()
        .build_dir
        .join(obj_sub_path.unwrap());

    if !obj_path.parent().expect("Failed to get the folder of object file").exists() {
        std::fs::create_dir_all(obj_path.parent().unwrap()).expect("Failed to create object file directory");
    }

    Ok(obj_path.with_extension(T::OBJECT_LIB_EXTENSION))
}

pub fn check_executable_exists(executable: &str) -> String {
    which::which(executable).expect(format!("Failed to find executable {} in system", executable).as_str()).to_str().unwrap().to_string()
}
