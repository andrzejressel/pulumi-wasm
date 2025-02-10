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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleArgs {
        /// The Action to perform when the client connection triggers the rule. Valid actions are "allow", "deny", "goto_next" and "apply_security_profile_group".
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description for this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The direction in which this rule applies.
        /// Possible values are: `INGRESS`, `EGRESS`.
        #[builder(into)]
        pub direction: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Denotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and
        /// traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on
        /// "goto_next" rules.
        #[builder(into, default)]
        pub enable_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The firewall policy of the resource.
        #[builder(into)]
        pub firewall_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        #[builder(into)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::FirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions.
        #[builder(into, default)]
        pub security_profile_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        #[builder(into, default)]
        pub target_resources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        #[builder(into, default)]
        pub target_service_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Boolean flag indicating if the traffic should be TLS decrypted. Can be set only if action =
        /// 'apply_security_profile_group' and cannot be set for other actions.
        #[builder(into, default)]
        pub tls_inspect: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleResult {
        /// The Action to perform when the client connection triggers the rule. Valid actions are "allow", "deny", "goto_next" and "apply_security_profile_group".
        pub action: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description for this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The direction in which this rule applies.
        /// Possible values are: `INGRESS`, `EGRESS`.
        pub direction: pulumi_gestalt_rust::Output<String>,
        /// Denotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and
        /// traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on
        /// "goto_next" rules.
        pub enable_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The firewall policy of the resource.
        pub firewall_policy: pulumi_gestalt_rust::Output<String>,
        /// Type of the resource. Always `compute#firewallPolicyRule` for firewall policy rules
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        pub match_: pulumi_gestalt_rust::Output<
            super::super::types::compute::FirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Calculation of the complexity of a single firewall policy rule.
        pub rule_tuple_count: pulumi_gestalt_rust::Output<i32>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions.
        pub security_profile_group: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        pub target_resources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        pub target_service_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Boolean flag indicating if the traffic should be TLS decrypted. Can be set only if action =
        /// 'apply_security_profile_group' and cannot be set for other actions.
        pub tls_inspect: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallPolicyRuleArgs,
    ) -> FirewallPolicyRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let direction_binding = args.direction.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let enable_logging_binding = args.enable_logging.get_output(context);
        let firewall_policy_binding = args.firewall_policy.get_output(context);
        let match__binding = args.match_.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let security_profile_group_binding = args
            .security_profile_group
            .get_output(context);
        let target_resources_binding = args.target_resources.get_output(context);
        let target_service_accounts_binding = args
            .target_service_accounts
            .get_output(context);
        let tls_inspect_binding = args.tls_inspect.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicyRule:FirewallPolicyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "direction".into(),
                    value: direction_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableLogging".into(),
                    value: enable_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicy".into(),
                    value: firewall_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "match".into(),
                    value: match__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityProfileGroup".into(),
                    value: security_profile_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResources".into(),
                    value: target_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccounts".into(),
                    value: target_service_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsInspect".into(),
                    value: tls_inspect_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallPolicyRuleResult {
            action: o.get_field("action"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            direction: o.get_field("direction"),
            disabled: o.get_field("disabled"),
            enable_logging: o.get_field("enableLogging"),
            firewall_policy: o.get_field("firewallPolicy"),
            kind: o.get_field("kind"),
            match_: o.get_field("match"),
            priority: o.get_field("priority"),
            rule_tuple_count: o.get_field("ruleTupleCount"),
            security_profile_group: o.get_field("securityProfileGroup"),
            target_resources: o.get_field("targetResources"),
            target_service_accounts: o.get_field("targetServiceAccounts"),
            tls_inspect: o.get_field("tlsInspect"),
        }
    }
}
