/// ## Example Usage
///
/// ### Compute Network Firewall Policy With Rules Full
///
///
/// ```yaml
/// resources:
///   network-firewall-policy-with-rules:
///     type: gcp:compute:NetworkFirewallPolicyWithRules
///     properties:
///       name: tf-fw-policy-with-rules
///       description: Terraform test
///       rules:
///         - description: tcp rule
///           priority: 1000
///           enableLogging: true
///           action: allow
///           direction: EGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: tcp
///                 ports:
///                   - 8080
///                   - 7070
///             destIpRanges:
///               - 11.100.0.1/32
///             destFqdns:
///               - www.yyy.com
///               - www.zzz.com
///             destRegionCodes:
///               - HK
///               - IN
///             destThreatIntelligences:
///               - iplist-search-engines-crawlers
///               - iplist-tor-exit-nodes
///             destAddressGroups:
///               - ${addressGroup1.id}
///           targetSecureTags:
///             - name: ${secureTagValue1.id}
///         - description: udp rule
///           priority: 2000
///           enableLogging: false
///           action: deny
///           direction: INGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: udp
///             srcIpRanges:
///               - 0.0.0.0/0
///             srcFqdns:
///               - www.abc.com
///               - www.def.com
///             srcRegionCodes:
///               - US
///               - CA
///             srcThreatIntelligences:
///               - iplist-known-malicious-ips
///               - iplist-public-clouds
///             srcAddressGroups:
///               - ${addressGroup1.id}
///             srcSecureTags:
///               - name: ${secureTagValue1.id}
///           disabled: true
///         - description: security profile group rule
///           ruleName: tcp rule
///           priority: 3000
///           enableLogging: false
///           action: apply_security_profile_group
///           direction: INGRESS
///           match:
///             layer4Configs:
///               - ipProtocol: tcp
///             srcIpRanges:
///               - 0.0.0.0/0
///           targetServiceAccounts:
///             - test@google.com
///           securityProfileGroup: //networksecurity.googleapis.com/${securityProfileGroup1.id}
///           tlsInspect: true
///   addressGroup1:
///     type: gcp:networksecurity:AddressGroup
///     name: address_group_1
///     properties:
///       name: tf-address-group
///       parent: ${project.id}
///       description: Global address group
///       location: global
///       items:
///         - 208.80.154.224/32
///       type: IPV4
///       capacity: 100
///   secureTagKey1:
///     type: gcp:tags:TagKey
///     name: secure_tag_key_1
///     properties:
///       description: Tag key
///       parent: ${project.id}
///       purpose: GCE_FIREWALL
///       shortName: tf-tag-key
///       purposeData:
///         network: ${project.name}/default
///   secureTagValue1:
///     type: gcp:tags:TagValue
///     name: secure_tag_value_1
///     properties:
///       description: Tag value
///       parent: ${secureTagKey1.id}
///       shortName: tf-tag-value
///   securityProfileGroup1:
///     type: gcp:networksecurity:SecurityProfileGroup
///     name: security_profile_group_1
///     properties:
///       name: tf-security-profile-group
///       parent: organizations/123456789
///       description: my description
///       threatPreventionProfile: ${securityProfile1.id}
///   securityProfile1:
///     type: gcp:networksecurity:SecurityProfile
///     name: security_profile_1
///     properties:
///       name: tf-security-profile
///       type: THREAT_PREVENTION
///       parent: organizations/123456789
///       location: global
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// NetworkFirewallPolicyWithRules can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/firewallPolicies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NetworkFirewallPolicyWithRules can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyWithRules:NetworkFirewallPolicyWithRules default projects/{{project}}/global/firewallPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyWithRules:NetworkFirewallPolicyWithRules default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyWithRules:NetworkFirewallPolicyWithRules default {{name}}
/// ```
///
pub mod network_firewall_policy_with_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkFirewallPolicyWithRulesArgs {
        /// (Output)
        /// A description of the rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User-provided name of the Network firewall policy.
        /// The name should be unique in the project in which the firewall policy is created.
        /// The name must be 1-63 characters long, and comply with RFC1035. Specifically,
        /// the name must be 1-63 characters long and match the regular expression a-z?
        /// which means the first character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of firewall policy rules.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::compute::NetworkFirewallPolicyWithRulesRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkFirewallPolicyWithRulesResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// A description of the rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of the resource. This field is used internally during updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// User-provided name of the Network firewall policy.
        /// The name should be unique in the project in which the firewall policy is created.
        /// The name must be 1-63 characters long, and comply with RFC1035. Specifically,
        /// the name must be 1-63 characters long and match the regular expression a-z?
        /// which means the first character must be a lowercase letter, and all following characters must be a dash,
        /// lowercase letter, or digit, except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub network_firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// A list of firewall policy pre-defined rules.
        /// Structure is documented below.
        pub predefined_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::compute::NetworkFirewallPolicyWithRulesPredefinedRule,
            >,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Total count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples.
        pub rule_tuple_count: pulumi_wasm_rust::Output<i32>,
        /// A list of firewall policy rules.
        /// Structure is documented below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::NetworkFirewallPolicyWithRulesRule>,
        >,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkFirewallPolicyWithRulesArgs,
    ) -> NetworkFirewallPolicyWithRulesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/networkFirewallPolicyWithRules:NetworkFirewallPolicyWithRules"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkFirewallPolicyWithRulesResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkFirewallPolicyId"),
            ),
            predefined_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predefinedRules"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            rule_tuple_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ruleTupleCount"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            self_link_with_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLinkWithId"),
            ),
        }
    }
}
