use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = Path::new("invest-contracts/src/docs/contracts");

    let mut protos: Vec<PathBuf> = std::fs::read_dir(proto_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "proto")
                && !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with('.'))
        })
        .collect();

    // Nested google/api/*.proto are imports, not top-level contracts.
    protos.sort();

    println!("cargo:rerun-if-changed={}", proto_dir.display());
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&protos, &[proto_dir.to_path_buf()])?;

    Ok(())
}
