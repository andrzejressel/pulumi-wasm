use crate::source::{DefaultProviderSource, ProviderSource, PulumiWasmSource};
use anyhow::{bail, Context};
use itertools::Itertools;
use log::info;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;
use wac_graph::types::{Package, SubtypeChecker};
use wac_graph::{CompositionGraph, EncodeOptions, NodeId, PackageId};

pub mod source;

const PROVIDER_REGEX: &str = r"pulumi:(.*)/.*@(.*)--(.*)";

pub async fn create(
    providers_paths: BTreeMap<String, Box<dyn ProviderSource>>,
    default_provider_source: &dyn DefaultProviderSource,
    pulumi_wasm: &dyn PulumiWasmSource,
    program: Vec<u8>,
    debug: bool,
) -> anyhow::Result<Vec<u8>> {
    let mut graph = CompositionGraph::new();

    let pulumi_wasm_version_regex: Regex = Regex::new(r"component:pulumi-wasm/.*@(.*)")?;

    // Register the package dependencies into the graph
    let main = Package::from_bytes("main", None, program, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();
    let main_instance = graph.instantiate(main_package_id);

    let import_names = graph.types()[graph[main_package_id].ty()]
        .imports
        .iter()
        .map(|(name, _)| name);

    let all_providers: BTreeMap<_, BTreeSet<_>> = import_names
        .clone()
        .filter_map(extract_provider_info)
        .chunk_by(|pi| pi.name.clone())
        .into_iter()
        .map(|(name, providers)| (name, providers.collect()))
        .collect();

    check_for_multiple_instances_of_same_provider(&all_providers)?;

    // After check_for_multiple_instances_of_same_provider we are sure each set has only one element
    let all_providers: BTreeMap<_, _> = all_providers
        .iter()
        .map(|(key, value)| (key.clone(), value.iter().next().unwrap()))
        .collect();

    let pulumi_wasm_versions: BTreeSet<String> = import_names
        .filter_map(|import| pulumi_wasm_version_regex.captures(import))
        .map(|captures| captures[1].to_string())
        .collect();

    if pulumi_wasm_versions.is_empty() {
        bail!("No Pulumi-Wasm version found");
    } else if pulumi_wasm_versions.len() > 1 {
        bail!(
            "Found multiple Pulumi-Wasm versions: {}. Ensure only one is used.",
            pulumi_wasm_versions.into_iter().sorted().join(", ")
        );
    }

    let pulumi_wasm_version = pulumi_wasm_versions.iter().next().unwrap();

    info!("Pulumi WASM version: {pulumi_wasm_version}");

    // Is not getting invoked, because different version are capture before
    // (because of transitive output dependency)
    //
    // {
    //     let providers_per_pulumi_wasm_version: BTreeMap<_, BTreeSet<_>> = all_providers
    //         .iter()
    //         .map(|(_, pi)| { pi })
    //         .chunk_by(|pi| pi.pulumi_wasm_version.clone())
    //         .into_iter()
    //         .map(|(name, providers)| (name, providers.collect()))
    //         .collect();
    //
    //     if providers_per_pulumi_wasm_version.len() > 1 {
    //
    //         let mut message = "References to multiple pulumi-wasm versions detected\n:".to_string();
    //
    //         for (version, providers) in providers_per_pulumi_wasm_version {
    //             message.push_str(format!("- Pulumi WASM {version}:\n").as_str());
    //
    //             for (provider) in providers {
    //                 message.push_str(format!("  - {}@{}\n", provider.name, provider.version).as_str());
    //             }
    //         }
    //
    //         bail!("{}", message);
    //
    //         // bail!("Found multiple providers with the same Pulumi-Wasm version: {}. Ensure only one is used.", providers_per_pulumi_wasm_version.keys().sorted().join(", "));
    //     }
    //
    // }

    let mut provider_wasm_files = BTreeMap::new();

    for provider in all_providers.values() {
        let provider_name = &provider.name;
        let downloaded = match providers_paths.get(provider_name) {
            None => {
                default_provider_source
                    .get_component(provider_name, &provider.version, pulumi_wasm_version, debug)
                    .await
            }
            Some(provider_source) => provider_source.get_component(pulumi_wasm_version).await,
        }
        .context(format!("Cannot obtain provider {}", provider_name))?;
        provider_wasm_files.insert(provider_name.clone(), downloaded);
    }

    let pulumi_wasm = pulumi_wasm
        .get(pulumi_wasm_version, debug)
        .await
        .context("Cannot obtain pulumi_wasm component WASM")?;

    let pulumi_wasm =
        Package::from_bytes("pulumi_wasm", None, pulumi_wasm, graph.types_mut()).unwrap();
    let pulumi_wasm_package_id = graph.register_package(pulumi_wasm.clone()).unwrap();
    let pulumi_wasm_instance = graph.instantiate(pulumi_wasm_package_id);

    for (i, (_, provider)) in provider_wasm_files.into_iter().enumerate() {
        let provider = Package::from_bytes(
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

    let pulumi_main_component_name = "component:pulumi-wasm-external/pulumi-main@0.0.0-STABLE-DEV";
    let pulumi_main_export = graph
        .alias_instance_export(main_instance, pulumi_main_component_name)
        .unwrap();
    graph
        .export(pulumi_main_export, pulumi_main_component_name)
        .unwrap();

    Ok(graph.encode(EncodeOptions::default()).unwrap())
}

fn check_for_multiple_instances_of_same_provider(
    all_providers: &BTreeMap<String, BTreeSet<ProviderInfo>>,
) -> anyhow::Result<()> {
    for (provider_name, provider_infos) in all_providers {
        if provider_infos.len() > 1 {
            let mut message =
                format!("Provider \"{provider_name}\" is requested in multiple versions:\n");
            for pi in provider_infos {
                message.push_str(
                    format!(
                        "- {} that requires pulumi_wasm in version {}\n",
                        pi.version, pi.pulumi_wasm_version
                    )
                    .as_str(),
                )
            }
            let message = message.trim().to_string();
            bail!(message)
        }
    }
    Ok(())
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

fn extract_provider_info(import_name: impl AsRef<str>) -> Option<ProviderInfo> {
    let regex = Regex::new(PROVIDER_REGEX).unwrap();
    regex
        .captures(import_name.as_ref())
        .map(|captures| ProviderInfo {
            name: captures.get(1).unwrap().as_str().to_string(),
            version: captures.get(2).unwrap().as_str().to_string(),
            pulumi_wasm_version: captures.get(3).unwrap().as_str().to_string(),
        })
}

#[derive(Eq, Debug, PartialOrd, PartialEq, Hash, Ord)]
struct ProviderInfo {
    name: String,
    version: String,
    pulumi_wasm_version: String,
}

#[cfg(test)]
mod tests {

    use anyhow::*;

    mod provider_regex {
        use super::*;
        use crate::{extract_provider_info, ProviderInfo};

        #[test]
        fn provider_regex_should_work() -> Result<()> {
            assert_eq!(
                extract_provider_info("pulumi:docker/container@4.5.3--0.0.0-DEV"),
                Some(ProviderInfo {
                    name: "docker".to_string(),
                    version: "4.5.3".to_string(),
                    pulumi_wasm_version: "0.0.0-DEV".to_string(),
                })
            );

            Ok(())
        }

        #[test]
        fn extract_provider_info_return_none_if_does_not_match() -> Result<()> {
            assert_eq!(extract_provider_info("test"), None);

            Ok(())
        }
    }
}
