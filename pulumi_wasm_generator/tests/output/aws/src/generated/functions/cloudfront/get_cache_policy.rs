pub mod get_cache_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCachePolicyArgs {
        /// Identifier for the cache policy.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name to identify the cache policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCachePolicyResult {
        /// Comment to describe the cache policy.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// Default amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub default_ttl: pulumi_wasm_rust::Output<i32>,
        /// Current version of the cache policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub max_ttl: pulumi_wasm_rust::Output<i32>,
        /// Minimum amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub min_ttl: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The HTTP headers, cookies, and URL query strings to include in the cache key. See Parameters In Cache Key And Forwarded To Origin for more information.
        pub parameters_in_cache_key_and_forwarded_to_origins: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOrigin,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCachePolicyArgs) -> GetCachePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getCachePolicy:getCachePolicy".into(),
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
                    name: "defaultTtl".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "maxTtl".into(),
                },
                register_interface::ResultField {
                    name: "minTtl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parametersInCacheKeyAndForwardedToOrigins".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCachePolicyResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            default_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultTtl").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            max_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxTtl").unwrap(),
            ),
            min_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minTtl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters_in_cache_key_and_forwarded_to_origins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parametersInCacheKeyAndForwardedToOrigins").unwrap(),
            ),
        }
    }
}
