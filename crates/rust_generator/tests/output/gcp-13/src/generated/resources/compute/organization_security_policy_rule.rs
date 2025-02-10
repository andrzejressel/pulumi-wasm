/// A rule for the OrganizationSecurityPolicy.
///
/// To get more information about OrganizationSecurityPolicyRule, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/organizationSecurityPolicies/addRule)
/// * How-to Guides
///     * [Creating firewall rules](https://cloud.google.com/vpc/docs/using-firewall-policies#create-rules)
///
/// ## Example Usage
///
/// ### Organization Security Policy Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = organization_security_policy::create(
///         "policy",
///         OrganizationSecurityPolicyArgs::builder()
///             .display_name("tf-test")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
///     let policyOrganizationSecurityPolicyRule = organization_security_policy_rule::create(
///         "policyOrganizationSecurityPolicyRule",
///         OrganizationSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .direction("INGRESS")
///             .enable_logging(true)
///             .match_(
///                 OrganizationSecurityPolicyRuleMatch::builder()
///                     .config(
///                         OrganizationSecurityPolicyRuleMatchConfig::builder()
///                             .layer4Configs(
///                                 vec![
///                                     OrganizationSecurityPolicyRuleMatchConfigLayer4Config::builder()
///                                     .ipProtocol("tcp").ports(vec!["22",]).build_struct(),
///                                     OrganizationSecurityPolicyRuleMatchConfigLayer4Config::builder()
///                                     .ipProtocol("icmp").build_struct(),
///                                 ],
///                             )
///                             .srcIpRanges(vec!["192.168.0.0/16", "10.0.0.0/8",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .policy_id("${policy.id}")
///             .priority(100)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationSecurityPolicyRule can be imported using any of these accepted formats:
///
/// * `{{policy_id}}/priority/{{priority}}`
///
/// When using the `pulumi import` command, OrganizationSecurityPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicyRule:OrganizationSecurityPolicyRule default {{policy_id}}/priority/{{priority}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_security_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyRuleArgs {
        /// The Action to perform when the client connection triggers the rule. Can currently be either
        /// "allow", "deny" or "goto_next".
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: ["INGRESS",
        /// "EGRESS"]
        #[builder(into, default)]
        pub direction: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver.
        #[builder(into, default)]
        pub enable_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        #[builder(into)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::OrganizationSecurityPolicyRuleMatch,
        >,
        /// The ID of the OrganizationSecurityPolicy this rule applies to.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If set to true, the specified action is not enforced.
        #[builder(into, default)]
        pub preview: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An integer indicating the priority of a rule in the list. The priority must be a value
        /// between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the
        /// highest priority and 2147483647 is the lowest prority.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        #[builder(into, default)]
        pub target_resources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        #[builder(into, default)]
        pub target_service_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyRuleResult {
        /// The Action to perform when the client connection triggers the rule. Can currently be either
        /// "allow", "deny" or "goto_next".
        pub action: pulumi_gestalt_rust::Output<String>,
        /// A description of the rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: ["INGRESS",
        /// "EGRESS"]
        pub direction: pulumi_gestalt_rust::Output<Option<String>>,
        /// Denotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured
        /// export destination in Stackdriver.
        pub enable_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        pub match_: pulumi_gestalt_rust::Output<
            super::super::types::compute::OrganizationSecurityPolicyRuleMatch,
        >,
        /// The ID of the OrganizationSecurityPolicy this rule applies to.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// If set to true, the specified action is not enforced.
        pub preview: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An integer indicating the priority of a rule in the list. The priority must be a value
        /// between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the
        /// highest priority and 2147483647 is the lowest prority.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// A list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get
        /// this rule. If this field is left blank, all VMs within the organization will receive the rule.
        pub target_resources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of service accounts indicating the sets of instances that are applied with this rule.
        pub target_service_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationSecurityPolicyRuleArgs,
    ) -> OrganizationSecurityPolicyRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let direction_binding = args.direction.get_output(context);
        let enable_logging_binding = args.enable_logging.get_output(context);
        let match__binding = args.match_.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let preview_binding = args.preview.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let target_resources_binding = args.target_resources.get_output(context);
        let target_service_accounts_binding = args
            .target_service_accounts
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/organizationSecurityPolicyRule:OrganizationSecurityPolicyRule"
                .into(),
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
                    name: "enableLogging".into(),
                    value: enable_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "match".into(),
                    value: match__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preview".into(),
                    value: preview_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResources".into(),
                    value: target_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccounts".into(),
                    value: target_service_accounts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationSecurityPolicyRuleResult {
            action: o.get_field("action"),
            description: o.get_field("description"),
            direction: o.get_field("direction"),
            enable_logging: o.get_field("enableLogging"),
            match_: o.get_field("match"),
            policy_id: o.get_field("policyId"),
            preview: o.get_field("preview"),
            priority: o.get_field("priority"),
            target_resources: o.get_field("targetResources"),
            target_service_accounts: o.get_field("targetServiceAccounts"),
        }
    }
}
