/// Represents a rule that describes one or more match conditions along with the action to be taken when traffic matches this condition (allow or deny).
///
///
/// To get more information about RegionNetworkFirewallPolicyRule, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionNetworkFirewallPolicies/addRule)
///
/// ## Example Usage
///
/// ### Region Network Firewall Policy Rule
///
///
/// ```yaml
/// resources:
///   basicRegionalNetworksecurityAddressGroup:
///     type: gcp:networksecurity:AddressGroup
///     name: basic_regional_networksecurity_address_group
///     properties:
///       name: address
///       parent: projects/my-project-name
///       description: Sample regional networksecurity_address_group
///       location: us-west1
///       items:
///         - 208.80.154.224/32
///       type: IPV4
///       capacity: 100
///   basicRegionalNetworkFirewallPolicy:
///     type: gcp:compute:RegionNetworkFirewallPolicy
///     name: basic_regional_network_firewall_policy
///     properties:
///       name: policy
///       description: Sample regional network firewall policy
///       project: my-project-name
///       region: us-west1
///   primary:
///     type: gcp:compute:RegionNetworkFirewallPolicyRule
///     properties:
///       action: allow
///       description: This is a simple rule description
///       direction: INGRESS
///       disabled: false
///       enableLogging: true
///       firewallPolicy: ${basicRegionalNetworkFirewallPolicy.name}
///       priority: 1000
///       region: us-west1
///       ruleName: test-rule
///       targetServiceAccounts:
///         - my@service-account.com
///       match:
///         srcIpRanges:
///           - 10.100.0.1/32
///         srcFqdns:
///           - example.com
///         srcRegionCodes:
///           - US
///         srcThreatIntelligences:
///           - iplist-known-malicious-ips
///         layer4Configs:
///           - ipProtocol: all
///         srcSecureTags:
///           - name: ${basicValue.id}
///         srcAddressGroups:
///           - ${basicRegionalNetworksecurityAddressGroup.id}
///   basicNetwork:
///     type: gcp:compute:Network
///     name: basic_network
///     properties:
///       name: network
///   basicKey:
///     type: gcp:tags:TagKey
///     name: basic_key
///     properties:
///       description: For keyname resources.
///       parent: organizations/123456789
///       purpose: GCE_FIREWALL
///       shortName: tagkey
///       purposeData:
///         network: my-project-name/${basicNetwork.name}
///   basicValue:
///     type: gcp:tags:TagValue
///     name: basic_value
///     properties:
///       description: For valuename resources.
///       parent: ${basicKey.id}
///       shortName: tagvalue
/// ```
///
/// ## Import
///
/// RegionNetworkFirewallPolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/firewallPolicies/{{firewall_policy}}/{{priority}}`
///
/// * `{{project}}/{{region}}/{{firewall_policy}}/{{priority}}`
///
/// * `{{region}}/{{firewall_policy}}/{{priority}}`
///
/// * `{{firewall_policy}}/{{priority}}`
///
/// When using the `pulumi import` command, RegionNetworkFirewallPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyRule:RegionNetworkFirewallPolicyRule default projects/{{project}}/regions/{{region}}/firewallPolicies/{{firewall_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyRule:RegionNetworkFirewallPolicyRule default {{project}}/{{region}}/{{firewall_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyRule:RegionNetworkFirewallPolicyRule default {{region}}/{{firewall_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyRule:RegionNetworkFirewallPolicyRule default {{firewall_policy}}/{{priority}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_network_firewall_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionNetworkFirewallPolicyRuleArgs {
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
            super::super::types::compute::RegionNetworkFirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of this resource.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional name for the rule. This field is not a unique identifier and can be updated.
        #[builder(into, default)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions. Security Profile
        /// Group and Firewall Policy Rule must be in the same scope.
        #[builder(into, default)]
        pub security_profile_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of secure tags that controls which instances the firewall rule applies to. If targetSecureTag are specified, then
        /// the firewall rule applies only to instances in the VPC network that have one of those EFFECTIVE secure tags, if all the
        /// targetSecureTag are in INEFFECTIVE state, then this rule will be ignored. targetSecureTag may not be set at the same
        /// time as targetServiceAccounts. If neither targetServiceAccounts nor targetSecureTag are specified, the firewall rule
        /// applies to all instances on the specified network. Maximum number of target label tags allowed is 256.
        #[builder(into, default)]
        pub target_secure_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::RegionNetworkFirewallPolicyRuleTargetSecureTag,
                >,
            >,
        >,
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
    pub struct RegionNetworkFirewallPolicyRuleResult {
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
            super::super::types::compute::RegionNetworkFirewallPolicyRuleMatch,
        >,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The location of this resource.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// An optional name for the rule. This field is not a unique identifier and can be updated.
        pub rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Calculation of the complexity of a single firewall policy rule.
        pub rule_tuple_count: pulumi_gestalt_rust::Output<i32>,
        /// A fully-qualified URL of a SecurityProfile resource instance. Example:
        /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
        /// Must be specified if action = 'apply_security_profile_group' and cannot be specified for other actions. Security Profile
        /// Group and Firewall Policy Rule must be in the same scope.
        pub security_profile_group: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of secure tags that controls which instances the firewall rule applies to. If targetSecureTag are specified, then
        /// the firewall rule applies only to instances in the VPC network that have one of those EFFECTIVE secure tags, if all the
        /// targetSecureTag are in INEFFECTIVE state, then this rule will be ignored. targetSecureTag may not be set at the same
        /// time as targetServiceAccounts. If neither targetServiceAccounts nor targetSecureTag are specified, the firewall rule
        /// applies to all instances on the specified network. Maximum number of target label tags allowed is 256.
        pub target_secure_tags: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::compute::RegionNetworkFirewallPolicyRuleTargetSecureTag,
                >,
            >,
        >,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionNetworkFirewallPolicyRuleArgs,
    ) -> RegionNetworkFirewallPolicyRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let rule_name_binding = args.rule_name.get_output(context).get_inner();
        let security_profile_group_binding = args
            .security_profile_group
            .get_output(context)
            .get_inner();
        let target_secure_tags_binding = args
            .target_secure_tags
            .get_output(context)
            .get_inner();
        let target_service_accounts_binding = args
            .target_service_accounts
            .get_output(context)
            .get_inner();
        let tls_inspect_binding = args.tls_inspect.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionNetworkFirewallPolicyRule:RegionNetworkFirewallPolicyRule"
                .into(),
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding,
                },
                register_interface::ObjectField {
                    name: "securityProfileGroup".into(),
                    value: &security_profile_group_binding,
                },
                register_interface::ObjectField {
                    name: "targetSecureTags".into(),
                    value: &target_secure_tags_binding,
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
        RegionNetworkFirewallPolicyRuleResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            direction: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("direction"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            enable_logging: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableLogging"),
            ),
            firewall_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicy"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            match_: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("match"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            rule_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleName"),
            ),
            rule_tuple_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleTupleCount"),
            ),
            security_profile_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityProfileGroup"),
            ),
            target_secure_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetSecureTags"),
            ),
            target_service_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetServiceAccounts"),
            ),
            tls_inspect: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsInspect"),
            ),
        }
    }
}
