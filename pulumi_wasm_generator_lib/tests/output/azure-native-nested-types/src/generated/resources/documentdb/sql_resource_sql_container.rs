pub struct SqlResourceSqlContainerResult {
    pub resource: pulumi_wasm_rust::Output<
        Option<
            super::super::types::documentdb::SqlContainerGetPropertiesResponseResource,
        >,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str) -> SqlResourceSqlContainerResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let request = register_interface::RegisterResourceRequest {
        type_: "azure-native:documentdb:SqlResourceSqlContainer".into(),
        name: name.to_string(),
        object: Vec::from([]),
        results: vec![register_interface::ResultField { name : "resource".into() },],
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
    SqlResourceSqlContainerResult {
        resource: into_domain(hashmap.remove("resource").unwrap()),
    }
}
