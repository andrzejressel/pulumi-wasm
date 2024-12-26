#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default)]
    pub properties_collection: pulumi_wasm_rust::Output<
        Option<
            Vec<
                pulumi_wasm_provider_common::OneOf2<
                    super::types::ServerPropertiesForReplica,
                    super::types::ServerPropertiesForRestore,
                >,
            >,
        >,
    >,
}
pub struct ExampleServerResult {
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let properties_collection_binding = args.properties_collection.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "example:index:ExampleServer".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "propertiesCollection".into(),
                value: &properties_collection_binding,
            },
        ]),
        results: vec![register_interface::ResultField { name : "name".into() },],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ExampleServerResult {
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
