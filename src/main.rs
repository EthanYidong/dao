use wasm_pack::command as wp_cmd;
use glob::glob;

use cargo_dao::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut attribute_collector = AttributeCollector::default();

    let src_files: Vec<_> = glob("./src/**/*.rs")?
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
        wp_cmd::run_wasm_pack(wp_cmd::Command::Build(wp_cmd::build::BuildOptions {
            target: wp_cmd::build::Target::Web,
            out_name: Some(route),
            out_dir: String::from("./web/pkg"),
            disable_dts: true,
            ..Default::default()
        }))?;
    }

    Ok(())
}
