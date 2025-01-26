/// Manages a Firewall Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod firewall_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// Whether enable auto learn private ip range.
        #[builder(into, default)]
        pub auto_learn_private_ranges_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the base Firewall Policy.
        #[builder(into, default)]
        pub base_policy_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `dns` block as defined below.
        #[builder(into, default)]
        pub dns: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyDns>,
        >,
        /// A `explicit_proxy` block as defined below.
        #[builder(into, default)]
        pub explicit_proxy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyExplicitProxy>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyIdentity>,
        >,
        /// An `insights` block as defined below.
        #[builder(into, default)]
        pub insights: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyInsights>,
        >,
        /// A `intrusion_detection` block as defined below.
        #[builder(into, default)]
        pub intrusion_detection: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyIntrusionDetection>,
        >,
        /// The Azure Region where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Firewall Policy. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of private IP ranges to which traffic will not be SNAT.
        #[builder(into, default)]
        pub private_ip_ranges: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Resource Group where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SKU Tier of the Firewall Policy. Possible values are `Standard`, `Premium` and `Basic`. Defaults to `Standard`. Changing this forces a new Firewall Policy to be created.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether SQL Redirect traffic filtering is allowed. Enabling this flag requires no rule using ports between `11000`-`11999`.
        #[builder(into, default)]
        pub sql_redirect_allowed: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags which should be assigned to the Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `threat_intelligence_allowlist` block as defined below.
        #[builder(into, default)]
        pub threat_intelligence_allowlist: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::network::FirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        /// The operation mode for Threat Intelligence. Possible values are `Alert`, `Deny` and `Off`. Defaults to `Alert`.
        #[builder(into, default)]
        pub threat_intelligence_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `tls_certificate` block as defined below.
        #[builder(into, default)]
        pub tls_certificate: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::FirewallPolicyTlsCertificate>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// Whether enable auto learn private ip range.
        pub auto_learn_private_ranges_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the base Firewall Policy.
        pub base_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of reference to child Firewall Policies of this Firewall Policy.
        pub child_policies: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `dns` block as defined below.
        pub dns: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyDns>,
        >,
        /// A `explicit_proxy` block as defined below.
        pub explicit_proxy: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyExplicitProxy>,
        >,
        /// A list of references to Azure Firewalls that this Firewall Policy is associated with.
        pub firewalls: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyIdentity>,
        >,
        /// An `insights` block as defined below.
        pub insights: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyInsights>,
        >,
        /// A `intrusion_detection` block as defined below.
        pub intrusion_detection: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyIntrusionDetection>,
        >,
        /// The Azure Region where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Firewall Policy. Changing this forces a new Firewall Policy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of private IP ranges to which traffic will not be SNAT.
        pub private_ip_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Firewall Policy should exist. Changing this forces a new Firewall Policy to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of references to Firewall Policy Rule Collection Groups that belongs to this Firewall Policy.
        pub rule_collection_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The SKU Tier of the Firewall Policy. Possible values are `Standard`, `Premium` and `Basic`. Defaults to `Standard`. Changing this forces a new Firewall Policy to be created.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether SQL Redirect traffic filtering is allowed. Enabling this flag requires no rule using ports between `11000`-`11999`.
        pub sql_redirect_allowed: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Firewall Policy.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `threat_intelligence_allowlist` block as defined below.
        pub threat_intelligence_allowlist: pulumi_wasm_rust::Output<
            Option<
                super::super::types::network::FirewallPolicyThreatIntelligenceAllowlist,
            >,
        >,
        /// The operation mode for Threat Intelligence. Possible values are `Alert`, `Deny` and `Off`. Defaults to `Alert`.
        pub threat_intelligence_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// A `tls_certificate` block as defined below.
        pub tls_certificate: pulumi_wasm_rust::Output<
            Option<super::super::types::network::FirewallPolicyTlsCertificate>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallPolicyArgs,
    ) -> FirewallPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_learn_private_ranges_enabled_binding = args
            .auto_learn_private_ranges_enabled
            .get_output(context)
            .get_inner();
        let base_policy_id_binding = args.base_policy_id.get_output(context).get_inner();
        let dns_binding = args.dns.get_output(context).get_inner();
        let explicit_proxy_binding = args.explicit_proxy.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let insights_binding = args.insights.get_output(context).get_inner();
        let intrusion_detection_binding = args
            .intrusion_detection
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_ip_ranges_binding = args
            .private_ip_ranges
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let sql_redirect_allowed_binding = args
            .sql_redirect_allowed
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let threat_intelligence_allowlist_binding = args
            .threat_intelligence_allowlist
            .get_output(context)
            .get_inner();
        let threat_intelligence_mode_binding = args
            .threat_intelligence_mode
            .get_output(context)
            .get_inner();
        let tls_certificate_binding = args
            .tls_certificate
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoLearnPrivateRangesEnabled".into(),
                    value: &auto_learn_private_ranges_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "basePolicyId".into(),
                    value: &base_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "explicitProxy".into(),
                    value: &explicit_proxy_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "insights".into(),
                    value: &insights_binding,
                },
                register_interface::ObjectField {
                    name: "intrusionDetection".into(),
                    value: &intrusion_detection_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpRanges".into(),
                    value: &private_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "sqlRedirectAllowed".into(),
                    value: &sql_redirect_allowed_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threatIntelligenceAllowlist".into(),
                    value: &threat_intelligence_allowlist_binding,
                },
                register_interface::ObjectField {
                    name: "threatIntelligenceMode".into(),
                    value: &threat_intelligence_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tlsCertificate".into(),
                    value: &tls_certificate_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoLearnPrivateRangesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "basePolicyId".into(),
                },
                register_interface::ResultField {
                    name: "childPolicies".into(),
                },
                register_interface::ResultField {
                    name: "dns".into(),
                },
                register_interface::ResultField {
                    name: "explicitProxy".into(),
                },
                register_interface::ResultField {
                    name: "firewalls".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "insights".into(),
                },
                register_interface::ResultField {
                    name: "intrusionDetection".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "ruleCollectionGroups".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "sqlRedirectAllowed".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threatIntelligenceAllowlist".into(),
                },
                register_interface::ResultField {
                    name: "threatIntelligenceMode".into(),
                },
                register_interface::ResultField {
                    name: "tlsCertificate".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallPolicyResult {
            auto_learn_private_ranges_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoLearnPrivateRangesEnabled").unwrap(),
            ),
            base_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basePolicyId").unwrap(),
            ),
            child_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("childPolicies").unwrap(),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dns").unwrap(),
            ),
            explicit_proxy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("explicitProxy").unwrap(),
            ),
            firewalls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewalls").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            insights: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insights").unwrap(),
            ),
            intrusion_detection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("intrusionDetection").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpRanges").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rule_collection_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleCollectionGroups").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            sql_redirect_allowed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlRedirectAllowed").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threat_intelligence_allowlist: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatIntelligenceAllowlist").unwrap(),
            ),
            threat_intelligence_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatIntelligenceMode").unwrap(),
            ),
            tls_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsCertificate").unwrap(),
            ),
        }
    }
}
