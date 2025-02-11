/// Provides a resource, that manages Cloudflare Tiered Cache settings.
/// This allows you to adjust topologies for your zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tiered_cache::create(
///         "example",
///         TieredCacheArgs::builder()
///             .cache_type("smart")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tiered_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TieredCacheArgs {
        /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
        #[builder(into)]
        pub cache_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TieredCacheResult {
        /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
        pub cache_type: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TieredCacheArgs,
    ) -> TieredCacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cache_type_binding = args.cache_type.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tieredCache:TieredCache".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheType".into(),
                    value: &cache_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TieredCacheResult {
            cache_type: o.get_field("cacheType"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
