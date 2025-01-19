/// Provides an AppSync API Cache.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = graph_ql_api::create(
///         "example",
///         GraphQlApiArgs::builder()
///             .authentication_type("API_KEY")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleApiCache = api_cache::create(
///         "exampleApiCache",
///         ApiCacheArgs::builder()
///             .api_caching_behavior("FULL_REQUEST_CACHING")
///             .api_id("${example.id}")
///             .ttl(900)
///             .type_("LARGE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_api_cache` using the AppSync API ID. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/apiCache:ApiCache example xxxxx
/// ```
pub mod api_cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiCacheArgs {
        /// Caching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.
        #[builder(into)]
        pub api_caching_behavior: pulumi_wasm_rust::Output<String>,
        /// GraphQL API ID.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// At-rest encryption flag for cache. You cannot update this setting after creation.
        #[builder(into, default)]
        pub at_rest_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Transit encryption flag when connecting to cache. You cannot update this setting after creation.
        #[builder(into, default)]
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// TTL in seconds for cache entries.
        #[builder(into)]
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiCacheResult {
        /// Caching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.
        pub api_caching_behavior: pulumi_wasm_rust::Output<String>,
        /// GraphQL API ID.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// At-rest encryption flag for cache. You cannot update this setting after creation.
        pub at_rest_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Transit encryption flag when connecting to cache. You cannot update this setting after creation.
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// TTL in seconds for cache entries.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiCacheArgs) -> ApiCacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_caching_behavior_binding = args.api_caching_behavior.get_inner();
        let api_id_binding = args.api_id.get_inner();
        let at_rest_encryption_enabled_binding = args
            .at_rest_encryption_enabled
            .get_inner();
        let transit_encryption_enabled_binding = args
            .transit_encryption_enabled
            .get_inner();
        let ttl_binding = args.ttl.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/apiCache:ApiCache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiCachingBehavior".into(),
                    value: &api_caching_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "atRestEncryptionEnabled".into(),
                    value: &at_rest_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "transitEncryptionEnabled".into(),
                    value: &transit_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiCachingBehavior".into(),
                },
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "atRestEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiCacheResult {
            api_caching_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiCachingBehavior").unwrap(),
            ),
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            at_rest_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("atRestEncryptionEnabled").unwrap(),
            ),
            transit_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionEnabled").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
