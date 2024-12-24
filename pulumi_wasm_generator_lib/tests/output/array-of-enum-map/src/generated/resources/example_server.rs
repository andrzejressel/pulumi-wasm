#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ExampleServerArgs {
    #[builder(into, default)]
    pub map_array_enum: pulumi_wasm_rust::Output<
        Option<
            Vec<
                std::collections::HashMap<
                    String,
                    super::types::AnnotationStoreSchemaValueType,
                >,
            >,
        >,
    >,
}
pub struct ExampleServerResult {
    pub map_array_enum: pulumi_wasm_rust::Output<
        Option<
            Vec<
                std::collections::HashMap<
                    String,
                    super::types::AnnotationStoreSchemaValueType,
                >,
            >,
        >,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let map_array_enum_binding = args.map_array_enum.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "example:index:ExampleServer".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "mapArrayEnum".into(),
                value: &map_array_enum_binding,
            },
        ]),
        results: vec![register_interface::ResultField { name : "mapArrayEnum".into() },],
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
        map_array_enum: into_domain(hashmap.remove("mapArrayEnum").unwrap()),
    }
}
