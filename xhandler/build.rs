use std::path::Path;

use static_files::resource_dir;

/// Embeds the xbuilder and xsender templates
fn main() -> std::io::Result<()> {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    for module in ["xbuilder", "xsender"] {
        let name = format!("{module}_templates");
        let mut templates = resource_dir(format!("./resources/templates/{module}"));
        templates
            .with_generated_filename(Path::new(&out_dir).join(format!("{name}.rs")))
            // Each set needs its own module: both are written into OUT_DIR
            .with_module_name(format!("{name}_sets"));
        templates.build()?;
    }
    Ok(())
}
