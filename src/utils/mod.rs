use std::path::PathBuf;

pub fn normalize_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)] {
        // 移除 \\?\ 前缀
        let clean_path = path.to_str().unwrap().trim_start_matches("\\\\?\\");
        PathBuf::from(clean_path)
    }
    #[cfg(not(windows))] {
        path
    }
}