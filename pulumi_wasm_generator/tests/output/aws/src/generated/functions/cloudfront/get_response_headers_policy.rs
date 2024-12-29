pub mod get_response_headers_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResponseHeadersPolicyArgs {
        /// Identifier for the response headers policy.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name to identify the response headers policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResponseHeadersPolicyResult {
        /// Comment to describe the response headers policy. The comment cannot be longer than 128 characters.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// Configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
        pub cors_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfig,
            >,
        >,
        /// Object that contains an attribute `items` that contains a list of Custom Headers. See Custom Header for more information.
        pub custom_headers_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyCustomHeadersConfig,
            >,
        >,
        /// Current version of the response headers policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Object that contains an attribute `items` that contains a list of Remove Headers. See Remove Header for more information.
        pub remove_headers_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyRemoveHeadersConfig,
            >,
        >,
        /// A configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
        pub security_headers_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfig,
            >,
        >,
        /// (Optional) Configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
        pub server_timing_headers_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyServerTimingHeadersConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetResponseHeadersPolicyArgs) -> GetResponseHeadersPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getResponseHeadersPolicy:getResponseHeadersPolicy"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "corsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "customHeadersConfigs".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "removeHeadersConfigs".into(),
                },
                register_interface::ResultField {
                    name: "securityHeadersConfigs".into(),
                },
                register_interface::ResultField {
                    name: "serverTimingHeadersConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResponseHeadersPolicyResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            cors_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsConfigs").unwrap(),
            ),
            custom_headers_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customHeadersConfigs").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            remove_headers_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("removeHeadersConfigs").unwrap(),
            ),
            security_headers_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityHeadersConfigs").unwrap(),
            ),
            server_timing_headers_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverTimingHeadersConfigs").unwrap(),
            ),
        }
    }
}
