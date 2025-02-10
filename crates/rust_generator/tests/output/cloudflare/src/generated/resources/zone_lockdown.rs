/// Provides a Cloudflare Zone Lockdown resource. Zone Lockdown allows
/// you to define one or more URLs (with wildcard matching on the domain
/// or path) that will only permit access if the request originates
/// from an IP address that matches a safelist of one or more IP
/// addresses and/or IP ranges.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Restrict access to these endpoints to requests from a known IP address range.
///   example:
///     type: cloudflare:ZoneLockdown
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       paused: 'false'
///       description: Restrict access to these endpoints to requests from a known IP address range
///       urls:
///         - api.mysite.com/some/endpoint*
///       configurations:
///         - target: ip_range
///           value: 192.0.2.0/24
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneLockdown:ZoneLockdown example <zone_id>/<lockdown_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone_lockdown {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneLockdownArgs {
        /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
        #[builder(into)]
        pub configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::ZoneLockdownConfiguration>,
        >,
        /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
        #[builder(into, default)]
        pub paused: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
        #[builder(into)]
        pub urls: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneLockdownResult {
        /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::types::ZoneLockdownConfiguration>,
        >,
        /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
        pub paused: pulumi_gestalt_rust::Output<Option<bool>>,
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
        pub urls: pulumi_gestalt_rust::Output<Vec<String>>,
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
        args: ZoneLockdownArgs,
    ) -> ZoneLockdownResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configurations_binding = args.configurations.get_output(context);
        let description_binding = args.description.get_output(context);
        let paused_binding = args.paused.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let urls_binding = args.urls.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zoneLockdown:ZoneLockdown".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurations".into(),
                    value: configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "paused".into(),
                    value: paused_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urls".into(),
                    value: urls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneLockdownResult {
            configurations: o.get_field("configurations"),
            description: o.get_field("description"),
            paused: o.get_field("paused"),
            priority: o.get_field("priority"),
            urls: o.get_field("urls"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
