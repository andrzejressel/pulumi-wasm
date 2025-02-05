pub mod get_registry_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        #[builder(into, default)]
        pub insecure_skip_verify: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Docker image, including any tags. e.g. `alpine:latest`
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegistryImageArgs,
    ) -> GetRegistryImageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let insecure_skip_verify_binding = args
            .insecure_skip_verify
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "docker:index/getRegistryImage:getRegistryImage".into(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegistryImageResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            insecure_skip_verify: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("insecureSkipVerify"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            sha256_digest: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sha256Digest"),
            ),
        }
    }
}
