#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ResourceArgs {
    #[builder(into, default)]
    pub baz: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ResourceResult {
    pub baz: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ResourceArgs) -> ResourceResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let baz_binding = args.baz.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "foo-bar:deeply/nested/module:Resource".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "baz".into(),
                value: &baz_binding,
            },
        ]),
        results: vec![register_interface::ResultField { name : "baz".into() },],
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
    ResourceResult {
        baz: into_domain(hashmap.remove("baz").unwrap()),
    }
}
