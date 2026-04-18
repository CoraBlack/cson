//! Build executor for module-aware plans.
//!
//! Current V2 stage:
//! - Build each target in topological order.
//! - Child targets are built before parents.
//! - Parent targets currently keep their own link inputs (module artifact
//!   injection is the next incremental step).

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    build_plan::{BuildPlan, TargetPlan},
    compile_commands_json::generate_compile_commands_json,
    cxon::CxonConfig,
    object::{output::ObjectCollection, source::Source},
    toolchain::{self, compiler, linker, TargetType, ToolChain, ToolChainTrait},
};

#[derive(Debug, Clone)]
pub struct Artifact {
    /// Module that produced this artifact.
    pub module_id: std::path::PathBuf,
    /// Absolute path to the produced output file.
    pub output: std::path::PathBuf,
    /// Artifact kind used for downstream link decisions.
    pub target_type: TargetType,
}

/// Execute all targets in dependency order.
pub fn execute_build_plan(plan: &BuildPlan) {
    let mut artifacts = std::collections::HashMap::new();

    for id in &plan.order {
        let target = plan
            .targets
            .get(id)
            .expect("Target missing while executing build plan")
            .clone();

        let dep_artifacts = target
            .deps
            .iter()
            .filter_map(|dep| artifacts.get(dep).cloned())
            .collect::<Vec<_>>();

        let artifact = build_target(&target, &dep_artifacts);
        artifacts.insert(target.id.clone(), artifact);
    }

    let root = plan.root_target();
    if root.config.export_compile_commands {
        generate_compile_commands_json(&root.config)
            .expect("Failed to export compile_commands.json");
    }
}

fn build_target(target: &TargetPlan, dep_artifacts: &[Artifact]) -> Artifact {
    match target.config.get_toolchain() {
        ToolChain::GNU() => {
            build_with_toolchain::<toolchain::gnu::GNU>(&target.config, dep_artifacts)
        }
        ToolChain::LLVM() => {
            build_with_toolchain::<toolchain::llvm::LLVM>(&target.config, dep_artifacts)
        }
        ToolChain::MSVC() => {
            build_with_toolchain::<toolchain::msvc::MSVC>(&target.config, dep_artifacts)
        }
    }
}

/// Compile and link one target with a concrete toolchain implementation.
fn build_with_toolchain<T: ToolChainTrait>(
    cxon: &CxonConfig,
    dep_artifacts: &[Artifact],
) -> Artifact {
    let mut cxon = cxon.clone();
    inject_dependency_artifacts(&mut cxon, dep_artifacts);
    let target_type = cxon.get_target_type();

    std::fs::create_dir_all(&cxon.build_dir)
        .expect(format!("Failed to create build dir {}", cxon.build_dir.display()).as_str());
    std::fs::create_dir_all(&cxon.output_dir)
        .expect(format!("Failed to create output dir {}", cxon.output_dir.display()).as_str());

    let sources = cxon.sources.clone().unwrap_or_default();
    let sources = Arc::new(Mutex::new(VecDeque::from(sources)));

    let objects = Arc::new(Mutex::new(ObjectCollection {
        objects: Vec::new(),
    }));

    let thread_count = match cxon.threads {
        Some(count) => count,
        None => std::cmp::max(1, num_cpus::get().saturating_sub(1)),
    };

    let mut compile_threads = Vec::new();

    for _ in 0..thread_count {
        let sources = sources.clone();
        let objects = objects.clone();
        let cxon = cxon.clone();

        compile_threads.push(thread::spawn(move || {
            while let Some(source) = sources.lock().unwrap().pop_back() {
                let source = Source::new(source.clone().as_path(), &cxon.project_dir);
                let obj = compiler::compile::<T>(source, &cxon);
                objects.lock().unwrap().objects.push(obj);
            }
        }));
    }

    for thread in compile_threads {
        thread.join().unwrap();
    }

    linker::link::<T>(objects.lock().unwrap().clone(), target_type, &cxon);

    Artifact {
        module_id: cxon.project_dir.clone(),
        output: get_target_output_path::<T>(&cxon, target_type),
        target_type,
    }
}

/// Inject child module artifacts into the parent target's link inputs.
fn inject_dependency_artifacts(cxon: &mut CxonConfig, dep_artifacts: &[Artifact]) {
    // Ensure deterministic link input set for this target build.
    cxon.clear_extra_link_files();

    for artifact in dep_artifacts {
        match artifact.target_type {
            TargetType::SharedLib | TargetType::StaticLib => {
                // Shared/static dependency outputs are linked as direct files.
                cxon.add_extra_link_file(artifact.output.clone());
            }
            TargetType::ObjectLib => {
                // Object library output is also consumed as direct link input.
                cxon.add_extra_link_file(artifact.output.clone());
            }
            TargetType::Executable => {
                // Executable targets are not linkable dependencies.
            }
        }
    }
}

/// Compute final output path for one target kind and toolchain.
fn get_target_output_path<T: ToolChainTrait>(
    cxon: &CxonConfig,
    target_type: TargetType,
) -> std::path::PathBuf {
    let output_base = cxon.output_dir.join(cxon.get_target_name());

    match target_type {
        TargetType::Executable => {
            if T::EXECUTABLE_EXTENSION.is_empty() {
                output_base
            } else {
                output_base.with_extension(T::EXECUTABLE_EXTENSION)
            }
        }
        TargetType::StaticLib => output_base.with_extension(T::STATIC_LIB_EXTENSION),
        TargetType::SharedLib => output_base.with_extension(T::SHARED_LIB_EXTENSION),
        TargetType::ObjectLib => output_base.with_extension(T::OBJECT_LIB_EXTENSION),
    }
}
