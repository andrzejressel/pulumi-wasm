pub mod get_backend_bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendBucketArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetBackendBucketArgs) -> GetBackendBucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getBackendBucket:getBackendBucket".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucketName".into(),
                },
                register_interface::ResultField {
                    name: "cdnPolicies".into(),
                },
                register_interface::ResultField {
                    name: "compressionMode".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "customResponseHeaders".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "edgeSecurityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "enableCdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBackendBucketResult {
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketName").unwrap(),
            ),
            cdn_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnPolicies").unwrap(),
            ),
            compression_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compressionMode").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            custom_response_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customResponseHeaders").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edge_security_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeSecurityPolicy").unwrap(),
            ),
            enable_cdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableCdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
