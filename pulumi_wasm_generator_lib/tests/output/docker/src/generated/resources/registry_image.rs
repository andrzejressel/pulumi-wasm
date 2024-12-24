#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RegistryImageArgs {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    #[builder(into, default)]
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
    #[builder(into, default)]
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
    #[builder(into, default)]
    pub triggers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
}
pub struct RegistryImageResult {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The sha256 digest of the image.
    pub sha256_digest: pulumi_wasm_rust::Output<String>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
    pub triggers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegistryImageArgs) -> RegistryImageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let insecure_skip_verify_binding = args.insecure_skip_verify.get_inner();
    let keep_remotely_binding = args.keep_remotely.get_inner();
    let name_binding = args.name.get_inner();
    let triggers_binding = args.triggers.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/registryImage:RegistryImage".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "insecureSkipVerify".into(),
                value: &insecure_skip_verify_binding,
            },
            register_interface::ObjectField {
                name: "keepRemotely".into(),
                value: &keep_remotely_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "triggers".into(),
                value: &triggers_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "insecureSkipVerify".into() },
            register_interface::ResultField { name : "keepRemotely".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "sha256Digest".into() },
            register_interface::ResultField { name : "triggers".into() },
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
    RegistryImageResult {
        insecure_skip_verify: into_domain(hashmap.remove("insecureSkipVerify").unwrap()),
        keep_remotely: into_domain(hashmap.remove("keepRemotely").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        sha256_digest: into_domain(hashmap.remove("sha256Digest").unwrap()),
        triggers: into_domain(hashmap.remove("triggers").unwrap()),
    }
}
