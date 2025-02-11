/// Provides an AppSync API Cache.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiCacheArgs {
        /// Caching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.
        #[builder(into)]
        pub api_caching_behavior: pulumi_gestalt_rust::InputOrOutput<String>,
        /// GraphQL API ID.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// At-rest encryption flag for cache. You cannot update this setting after creation.
        #[builder(into, default)]
        pub at_rest_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Transit encryption flag when connecting to cache. You cannot update this setting after creation.
        #[builder(into, default)]
        pub transit_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// TTL in seconds for cache entries.
        #[builder(into)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiCacheResult {
        /// Caching behavior. Valid values are `FULL_REQUEST_CACHING` and `PER_RESOLVER_CACHING`.
        pub api_caching_behavior: pulumi_gestalt_rust::Output<String>,
        /// GraphQL API ID.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// At-rest encryption flag for cache. You cannot update this setting after creation.
        pub at_rest_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Transit encryption flag when connecting to cache. You cannot update this setting after creation.
        pub transit_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// TTL in seconds for cache entries.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        /// Cache instance type. Valid values are `SMALL`, `MEDIUM`, `LARGE`, `XLARGE`, `LARGE_2X`, `LARGE_4X`, `LARGE_8X`, `LARGE_12X`, `T2_SMALL`, `T2_MEDIUM`, `R4_LARGE`, `R4_XLARGE`, `R4_2XLARGE`, `R4_4XLARGE`, `R4_8XLARGE`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiCacheArgs,
    ) -> ApiCacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_caching_behavior_binding = args.api_caching_behavior.get_output(context);
        let api_id_binding = args.api_id.get_output(context);
        let at_rest_encryption_enabled_binding = args
            .at_rest_encryption_enabled
            .get_output(context);
        let transit_encryption_enabled_binding = args
            .transit_encryption_enabled
            .get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/apiCache:ApiCache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiCachingBehavior".into(),
                    value: &api_caching_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "atRestEncryptionEnabled".into(),
                    value: &at_rest_encryption_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitEncryptionEnabled".into(),
                    value: &transit_encryption_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiCacheResult {
            api_caching_behavior: o.get_field("apiCachingBehavior"),
            api_id: o.get_field("apiId"),
            at_rest_encryption_enabled: o.get_field("atRestEncryptionEnabled"),
            transit_encryption_enabled: o.get_field("transitEncryptionEnabled"),
            ttl: o.get_field("ttl"),
            type_: o.get_field("type"),
        }
    }
}
