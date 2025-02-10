/// Define Firewall rules using filter expressions for more control over
/// how traffic is matched to the rule. A filter expression permits
/// selecting traffic by multiple criteria allowing greater freedom in
/// rule creation.
///
/// Filter expressions needs to be created first before using Firewall
/// Rule.
///
/// > `cloudflare.FirewallRule` is in a deprecation phase until January 15th, 2025.
///   During this time period, this resource is still
///   fully supported but you are strongly advised  to move to the
///   `cloudflare.Ruleset` resource. Full details can be found in the
///   developer documentation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let wordpress = filter::create(
///         "wordpress",
///         FilterArgs::builder()
///             .description("Wordpress break-in attempts that are outside of the office")
///             .expression(
///                 "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let wordpressFirewallRule = firewall_rule::create(
///         "wordpressFirewallRule",
///         FirewallRuleArgs::builder()
///             .action("block")
///             .description("Block wordpress break-in attempts")
///             .filter_id("${wordpress.id}")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/firewallRule:FirewallRule example <zone_id>/<firewall_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the rule to help identify it.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
        #[builder(into)]
        pub filter_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether this filter based firewall rule is currently paused.
        #[builder(into, default)]
        pub paused: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
        #[builder(into, default)]
        pub products: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// A description of the rule to help identify it.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
        pub filter_id: pulumi_gestalt_rust::Output<String>,
        /// Whether this filter based firewall rule is currently paused.
        pub paused: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
        pub products: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
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
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let filter_id_binding = args.filter_id.get_output(context);
        let paused_binding = args.paused.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let products_binding = args.products.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterId".into(),
                    value: filter_id_binding.get_id(),
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
                    name: "products".into(),
                    value: products_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallRuleResult {
            action: o.get_field("action"),
            description: o.get_field("description"),
            filter_id: o.get_field("filterId"),
            paused: o.get_field("paused"),
            priority: o.get_field("priority"),
            products: o.get_field("products"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
