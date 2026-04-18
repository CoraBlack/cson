//! CLI entrypoint for cxon.
//!
//! The runtime flow is:
//! 1. Parse CLI args and locate root `cxon.json`.
//! 2. Load the module graph recursively.
//! 3. Generate build plan and execute in dependency order.

use crate::{
    build_executor::execute_build_plan, build_plan::BuildPlan, project_graph::load_module_graph,
};

pub mod cli {
    pub mod arg;
}
pub mod object {
    pub mod output;
    pub mod source;
}
pub mod build_executor;
pub mod build_plan;
pub mod compile_commands_json;
pub mod cxon;
pub mod project_graph;
pub mod toolchain;
pub mod utils;

fn main() -> () {
    // CLI accepts either a project directory or a direct `cxon.json` path.
    let args = cli::arg::get_args();
    let module_graph = load_module_graph(args.project_dir.as_path());
    let plan = BuildPlan::from_module_graph(&module_graph);
    execute_build_plan(&plan);
}
