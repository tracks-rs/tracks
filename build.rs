use std::fs::{self, File, DirEntry};
use std::io::{Write, Result};
use std::path::Path;

fn main() -> Result<()> {
    update_mod_files("src/app/controllers")?;
    update_mod_files("src/app/jobs")?;
    update_mod_files("src/app/models")?;
    update_mod_files("src/app/views")?;
    update_mod_files("src/app/watchers")?;

    update_prelude_file("src/app/models")?;

    Ok(())
}

fn update_mod_files(dir: &str) -> Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut modules = Vec::new();

    for path in paths {
        let entry = path?;
        let path = entry.path();

        // Filter out 'mod.rs' and non-Rust files
        if path.is_file() && path.file_name().unwrap() != "mod.rs" && path.extension().map_or(false, |ext| ext == "rs") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                modules.push(file_stem.to_string());
            }
        }
    }

    let mod_path = Path::new(dir).join("mod.rs");
    let mut mod_file = File::create(mod_path)?;

    for module in modules {
        writeln!(mod_file, "pub mod {};", module)?;
    }

    Ok(())
}

fn update_prelude_file(dir: &str) -> Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut exports = Vec::new();

    for path in paths {
        let entry = path?;
        let path = entry.path();

        if path.is_file() && path.file_name().unwrap() != "mod.rs" && path.file_name().unwrap() != "prelude.rs" && path.extension().map_or(false, |ext| ext == "rs") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                exports.push(format!("pub use super::{}::*;", file_stem));
            }
        }
    }

    let prelude_path = Path::new(dir).join("prelude.rs");
    let mut prelude_file = File::create(prelude_path)?;

    for export in exports {
        writeln!(prelude_file, "{}", export)?;
    }

    Ok(())
}

fn is_rust_file(entry: &DirEntry) -> bool {
    entry.path().extension().map_or(false, |ext| ext == "rs")
}
