use std::path::PathBuf;
use wac_graph::types::SubtypeChecker;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions, NodeId, PackageId};

pub(crate) fn create(providers: &[PathBuf], pulumi_wasm: &PathBuf, program: &PathBuf) -> Vec<u8> {
    let mut graph = CompositionGraph::new();

    // Register the package dependencies into the graph
    let main = Package::from_file("main", None, program, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();
    let main_instance = graph.instantiate(main_package_id);

    let pulumi_wasm =
        Package::from_file("pulumi_wasm", None, pulumi_wasm, graph.types_mut()).unwrap();
    let pulumi_wasm_package_id = graph.register_package(pulumi_wasm.clone()).unwrap();
    let pulumi_wasm_instance = graph.instantiate(pulumi_wasm_package_id);

    for (i, provider) in providers.iter().enumerate() {
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

    graph.encode(EncodeOptions::default()).unwrap()
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
