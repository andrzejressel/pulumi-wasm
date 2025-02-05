pub mod get_backend_bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendBucketArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackendBucketResult {
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        pub cdn_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendBucketCdnPolicy>,
        >,
        pub compression_mode: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub custom_response_headers: pulumi_wasm_rust::Output<Vec<String>>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub edge_security_policy: pulumi_wasm_rust::Output<String>,
        pub enable_cdn: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBackendBucketArgs,
    ) -> GetBackendBucketResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getBackendBucket:getBackendBucket".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBackendBucketResult {
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketName"),
            ),
            cdn_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdnPolicies"),
            ),
            compression_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compressionMode"),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            custom_response_headers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customResponseHeaders"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            edge_security_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("edgeSecurityPolicy"),
            ),
            enable_cdn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableCdn"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
