/// Represents a rule that describes one or more match conditions along with the action to be taken when traffic matches this condition (allow or deny).
///
///
/// To get more information about FirewallPolicyRule, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/firewallPolicies/addRule)
///
/// ## Example Usage
///
/// ### Firewall Policy Rule
///
///
/// ```yaml
/// resources:
///   basicGlobalNetworksecurityAddressGroup:
///     type: gcp:networksecurity:AddressGroup
///     name: basic_global_networksecurity_address_group
///     properties:
///       name: address
///       parent: organizations/123456789
///       description: Sample global networksecurity_address_group
///       location: global
///       items:
///         - 208.80.154.224/32
///       type: IPV4
///       capacity: 100
///   folder:
///     type: gcp:organizations:Folder
///     properties:
///       displayName: folder
///       parent: organizations/123456789
///       deletionProtection: false
///   default:
///     type: gcp:compute:FirewallPolicy
///     properties:
///       parent: ${folder.id}
///       shortName: policy
///       description: Resource created for Terraform acceptance testing
///   policyRule:
///     type: gcp:compute:FirewallPolicyRule
///     name: policy_rule
///     properties:
///       firewallPolicy: ${default.name}
///       description: Resource created for Terraform acceptance testing
///       priority: 9000
///       enableLogging: true
///       action: allow
///       direction: EGRESS
///       disabled: false
///       match:
///         layer4Configs:
///           - ipProtocol: tcp
///             ports:
///               - 8080
///           - ipProtocol: udp
///             ports:
///               - 22
///         destIpRanges:
///           - 11.100.0.1/32
///         destFqdns: []
///         destRegionCodes:
///           - US
///         destThreatIntelligences:
///           - iplist-known-malicious-ips
///         srcAddressGroups: []
///         destAddressGroups:
///           - ${basicGlobalNetworksecurityAddressGroup.id}
///       targetServiceAccounts:
///         - my@service-account.com
/// ```
///
/// ## Import
///
/// FirewallPolicyRule can be imported using any of these accepted formats:
///
/// * `locations/global/firewallPolicies/{{firewall_policy}}/rules/{{priority}}`
///
/// * `{{firewall_policy}}/{{priority}}`
///
/// When using the `pulumi import` command, FirewallPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyRule:FirewallPolicyRule default locations/global/firewallPolicies/{{firewall_policy}}/rules/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyRule:FirewallPolicyRule default {{firewall_policy}}/{{priority}}
/// ```
///
pub mod firewall_policy_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleArgs {
        /// The Action to perform when the client connection triggers the rule. Valid actions are "allow", "deny", "goto_next" and "apply_security_profile_group".
        #[builder(into)]
        pub action: pulumi_wasm_rust::InputOrOutput<String>,
        /// An optional description for this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The direction in which this rule applies.
        /// Possible values are: `INGRESS`, `EGRESS`.
        #[builder(into)]
        pub direction: pulumi_wasm_rust::InputOrOutput<String>,
        /// Denotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and
        /// traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on
        /// "goto_next" rules.
        #[builder(into, default)]
        pub enable_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The firewall policy of the resource.
        #[builder(into)]
        pub firewall_policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        #[builder(into)]
        pub match_: pulumi_wasm_rust::InputOrOutput<
            super::super::types::compute::FirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions.
        #[builder(into, default)]
        pub security_profile_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        #[builder(into, default)]
        pub target_resources: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        #[builder(into, default)]
        pub target_service_accounts: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Boolean flag indicating if the traffic should be TLS decrypted. Can be set only if action =
        /// 'apply_security_profile_group' and cannot be set for other actions.
        #[builder(into, default)]
        pub tls_inspect: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleResult {
        /// The Action to perform when the client connection triggers the rule. Valid actions are "allow", "deny", "goto_next" and "apply_security_profile_group".
        pub action: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description for this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The direction in which this rule applies.
        /// Possible values are: `INGRESS`, `EGRESS`.
        pub direction: pulumi_wasm_rust::Output<String>,
        /// Denotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and
        /// traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on
        /// "goto_next" rules.
        pub enable_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// The firewall policy of the resource.
        pub firewall_policy: pulumi_wasm_rust::Output<String>,
        /// Type of the resource. Always `compute#firewallPolicyRule` for firewall policy rules
        pub kind: pulumi_wasm_rust::Output<String>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        pub match_: pulumi_wasm_rust::Output<
            super::super::types::compute::FirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Calculation of the complexity of a single firewall policy rule.
        pub rule_tuple_count: pulumi_wasm_rust::Output<i32>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions.
        pub security_profile_group: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        pub target_resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        pub target_service_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag indicating if the traffic should be TLS decrypted. Can be set only if action =
        /// 'apply_security_profile_group' and cannot be set for other actions.
        pub tls_inspect: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallPolicyRuleArgs,
    ) -> FirewallPolicyRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let direction_binding = args.direction.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let enable_logging_binding = args.enable_logging.get_output(context).get_inner();
        let firewall_policy_binding = args
            .firewall_policy
            .get_output(context)
            .get_inner();
        let match__binding = args.match_.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let security_profile_group_binding = args
            .security_profile_group
            .get_output(context)
            .get_inner();
        let target_resources_binding = args
            .target_resources
            .get_output(context)
            .get_inner();
        let target_service_accounts_binding = args
            .target_service_accounts
            .get_output(context)
            .get_inner();
        let tls_inspect_binding = args.tls_inspect.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicyRule:FirewallPolicyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "direction".into(),
                    value: &direction_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "enableLogging".into(),
                    value: &enable_logging_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding,
                },
                register_interface::ObjectField {
                    name: "match".into(),
                    value: &match__binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "securityProfileGroup".into(),
                    value: &security_profile_group_binding,
                },
                register_interface::ObjectField {
                    name: "targetResources".into(),
                    value: &target_resources_binding,
                },
                register_interface::ObjectField {
                    name: "targetServiceAccounts".into(),
                    value: &target_service_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "tlsInspect".into(),
                    value: &tls_inspect_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallPolicyRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            direction: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("direction"),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            enable_logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableLogging"),
            ),
            firewall_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallPolicy"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            match_: pulumi_wasm_rust::__private::into_domain(o.extract_field("match")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            rule_tuple_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ruleTupleCount"),
            ),
            security_profile_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityProfileGroup"),
            ),
            target_resources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetResources"),
            ),
            target_service_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetServiceAccounts"),
            ),
            tls_inspect: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tlsInspect"),
            ),
        }
    }
}
