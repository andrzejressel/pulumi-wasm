use std::ops::Index;
use std::path::PathBuf;
use wac_graph::types::SubtypeChecker;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions, NodeId, PackageId};

pub(crate) fn create(
    providers: &Vec<PathBuf>,
    pulumi_wasm: &PathBuf,
    program: &PathBuf,
) -> Vec<u8> {
    let mut graph = CompositionGraph::new();

    // Register the package dependencies into the graph
    let main = Package::from_file("main", None, program, graph.types_mut()).unwrap();
    let main_package_id = graph.register_package(main.clone()).unwrap();
    let main_instance = graph.instantiate(main_package_id);

    // let docker_provider = Package::from_file(
    //     "docker_provider",
    //     None,
    //     "D:\\MojeProgramy\\pulumi-wasm\\target\\wasm32-wasi\\debug\\pulumi-wasm-docker-provider.wasm",
    //     graph.types_mut(),
    // )
    //     .unwrap();
    // let docker_provider_package_id = graph.register_package(docker_provider.clone()).unwrap();
    // let docker_provider_instance = graph.instantiate(docker_provider_package_id);
    //
    // let random_provider = Package::from_file(
    //     "random_provider",
    //     None,
    //     "D:\\MojeProgramy\\pulumi-wasm\\target\\wasm32-wasi\\debug\\pulumi-wasm-random-provider.wasm",
    //     graph.types_mut(),
    // )
    //     .unwrap();
    // let random_provider_package_id = graph.register_package(random_provider.clone()).unwrap();
    // let random_provider_instance = graph.instantiate(random_provider_package_id);

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

    // plug_into_socket(
    //     main_package_id,
    //     main_instance,
    //     docker_provider_package_id,
    //     docker_provider_instance,
    //     &mut graph,
    // ).unwrap();
    //
    // plug_into_socket(
    //     main_package_id,
    //     main_instance,
    //     random_provider_package_id,
    //     random_provider_instance,
    //     &mut graph,
    // ).unwrap();
    //
    //
    // plug_into_socket(
    //     random_provider_package_id,
    //     random_provider_instance,
    //     pulumi_wasm_package_id,
    //     pulumi_wasm_instance,
    //     &mut graph,
    // ).unwrap();
    //
    // plug_into_socket(
    //     docker_provider_package_id,
    //     docker_provider_instance,
    //     pulumi_wasm_package_id,
    //     pulumi_wasm_instance,
    //     &mut graph,
    // ).unwrap();

    plug_into_socket(
        main_package_id,
        main_instance,
        pulumi_wasm_package_id,
        pulumi_wasm_instance,
        &mut graph,
    )
    .unwrap();

    let greet_export = graph
        .alias_instance_export(main_instance, "component:pulumi-wasm/pulumi-main@0.1.0")
        .unwrap();
    graph
        .export(greet_export, "component:pulumi-wasm/pulumi-main@0.1.0")
        .unwrap();

    let encoding = graph.encode(EncodeOptions::default()).unwrap();
    encoding
    // std::fs::write("composition.wasm", encoding).unwrap();
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