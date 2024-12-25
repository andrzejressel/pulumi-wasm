#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRegistryImageArgs {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    #[builder(into, default)]
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image, including any tags. e.g. `alpine:latest`
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetRegistryImageResult {
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image, including any tags. e.g. `alpine:latest`
    pub name: pulumi_wasm_rust::Output<String>,
    /// The content digest of the image, as stored in the registry.
    pub sha256_digest: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetRegistryImageArgs) -> GetRegistryImageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let insecure_skip_verify_binding = args.insecure_skip_verify.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "docker:index/getRegistryImage:getRegistryImage".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "insecureSkipVerify".into(),
                value: &insecure_skip_verify_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "insecureSkipVerify".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "sha256Digest".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetRegistryImageResult {
        id: into_domain(hashmap.remove("id").unwrap()),
        insecure_skip_verify: into_domain(hashmap.remove("insecureSkipVerify").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        sha256_digest: into_domain(hashmap.remove("sha256Digest").unwrap()),
    }
}
