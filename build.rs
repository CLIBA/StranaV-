//! Embeds the local `assets/` folder (made by `strana extract` from the player's copy of the
//! original) into the executable. Without the folder the build still works: the game then
//! looks for assets on disk and extracts them on first run.

use std::path::{Path, PathBuf};

fn collect(dir: &Path, root: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let path = e.path();
        if path.is_dir() {
            collect(&path, root, out);
        } else if let Ok(rel) = path.strip_prefix(root) {
            out.push((rel.to_string_lossy().replace('\\', "/"), path));
        }
    }
}

fn main() {
    let root = std::env::var_os("STRANA_EMBED_ASSETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("assets"));
    println!("cargo:rerun-if-env-changed=STRANA_EMBED_ASSETS");
    println!("cargo:rerun-if-changed={}", root.display());

    let mut files = Vec::new();
    if root.join("game.json").is_file() {
        collect(&root, &root, &mut files);
        files.sort();
    }
    // An APK has nowhere to extract the original into, so it needs the assets built in.
    if files.is_empty() && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        panic!("{} has no game.json: run `cargo run -- extract StranaV.exe` before building for Android", root.display());
    }
    let mut src = String::from("pub static FILES: &[(&str, &[u8])] = &[\n");
    for (name, path) in &files {
        let abs = path.canonicalize().unwrap_or_else(|_| path.clone());
        src += &format!("    ({name:?}, include_bytes!({:?})),\n", abs.display().to_string());
    }
    src += "];\n";
    let out = Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("embedded_assets.rs");
    std::fs::write(out, src).unwrap();
}
