/// Provides an AWS Network Firewall Resource Policy Resource for a rule group or firewall policy.
///
/// ## Example Usage
///
/// ### For a Firewall Policy resource
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:ResourcePolicy
///     properties:
///       resourceArn: ${exampleAwsNetworkfirewallFirewallPolicy.arn}
///       policy:
///         fn::toJSON:
///           Statement:
///             - Action:
///                 - network-firewall:ListFirewallPolicies
///                 - network-firewall:CreateFirewall
///                 - network-firewall:UpdateFirewall
///                 - network-firewall:AssociateFirewallPolicy
///               Effect: Allow
///               Resource: ${exampleAwsNetworkfirewallFirewallPolicy.arn}
///               Principal:
///                 AWS: arn:aws:iam::123456789012:root
///           Version: 2012-10-17
/// ```
///
/// ### For a Rule Group resource
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:ResourcePolicy
///     properties:
///       resourceArn: ${exampleAwsNetworkfirewallRuleGroup.arn}
///       policy:
///         fn::toJSON:
///           Statement:
///             - Action:
///                 - network-firewall:ListRuleGroups
///                 - network-firewall:CreateFirewallPolicy
///                 - network-firewall:UpdateFirewallPolicy
///               Effect: Allow
///               Resource: ${exampleAwsNetworkfirewallRuleGroup.arn}
///               Principal:
///                 AWS: arn:aws:iam::123456789012:root
///           Version: 2012-10-17
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall Resource Policies using the `resource arn`. For example:
/// ```sh
/// $ pulumi import aws:networkfirewall/resourcePolicy:ResourcePolicy example arn:aws:network-firewall:us-west-1:123456789012:stateful-rulegroup/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// JSON formatted policy document that controls access to the Network Firewall resource. The policy must be provided **without whitespaces**.  We recommend using jsonencode for formatting as seen in the examples above. For more details, including available policy statement Actions, see the [Policy](https://docs.aws.amazon.com/network-firewall/latest/APIReference/API_PutResourcePolicy.html#API_PutResourcePolicy_RequestSyntax) parameter in the AWS API documentation.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the rule group or firewall policy.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// JSON formatted policy document that controls access to the Network Firewall resource. The policy must be provided **without whitespaces**.  We recommend using jsonencode for formatting as seen in the examples above. For more details, including available policy statement Actions, see the [Policy](https://docs.aws.amazon.com/network-firewall/latest/APIReference/API_PutResourcePolicy.html#API_PutResourcePolicy_RequestSyntax) parameter in the AWS API documentation.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the rule group or firewall policy.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let resource_arn_binding_1 = args.resource_arn.get_output(context);
        let resource_arn_binding = resource_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkfirewall/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourcePolicyResult {
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
