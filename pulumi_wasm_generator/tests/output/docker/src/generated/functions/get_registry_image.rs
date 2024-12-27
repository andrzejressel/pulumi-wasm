pub mod get_registry_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        #[builder(into, default)]
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Docker image, including any tags. e.g. `alpine:latest`
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRegistryImageArgs) -> GetRegistryImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "insecureSkipVerify".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sha256Digest".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegistryImageResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            insecure_skip_verify: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insecureSkipVerify").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sha256_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sha256Digest").unwrap(),
            ),
        }
    }
}
