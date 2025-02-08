/// Instructs Cloudflare to check a regional hub data center on the way to your upper tier.
/// This can help improve performance for smart and custom tiered cache topologies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = regional_tiered_cache::create(
///         "example",
///         RegionalTieredCacheArgs::builder()
///             .value("on")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/regionalTieredCache:RegionalTieredCache example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regional_tiered_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalTieredCacheArgs {
        /// Value of the Regional Tiered Cache zone setting.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalTieredCacheResult {
        /// Value of the Regional Tiered Cache zone setting.
        pub value: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionalTieredCacheArgs,
    ) -> RegionalTieredCacheResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let value_binding = args.value.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/regionalTieredCache:RegionalTieredCache".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionalTieredCacheResult {
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
