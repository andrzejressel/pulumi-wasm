#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct R2BucketArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket. Available values: `WNAM`, `ENAM`, `WEUR`, `EEUR`, `APAC`
    #[builder(into, default)]
    pub location: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the R2 bucket.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct R2BucketResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket. Available values: `WNAM`, `ENAM`, `WEUR`, `EEUR`, `APAC`
    pub location: pulumi_wasm_rust::Output<String>,
    /// The name of the R2 bucket.
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: R2BucketArgs) -> R2BucketResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let location_binding = args.location.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/r2Bucket:R2Bucket".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "location".into(),
                value: &location_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "location".into() },
            register_interface::ResultField { name : "name".into() },
        ],
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
    R2BucketResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        location: into_domain(hashmap.remove("location").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
