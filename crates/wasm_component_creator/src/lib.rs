use crate::source::WasmComponentSource;
use anyhow::{bail, Context};
use itertools::Itertools;
use log::info;
use regex::Regex;
use std::collections::BTreeSet;
use std::hash::Hash;
use wac_graph::types::{Package, SubtypeChecker};
use wac_graph::{CompositionGraph, EncodeOptions, NodeId, PackageId};

pub mod source;

const PROVIDER_REGEX: &str = r"pulumi:(.*)/.*@(.*)--(.*)";

pub async fn create(
    pulumi_gestalt: &dyn WasmComponentSource,
    program: Vec<u8>,
    debug: bool,
) -> anyhow::Result<Vec<u8>> {
    let mut graph = CompositionGraph::new();

    let pulumi_gestalt_version_regex: Regex = Regex::new(r"component:pulumi-gestalt/.*@(.*)")?;

    // Register the package dependencies into the graph
    let main = Package::from_bytes("main", None, program, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main).unwrap();
    let main_instance = graph.instantiate(main_package_id);

    let import_names = graph.types()[graph[main_package_id].ty()]
        .imports
        .iter()
        .map(|(name, _)| name);

    let pulumi_gestalt_versions: BTreeSet<String> = import_names
        .filter_map(|import| pulumi_gestalt_version_regex.captures(import))
        .map(|captures| captures[1].to_string())
        .collect();

    if pulumi_gestalt_versions.is_empty() {
        bail!("No Pulumi-Wasm version found");
    } else if pulumi_gestalt_versions.len() > 1 {
        bail!(
            "Found multiple Pulumi-Wasm versions: {}. Ensure only one is used.",
            pulumi_gestalt_versions.into_iter().sorted().join(", ")
        );
    }

    let pulumi_gestalt_version = pulumi_gestalt_versions.iter().next().unwrap();

    info!("Pulumi Wasm version: {pulumi_gestalt_version}");

    let pulumi_gestalt = pulumi_gestalt
        .get(pulumi_gestalt_version, debug)
        .await
        .context("Cannot obtain pulumi_gestalt component Wasm")?;

    let pulumi_gestalt =
        Package::from_bytes("pulumi_gestalt", None, pulumi_gestalt, graph.types_mut()).unwrap();
    let pulumi_gestalt_package_id = graph.register_package(pulumi_gestalt.clone()).unwrap();
    let pulumi_gestalt_instance = graph.instantiate(pulumi_gestalt_package_id);

    plug_into_socket(
        main_package_id,
        main_instance,
        pulumi_gestalt_package_id,
        pulumi_gestalt_instance,
        &mut graph,
    )
    .unwrap();

    let pulumi_main_component_name =
        "component:pulumi-gestalt-external/pulumi-main@0.0.0-STABLE-DEV";
    let pulumi_main_export = graph
        .alias_instance_export(main_instance, pulumi_main_component_name)
        .unwrap();
    graph
        .export(pulumi_main_export, pulumi_main_component_name)
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

fn extract_provider_info(import_name: impl AsRef<str>) -> Option<ProviderInfo> {
    let regex = Regex::new(PROVIDER_REGEX).unwrap();
    regex
        .captures(import_name.as_ref())
        .map(|captures| ProviderInfo {
            name: captures.get(1).unwrap().as_str().to_string(),
            version: captures.get(2).unwrap().as_str().to_string(),
            pulumi_gestalt_version: captures.get(3).unwrap().as_str().to_string(),
        })
}

#[derive(Eq, Debug, PartialOrd, PartialEq, Hash, Ord)]
struct ProviderInfo {
    name: String,
    version: String,
    pulumi_gestalt_version: String,
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
                    pulumi_gestalt_version: "0.0.0-DEV".to_string(),
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
