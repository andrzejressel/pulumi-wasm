/// Manages a Firewall Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFirewallPolicy = firewall_policy::create(
///         "exampleFirewallPolicy",
///         FirewallPolicyArgs::builder()
///             .location("${example.location}")
///             .name("example-policy")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Firewall Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewallPolicy:FirewallPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/firewallPolicies/policy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// Whether enable auto learn private ip range.
        #[builder(into, default)]
        pub auto_learn_private_ranges_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the base Firewall Policy.
        #[builder(into, default)]
        pub base_policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `dns` block as defined below.
        #[builder(into, default)]
        pub dns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyDns>,
        >,
        /// A `explicit_proxy` block as defined below.
        #[builder(into, default)]
        pub explicit_proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyExplicitProxy>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyIdentity>,
        >,
        /// An `insights` block as defined below.
        #[builder(into, default)]
        pub insights: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyInsights>,
        >,
        /// A `intrusion_detection` block as defined below.
        #[builder(into, default)]
        pub intrusion_detection: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyIntrusionDetection>,
        >,
        /// The Azure Region where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Firewall Policy. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of private IP ranges to which traffic will not be SNAT.
        #[builder(into, default)]
        pub private_ip_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Resource Group where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU Tier of the Firewall Policy. Possible values are `Standard`, `Premium` and `Basic`. Defaults to `Standard`. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether SQL Redirect traffic filtering is allowed. Enabling this flag requires no rule using ports between `11000`-`11999`.
        #[builder(into, default)]
        pub sql_redirect_allowed: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags which should be assigned to the Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `threat_intelligence_allowlist` block as defined below.
        #[builder(into, default)]
        pub threat_intelligence_allowlist: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::network::FirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        /// The operation mode for Threat Intelligence. Possible values are `Alert`, `Deny` and `Off`. Defaults to `Alert`.
        #[builder(into, default)]
        pub threat_intelligence_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `tls_certificate` block as defined below.
        #[builder(into, default)]
        pub tls_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyTlsCertificate>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// Whether enable auto learn private ip range.
        pub auto_learn_private_ranges_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the base Firewall Policy.
        pub base_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of reference to child Firewall Policies of this Firewall Policy.
        pub child_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `dns` block as defined below.
        pub dns: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyDns>,
        >,
        /// A `explicit_proxy` block as defined below.
        pub explicit_proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyExplicitProxy>,
        >,
        /// A list of references to Azure Firewalls that this Firewall Policy is associated with.
        pub firewalls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyIdentity>,
        >,
        /// An `insights` block as defined below.
        pub insights: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyInsights>,
        >,
        /// A `intrusion_detection` block as defined below.
        pub intrusion_detection: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyIntrusionDetection>,
        >,
        /// The Azure Region where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Firewall Policy. Changing this forces a new Firewall Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of private IP ranges to which traffic will not be SNAT.
        pub private_ip_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of references to Firewall Policy Rule Collection Groups that belongs to this Firewall Policy.
        pub rule_collection_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The SKU Tier of the Firewall Policy. Possible values are `Standard`, `Premium` and `Basic`. Defaults to `Standard`. Changing this forces a new Firewall Policy to be created.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether SQL Redirect traffic filtering is allowed. Enabling this flag requires no rule using ports between `11000`-`11999`.
        pub sql_redirect_allowed: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Firewall Policy.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `threat_intelligence_allowlist` block as defined below.
        pub threat_intelligence_allowlist: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::network::FirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        /// The operation mode for Threat Intelligence. Possible values are `Alert`, `Deny` and `Off`. Defaults to `Alert`.
        pub threat_intelligence_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `tls_certificate` block as defined below.
        pub tls_certificate: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::FirewallPolicyTlsCertificate>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallPolicyArgs,
    ) -> FirewallPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_learn_private_ranges_enabled_binding = args
            .auto_learn_private_ranges_enabled
            .get_output(context);
        let base_policy_id_binding = args.base_policy_id.get_output(context);
        let dns_binding = args.dns.get_output(context);
        let explicit_proxy_binding = args.explicit_proxy.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let insights_binding = args.insights.get_output(context);
        let intrusion_detection_binding = args.intrusion_detection.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let private_ip_ranges_binding = args.private_ip_ranges.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let sql_redirect_allowed_binding = args.sql_redirect_allowed.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let threat_intelligence_allowlist_binding = args
            .threat_intelligence_allowlist
            .get_output(context);
        let threat_intelligence_mode_binding = args
            .threat_intelligence_mode
            .get_output(context);
        let tls_certificate_binding = args.tls_certificate.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoLearnPrivateRangesEnabled".into(),
                    value: auto_learn_private_ranges_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basePolicyId".into(),
                    value: base_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dns".into(),
                    value: dns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "explicitProxy".into(),
                    value: explicit_proxy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insights".into(),
                    value: insights_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "intrusionDetection".into(),
                    value: intrusion_detection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpRanges".into(),
                    value: private_ip_ranges_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlRedirectAllowed".into(),
                    value: sql_redirect_allowed_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatIntelligenceAllowlist".into(),
                    value: threat_intelligence_allowlist_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatIntelligenceMode".into(),
                    value: threat_intelligence_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsCertificate".into(),
                    value: tls_certificate_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallPolicyResult {
            auto_learn_private_ranges_enabled: o
                .get_field("autoLearnPrivateRangesEnabled"),
            base_policy_id: o.get_field("basePolicyId"),
            child_policies: o.get_field("childPolicies"),
            dns: o.get_field("dns"),
            explicit_proxy: o.get_field("explicitProxy"),
            firewalls: o.get_field("firewalls"),
            identity: o.get_field("identity"),
            insights: o.get_field("insights"),
            intrusion_detection: o.get_field("intrusionDetection"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_ip_ranges: o.get_field("privateIpRanges"),
            resource_group_name: o.get_field("resourceGroupName"),
            rule_collection_groups: o.get_field("ruleCollectionGroups"),
            sku: o.get_field("sku"),
            sql_redirect_allowed: o.get_field("sqlRedirectAllowed"),
            tags: o.get_field("tags"),
            threat_intelligence_allowlist: o.get_field("threatIntelligenceAllowlist"),
            threat_intelligence_mode: o.get_field("threatIntelligenceMode"),
            tls_certificate: o.get_field("tlsCertificate"),
        }
    }
}
