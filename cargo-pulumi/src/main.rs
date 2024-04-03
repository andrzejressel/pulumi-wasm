use crate::metadata::GetRelatedCrate;
use anyhow::{anyhow, Error};
use cargo_metadata::{MetadataCommand, Package};
use clap::Parser;
use itertools::Itertools;
use log::{debug, info};
use normpath::PathExt;
use petgraph::visit::{Dfs, Walker};
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Command;

mod graph;
mod metadata;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(short, long)]
    package: Option<String>,
    #[clap(short, long, default_value = "false")]
    no_build: bool,
}

fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let args = App::parse();

    let path = Path::new("./Cargo.toml");

    if std::fs::metadata(path).is_err() {
        return Err(Error::msg(
            "Command run in directory that is not cargo project",
        ));
    }

    let path = path.normalize()?;

    let metadata = MetadataCommand::new().exec()?;

    let target = metadata.target_directory.clone();

    let package = match args.package {
        None => {
            let package = metadata.packages.iter().find(|p| {
                if let Ok(p) = Path::new(&p.manifest_path).normalize() {
                    p == path
                } else {
                    false
                }
            });
            match package {
                None => return Err(Error::msg("Cannot find current package in workspace")),
                Some(p) => p,
            }
        }
        Some(package_name) => {
            let package = metadata.packages.iter().find(|p| p.name == package_name);
            match package {
                None => {
                    return Err(Error::msg(format!(
                        "Cannot find package [{package_name}] in workspace"
                    )))
                }
                Some(p) => p,
            }
        }
    };
    let package = package.clone();

    let g = graph::build(metadata)?;

    let node = g.nodes.get(&package.id).unwrap();

    let dfs = Dfs::new(&g.graph, *node);

    let wasm_dependencies: Vec<_> = dfs
        .iter(&g.graph)
        .flat_map(|nx| g.graph[nx].get_related_crate().clone())
        .collect();

    if wasm_dependencies.is_empty() {
        return Err(anyhow!(
            "Cannot find WASM dependencies for package {}",
            &package.name
        ));
    }

    debug!("WASM dependencies: {:?}", wasm_dependencies);

    let mut all_packages = HashSet::new();
    all_packages.extend([package.name.clone()]);
    all_packages.extend(wasm_dependencies.iter().map(|s| s.to_string()));

    if args.no_build {
        info!("Skipping compiling step");
    } else {
        info!("Compiling {:?}", all_packages);
        compile_wasm_packages(&all_packages)?;
    }

    let wasm_files_location = target
        .clone()
        .into_std_path_buf()
        .join("wasm32-wasi")
        .join("debug");

    let packages = sort_packages(&package, all_packages);

    let wasm_files = convert_package_to_location(&wasm_files_location, packages)?;

    let current_composite =
        combine_wasm_files(&wasm_files_location.join(&package.name), wasm_files)?;

    let final_composite = copy_final_composite_to_composed_wasm(
        &package,
        wasm_files_location.join("pulumi"),
        current_composite,
    )?;

    info!("Final composite: {:?}", final_composite);

    Ok(())
}

fn convert_package_to_location(
    wasm_files_location: &Path,
    packages: Vec<String>,
) -> Result<Vec<PathBuf>, Error> {
    packages
        .iter()
        .map(|file_name| {
            let wasm_file = wasm_files_location.join(format!("{}.wasm", &file_name));
            if !wasm_file.exists() {
                Err(anyhow!(
                    "Cannot find WASM file for package {}. Tried {:?}",
                    &file_name,
                    &wasm_file
                ))
            } else {
                Ok(replace_underscores_with_dashes(&wasm_file))
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

fn sort_packages(package: &Package, all_packages: HashSet<String>) -> Vec<String> {
    #[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
    enum PackageType {
        Main(String),
        Provider(String),
        Other(String),
    }

    impl PackageType {
        fn get_name(&self) -> &str {
            match self {
                PackageType::Main(name) => name,
                PackageType::Provider(name) => name,
                PackageType::Other(name) => name,
            }
        }
    }

    all_packages
        .iter()
        .map(|p| {
            if p.contains("provider") {
                PackageType::Provider(p.clone())
            } else if p == &package.name {
                PackageType::Main(p.clone())
            } else {
                PackageType::Other(p.clone())
            }
        })
        .sorted()
        .map(|p| p.get_name().to_string())
        .collect()
}

fn combine_wasm_files(
    wasm_files_location: &Path,
    packages: Vec<PathBuf>,
) -> Result<PathBuf, Error> {
    let mut packages = VecDeque::from(packages);
    std::fs::create_dir_all(wasm_files_location)?;

    info!("Packages: {:?}", packages);

    let wasm_file = match packages.pop_front() {
        Some(location) => location,
        a => panic!("Expected main package, got {:?}", a),
    };

    let mut current_composite = wasm_file;
    let mut i = 0;

    while let Some(package) = packages.pop_front() {
        info!("Merging package: {:?}", package.file_name().unwrap());
        let composite = wasm_files_location.join(format!("composed{}.wasm", i));
        info!(
            "Composing {:?} and {:?} into {:?}",
            &current_composite, &package, &composite
        );
        let output = Command::new("wasm-tools")
            .args([
                "compose",
                "-o",
                &composite.to_str().unwrap(),
                current_composite.to_str().unwrap(),
                "-d",
                package.to_str().unwrap(),
            ])
            .output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to compose {:?} and {:?}: {}",
                &current_composite,
                &package,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        current_composite = composite;
        i += 1;
    }
    Ok(current_composite)
}

fn copy_final_composite_to_composed_wasm(
    package: &Package,
    wasm_files_location: PathBuf,
    current_composite: PathBuf,
) -> Result<PathBuf, Error> {
    std::fs::create_dir_all(&wasm_files_location)?;
    let final_composite = wasm_files_location.join(format!("{}.wasm", package.name));

    std::fs::copy(current_composite, &final_composite)?;
    Ok(final_composite)
}

fn compile_wasm_packages(all_packages: &HashSet<String>) -> Result<(), Error> {
    let binding = all_packages
        .iter()
        .map(|d| vec!["-p", d])
        .collect::<Vec<_>>();
    let flags = binding.iter().flatten().collect::<Vec<_>>();

    let output = Command::new("cargo")
        .args(["component", "build"])
        .args(&flags)
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to build {:?}: {}",
            all_packages,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn replace_underscores_with_dashes(file: &Path) -> PathBuf {
    if let Some(file_name) = file.file_name() {
        if file_name.to_str().unwrap().contains('_') {
            let new_file_name = file_name.to_str().unwrap().replace('_', "-");
            let new_file = file.with_file_name(new_file_name);
            println!("{:?} -> {:?}", file, new_file);
            std::fs::copy(file, &new_file).unwrap();
            new_file
        } else {
            file.to_path_buf()
        }
    } else {
        file.to_path_buf()
    }
}
