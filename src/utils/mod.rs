use std::path::PathBuf;

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