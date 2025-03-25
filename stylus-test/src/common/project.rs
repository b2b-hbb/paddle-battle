// from https://github.com/OpenZeppelin/rust-contracts-stylus/tree/v0.1.1/lib/e2e

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use toml::Table;

/// Returns the path to the compiled wasm binary with name `name`.
///
/// Note that this function works for both workspaces and standalone crates.
///
/// # Errors
///
/// May error if:
///
/// - Unable to read the current executable's path.
/// - The output directory is not `target`.
pub fn get_wasm(name: &str) -> eyre::Result<PathBuf> {
    let name = name.replace('-', "_");
    let manifest_dir = env::current_dir()?.parent().unwrap().to_path_buf();
    let target_dir = manifest_dir.join("target");

    let wasm = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!("{name}.wasm"));

    Ok(wasm)
}

pub fn read_pkg_name<P: AsRef<Path>>(path: P) -> eyre::Result<String> {
    let cargo_toml = path.as_ref().join("Cargo.toml");

    let mut reader = BufReader::new(File::open(cargo_toml)?);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let table = buffer.parse::<Table>()?;
    let name = table["package"]["name"].as_str();

    match name {
        Some(x) => Ok(x.to_owned()),
        None => Err(eyre::eyre!("unable to find package name in toml")),
    }
}
