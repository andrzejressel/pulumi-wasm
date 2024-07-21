use anyhow::Context;
use anyhow::Result;
use directories::BaseDirs;
use futures::{StreamExt, TryFutureExt};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use wac_graph::types::SubtypeChecker;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions, NodeId, PackageId};

pub(crate) async fn create(
    providers_paths: &HashMap<String, &PathBuf>,
    pulumi_wasm: &Option<PathBuf>,
    program: &PathBuf,
) -> Result<Vec<u8>> {
    let mut graph = CompositionGraph::new();

    let provider_name_regex: Regex = Regex::new(r"pulumi:(.*)/.*@[^-]*-(.*)")?;
    let pulumi_wasm_version_regex: Regex =
        Regex::new(r"component:pulumi-wasm/output-interface@(.*)")?;

    // Register the package dependencies into the graph
    let main = Package::from_file("main", None, program, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();
    let main_instance = graph.instantiate(main_package_id);

    let import_names = graph.types()[graph[main_package_id].ty()]
        .imports
        .iter()
        .map(|(name, _)| name);

    let provider_names: HashSet<String> = import_names
        .clone()
        .filter_map(|import| provider_name_regex.captures(import))
        .map(|captures| captures[1].to_string())
        .collect();

    let pulumi_wasm_version: String = import_names
        .filter_map(|import| pulumi_wasm_version_regex.captures(import))
        .map(|captures| captures[1].to_string())
        .next()
        .unwrap_or_else(|| panic!("No Pulumi-Wasm version found"));

    let mut provider_wasm_files = HashMap::new();

    for provider_name in &provider_names {
        match providers_paths.get(provider_name) {
            None => {
                let downloaded =
                    download_provider(provider_name.as_str(), pulumi_wasm_version.as_str())
                        .await
                        .context(format!("Failed to download provider [{}]", provider_name))?;
                provider_wasm_files.insert(provider_name.clone(), downloaded);
            }
            Some(location) => {
                provider_wasm_files.insert(provider_name.clone(), location.to_path_buf());
            }
        }
    }

    let pulumi_wasm = match pulumi_wasm {
        None => download_pulumi_wasm(&pulumi_wasm_version)
            .await
            .context("Failed to download Pulumi-Wasm")?,
        Some(pw) => pw.to_path_buf(),
    };

    let pulumi_wasm =
        Package::from_file("pulumi_wasm", None, pulumi_wasm, graph.types_mut()).unwrap();
    let pulumi_wasm_package_id = graph.register_package(pulumi_wasm.clone()).unwrap();
    let pulumi_wasm_instance = graph.instantiate(pulumi_wasm_package_id);

    for (i, (_, provider)) in provider_wasm_files.iter().enumerate() {
        let provider = Package::from_file(
            format!("provider-{}", i).as_str(),
            None,
            provider,
            graph.types_mut(),
        )
        .unwrap();
        let provider_package_id = graph.register_package(provider.clone()).unwrap();
        let provider_instance = graph.instantiate(provider_package_id);

        plug_into_socket(
            main_package_id,
            main_instance,
            provider_package_id,
            provider_instance,
            &mut graph,
        )
        .unwrap();

        plug_into_socket(
            provider_package_id,
            provider_instance,
            pulumi_wasm_package_id,
            pulumi_wasm_instance,
            &mut graph,
        )
        .unwrap();
    }

    plug_into_socket(
        main_package_id,
        main_instance,
        pulumi_wasm_package_id,
        pulumi_wasm_instance,
        &mut graph,
    )
    .unwrap();

    let pulumi_main_component_name = format!(
        "component:pulumi-wasm/pulumi-main@{}",
        env!("CARGO_PKG_VERSION")
    );
    let pulumi_main_export = graph
        .alias_instance_export(main_instance, pulumi_main_component_name.as_str())
        .unwrap();
    graph
        .export(pulumi_main_export, pulumi_main_component_name.as_str())
        .unwrap();

    Ok(graph.encode(EncodeOptions::default()).unwrap())
}

/// https://github.com/bytecodealliance/wac/blob/290c10068a080b33a49cb8d0b4f83601840cec51/src/commands/plug.rs#L282-L316
/// Take the exports of the plug component and plug them into the socket component.
fn plug_into_socket(
    socket: PackageId,
    socket_instantiation: NodeId,
    plug: PackageId,
    plug_instantiation: NodeId,
    graph: &mut CompositionGraph,
) -> Result<(), anyhow::Error> {
    let mut plugs = Vec::new();
    let mut cache = Default::default();
    let mut checker = SubtypeChecker::new(&mut cache);
    for (name, plug_ty) in &graph.types()[graph[plug].ty()].exports {
        if let Some(socket_ty) = graph.types()[graph[socket].ty()].imports.get(name) {
            if checker
                .is_subtype(*plug_ty, graph.types(), *socket_ty, graph.types())
                .is_ok()
            {
                plugs.push(name.clone());
            }
        }
    }

    // Instantiate the plug component
    for plug_name in plugs {
        log::debug!("using export `{plug_name}` for plug");
        let export = graph.alias_instance_export(plug_instantiation, &plug_name)?;
        graph.set_instantiation_argument(socket_instantiation, &plug_name, export)?;
    }
    Ok(())
}

async fn download_provider(provider_name: &str, pulumi_wasm_version: &str) -> Result<PathBuf> {
    let wasm_location = BaseDirs::new()
        .context("Unable to get user directories")?
        .cache_dir()
        .join("pulumi-wasm")
        .join("providers")
        .join(format!("{}-{}.wasm", provider_name, pulumi_wasm_version));

    if !wasm_location.exists() {
        std::fs::create_dir_all(wasm_location.parent().unwrap())?;

        let bytes = reqwest::get(format!("https://github.com/andrzejressel/pulumi-wasm/releases/download/v{}/pulumi_wasm_{}_provider.wasm", pulumi_wasm_version, provider_name))
            .await
            .context("Failed to download provider")?
            .bytes()
            .await
            .context("Failed to download provider")?;

        tokio::fs::write(&wasm_location, bytes)
            .await
            .context("Failed to write provider file")?;
    }
    Ok(wasm_location)
}

async fn download_pulumi_wasm(pulumi_wasm_version: &str) -> Result<PathBuf> {
    let wasm_location = BaseDirs::new()
        .context("Unable to get user directories")?
        .cache_dir()
        .join("pulumi-wasm")
        .join(format!("pulumi-wasm-{}.wasm", pulumi_wasm_version));

    if !wasm_location.exists() {
        std::fs::create_dir_all(wasm_location.parent().unwrap())?;

        let bytes = reqwest::get(format!(
            "https://github.com/andrzejressel/pulumi-wasm/releases/download/v{}/pulumi_wasm.wasm",
            pulumi_wasm_version
        ))
        .await
        .context("Failed to download pulumi_wasm")?
        .bytes()
        .await
        .context("Failed to download pulumi_wasm")?;

        tokio::fs::write(&wasm_location, bytes)
            .await
            .context("Failed to write pulumi_wasm file")?;
    }
    Ok(wasm_location)
}
