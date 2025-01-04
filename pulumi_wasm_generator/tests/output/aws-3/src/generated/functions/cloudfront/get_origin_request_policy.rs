pub mod get_origin_request_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginRequestPolicyArgs {
        /// Identifier for the origin request policy.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name to identify the origin request policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOriginRequestPolicyResult {
        /// Comment to describe the origin request policy.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        pub cookies_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyCookiesConfig,
            >,
        >,
        /// Current version of the origin request policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        pub headers_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyHeadersConfig,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        pub query_strings_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyQueryStringsConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOriginRequestPolicyArgs) -> GetOriginRequestPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getOriginRequestPolicy:getOriginRequestPolicy".into(),
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
                    name: "cookiesConfigs".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "headersConfigs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queryStringsConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginRequestPolicyResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            cookies_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cookiesConfigs").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            headers_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("headersConfigs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_strings_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryStringsConfigs").unwrap(),
            ),
        }
    }
}
