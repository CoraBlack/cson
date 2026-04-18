//! Recursive module graph loader.
//!
//! Each module is still configured by its own `cxon.json` and can declare
//! child modules through `modules`.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::cxon::CxonConfig;
use crate::error::fail;
use crate::utils;

#[derive(Debug, Clone)]
pub struct ModuleGraph {
    /// Stable module identity (canonical project directory).
    pub id: PathBuf,
    pub config: CxonConfig,
    /// Direct dependency modules declared by `modules`.
    pub children: Vec<ModuleGraph>,
}

/// Load module graph from root config path or root project directory.
pub fn load_module_graph(root_path: &Path) -> ModuleGraph {
    let mut loaded = HashMap::new();
    let mut visiting = HashSet::new();

    load_node(root_path, &mut loaded, &mut visiting)
}

fn load_node(
    path: &Path,
    loaded: &mut HashMap<PathBuf, ModuleGraph>,
    visiting: &mut HashSet<PathBuf>,
) -> ModuleGraph {
    // Parse current module config first so we know project root and module refs.
    let mut config = CxonConfig::from_path(path);
    let id = utils::normalize_and_canonicalize_path(config.project_dir.clone());

    if let Some(node) = loaded.get(&id) {
        return node.clone();
    }

    if visiting.contains(&id) {
        // DFS cycle detection (A -> B -> ... -> A).
        fail(format!(
            "detected cyclic module dependency at {}",
            id.display()
        ));
    }
    visiting.insert(id.clone());

    let module_refs = config.modules.clone().unwrap_or_default();
    let mut children = Vec::new();

    for module_ref in module_refs {
        // Child path is resolved relative to current module root.
        let module_path = module_ref.path();
        let module_path = config.project_dir.join(module_path);
        let child = load_node(module_path.as_path(), loaded, visiting);

        // Keep a single toolchain family in one module tree for now.
        if child.config.toolchain.to_lowercase() != config.toolchain.to_lowercase() {
            fail(format!(
                "Toolchain mismatch between module {} and child {}",
                config.project_dir.display(),
                child.config.project_dir.display()
            ));
        }

        children.push(child);
    }

    visiting.remove(&id);
    // Graph stores structure in `children`; raw module refs are not needed later.
    config.modules = None;

    let node = ModuleGraph {
        id: id.clone(),
        config,
        children,
    };

    loaded.insert(id, node.clone());
    node
}
