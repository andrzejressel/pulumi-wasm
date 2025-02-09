/// Cloudflare Argo controls the routing to your origin and tiered
/// caching options to speed up your website browsing experience.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = argo::create(
///         "example",
///         ArgoArgs::builder()
///             .smart_routing("on")
///             .tiered_caching("on")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/argo:Argo example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod argo {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ArgoArgs {
        /// Whether smart routing is enabled. Available values: `on`, `off`.
        #[builder(into, default)]
        pub smart_routing: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether tiered caching is enabled. Available values: `on`, `off`.
        #[builder(into, default)]
        pub tiered_caching: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ArgoResult {
        /// Whether smart routing is enabled. Available values: `on`, `off`.
        pub smart_routing: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether tiered caching is enabled. Available values: `on`, `off`.
        pub tiered_caching: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ArgoArgs,
    ) -> ArgoResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let smart_routing_binding = args.smart_routing.get_output(context);
        let tiered_caching_binding = args.tiered_caching.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/argo:Argo".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smartRouting".into(),
                    value: smart_routing_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tieredCaching".into(),
                    value: tiered_caching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ArgoResult {
            smart_routing: o.get_field("smartRouting"),
            tiered_caching: o.get_field("tieredCaching"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
