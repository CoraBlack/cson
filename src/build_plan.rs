//! Build planning for module-aware builds.
//!
//! `BuildPlan` flattens a recursive module graph into a dependency DAG.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Component;
use std::path::PathBuf;

use crate::cxon::CxonConfig;
use crate::project_graph::ModuleGraph;

#[derive(Debug, Clone)]
pub struct TargetPlan {
    /// Unique module identifier (canonical module root path).
    pub id: PathBuf,
    /// Fully resolved module config used during build execution.
    pub config: CxonConfig,
    /// Direct dependencies of this module.
    pub deps: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BuildPlan {
    pub root_id: PathBuf,
    pub targets: HashMap<PathBuf, TargetPlan>,
    /// Topological order where dependencies always appear before dependents.
    pub order: Vec<PathBuf>,
}

impl BuildPlan {
    /// Build a flat dependency plan from recursive module graph.
    pub fn from_module_graph(root: &ModuleGraph) -> Self {
        let mut targets = HashMap::new();
        collect_targets(root, &mut targets);

        if targets.len() > 1 {
            relocate_module_artifact_dirs(&root.id, &mut targets);
        }

        let mut order = Vec::new();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        topo_visit(&root.id, &targets, &mut visiting, &mut visited, &mut order);

        Self {
            root_id: root.id.clone(),
            targets,
            order,
        }
    }

    pub fn root_target(&self) -> &TargetPlan {
        self.targets
            .get(&self.root_id)
            .expect("Root target not found in build plan")
    }
}

/// For multi-module builds, keep submodule artifacts under root folders.
fn relocate_module_artifact_dirs(root_id: &PathBuf, targets: &mut HashMap<PathBuf, TargetPlan>) {
    let (root_project_dir, root_build_dir, root_output_dir) = {
        let root = targets
            .get(root_id)
            .expect("Root target not found while relocating module artifact dirs");

        (
            root.config.project_dir.clone(),
            root.config.build_dir.clone(),
            root.config.output_dir.clone(),
        )
    };

    for (id, target) in targets.iter_mut() {
        if id == root_id {
            continue;
        }

        let module_rel = module_relative_segment(&root_project_dir, &target.config.project_dir);

        target.config.build_dir = root_build_dir.join("modules").join(&module_rel);
        target.config.output_dir = root_output_dir.join("modules").join(&module_rel);

        fs::create_dir_all(&target.config.build_dir).expect(
            format!(
                "Failed to create relocated build dir {}",
                target.config.build_dir.display()
            )
            .as_str(),
        );
        fs::create_dir_all(&target.config.output_dir).expect(
            format!(
                "Failed to create relocated output dir {}",
                target.config.output_dir.display()
            )
            .as_str(),
        );
    }
}

fn module_relative_segment(root_project_dir: &PathBuf, module_project_dir: &PathBuf) -> PathBuf {
    let Some(rel) = pathdiff::diff_paths(module_project_dir, root_project_dir) else {
        return module_project_dir
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("module"));
    };

    let has_parent = rel
        .components()
        .any(|component| matches!(component, Component::ParentDir));

    if rel.as_os_str().is_empty() || has_parent {
        return module_project_dir
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("module"));
    }

    rel
}

/// DFS collection from graph nodes into map-based target table.
fn collect_targets(node: &ModuleGraph, targets: &mut HashMap<PathBuf, TargetPlan>) {
    if targets.contains_key(&node.id) {
        return;
    }

    let deps = node.children.iter().map(|child| child.id.clone()).collect();

    let plan = TargetPlan {
        id: node.id.clone(),
        config: node.config.clone(),
        deps,
    };

    targets.insert(node.id.clone(), plan);

    for child in &node.children {
        collect_targets(child, targets);
    }
}

/// DFS-based topological sorting with cycle protection.
fn topo_visit(
    id: &PathBuf,
    targets: &HashMap<PathBuf, TargetPlan>,
    visiting: &mut HashSet<PathBuf>,
    visited: &mut HashSet<PathBuf>,
    order: &mut Vec<PathBuf>,
) {
    if visited.contains(id) {
        return;
    }

    if visiting.contains(id) {
        panic!(
            "Detected cycle while generating build plan at {}",
            id.display()
        );
    }

    visiting.insert(id.clone());

    let target = targets
        .get(id)
        .expect("Target missing during build plan generation");

    for dep in &target.deps {
        topo_visit(dep, targets, visiting, visited, order);
    }

    visiting.remove(id);
    visited.insert(id.clone());
    order.push(id.clone());
}
