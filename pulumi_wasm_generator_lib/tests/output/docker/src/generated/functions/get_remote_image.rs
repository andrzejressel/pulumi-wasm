#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRemoteImageArgs {
    /// The name of the Docker image, including any tags or SHA256 repo digests.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetRemoteImageResult {
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The name of the Docker image, including any tags or SHA256 repo digests.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`. It may be empty in the edge case where the local image was pulled from a repo, tagged locally, and then referred to in the data source by that local name/tag.
    pub repo_digest: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetRemoteImageArgs) -> GetRemoteImageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "docker:index/getRemoteImage:getRemoteImage".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "repoDigest".into() },
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
    GetRemoteImageResult {
        id: into_domain(hashmap.remove("id").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        repo_digest: into_domain(hashmap.remove("repoDigest").unwrap()),
    }
}
