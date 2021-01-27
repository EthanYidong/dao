use wasm_pack::command::{run_wasm_pack, Command as WasmPackCommand, build::BuildOptions};

use structopt::StructOpt;

use glob::glob;

use std::path::PathBuf;

use cargo_dao::*;

/// Proxy args for wasm-pack build.
/// All args are supported except --out-name, which is used for code splitting.
#[derive(Debug, StructOpt)]
pub struct LocalBuildOptions {
    /// The path to the Rust crate. If not set, searches up the path from the current directory.
    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,

    /// The npm scope to use in package.json, if any.
    #[structopt(long = "scope", short = "s")]
    pub scope: Option<String>,

    #[structopt(long = "mode", short = "m", default_value = "normal")]
    /// Sets steps to be run. [possible values: no-install, normal, force]
    pub mode: wasm_pack::install::InstallMode,

    #[structopt(long = "no-typescript")]
    /// By default a *.d.ts file is generated for the generated JS file, but
    /// this flag will disable generating this TypeScript file.
    pub disable_dts: bool,

    #[structopt(long = "target", short = "t", default_value = "bundler")]
    /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules]
    pub target: wasm_pack::command::build::Target,

    #[structopt(long = "debug")]
    /// Deprecated. Renamed to `--dev`.
    pub debug: bool,

    #[structopt(long = "dev")]
    /// Create a development build. Enable debug info, and disable
    /// optimizations.
    pub dev: bool,

    #[structopt(long = "release")]
    /// Create a release build. Enable optimizations and disable debug info.
    pub release: bool,

    #[structopt(long = "profiling")]
    /// Create a profiling build. Enable optimizations and debug info.
    pub profiling: bool,

    #[structopt(long = "out-dir", short = "d", default_value = "pkg")]
    /// Sets the output directory with a relative path.
    pub out_dir: String,

    #[structopt(last = true)]
    /// List of extra options to pass to `cargo build`
    pub extra_options: Vec<String>,
}

fn duplicate_build_options(opts: &LocalBuildOptions, out_name: String) -> BuildOptions {
    BuildOptions {
        path: opts.path.clone(),
        scope: opts.scope.clone(),
        mode: opts.mode,
        disable_dts: opts.disable_dts,
        target: opts.target,
        debug: opts.debug,
        dev: opts.dev,
        release: opts.release,
        profiling: opts.profiling,
        out_dir: opts.out_dir.clone(),
        out_name: Some(out_name),
        extra_options: opts.extra_options.clone()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wp_args = LocalBuildOptions::from_args();

    let mut attribute_collector = AttributeCollector::default();

    let src_glob = if let Some(path) = &wp_args.path {
        format!("{}/src/**/*.rs", path.as_os_str().to_str().ok_or(String::from("Path is not UTF-8"))?)
    } else {
        String::from("./src/**/*.rs")
    };

    let src_files: Vec<_> = glob(&src_glob)?
        .filter_map(|p| {
            p.ok()
        }).filter_map(|p| {
            parse_file_from_path(&p).ok()
        }).collect(); 
    
    for file in &src_files {
        attribute_collector.collect_attributes(file);
    }

    let dao_routes = filter_attributes(&attribute_collector.attributes);

    for route in dao_routes {
        std::env::set_var("RUSTFLAGS", format!(r#"--cfg dao="{}""#, route));
        run_wasm_pack(WasmPackCommand::Build(duplicate_build_options(&wp_args, route)))?;
    }

    Ok(())
}
