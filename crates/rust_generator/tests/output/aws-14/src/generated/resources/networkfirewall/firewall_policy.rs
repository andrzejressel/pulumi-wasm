/// Provides an AWS Network Firewall Firewall Policy Resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:FirewallPolicy
///     properties:
///       name: example
///       firewallPolicy:
///         statelessDefaultActions:
///           - aws:pass
///         statelessFragmentDefaultActions:
///           - aws:drop
///         statelessRuleGroupReferences:
///           - priority: 1
///             resourceArn: ${exampleAwsNetworkfirewallRuleGroup.arn}
///         tlsInspectionConfigurationArn: arn:aws:network-firewall:REGION:ACCT:tls-configuration/example
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Policy with a HOME_NET Override
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:FirewallPolicy
///     properties:
///       name: example
///       firewallPolicy:
///         policyVariables:
///           ruleVariables:
///             - key: HOME_NET
///               ipSet:
///                 definitions:
///                   - 10.0.0.0/16
///                   - 10.1.0.0/24
///         statelessDefaultActions:
///           - aws:pass
///         statelessFragmentDefaultActions:
///           - aws:drop
///         statelessRuleGroupReferences:
///           - priority: 1
///             resourceArn: ${exampleAwsNetworkfirewallRuleGroup.arn}
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Policy with a Custom Action for Stateless Inspection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = firewall_policy::create(
///         "test",
///         FirewallPolicyArgs::builder()
///             .firewall_policy(
///                 FirewallPolicyFirewallPolicy::builder()
///                     .statelessCustomActions(
///                         vec![
///                             FirewallPolicyFirewallPolicyStatelessCustomAction::builder()
///                             .actionDefinition(FirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition::builder()
///                             .publishMetricAction(FirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricAction::builder()
///                             .dimensions(vec![FirewallPolicyFirewallPolicyStatelessCustomActionActionDefinitionPublishMetricActionDimension::builder()
///                             .value("1").build_struct(),]).build_struct()).build_struct())
///                             .actionName("ExampleCustomAction").build_struct(),
///                         ],
///                     )
///                     .statelessDefaultActions(vec!["aws:pass", "ExampleCustomAction",])
///                     .statelessFragmentDefaultActions(vec!["aws:drop",])
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall Policies using their `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:networkfirewall/firewallPolicy:FirewallPolicy example arn:aws:network-firewall:us-west-1:123456789012:firewall-policy/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// A friendly description of the firewall policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkfirewall::FirewallPolicyEncryptionConfiguration,
            >,
        >,
        /// A configuration block describing the rule groups and policy actions to use in the firewall policy. See Firewall Policy below for details.
        #[builder(into)]
        pub firewall_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkfirewall::FirewallPolicyFirewallPolicy,
        >,
        /// A friendly name of the firewall policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// The Amazon Resource Name (ARN) that identifies the firewall policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A friendly description of the firewall policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkfirewall::FirewallPolicyEncryptionConfiguration,
            >,
        >,
        /// A configuration block describing the rule groups and policy actions to use in the firewall policy. See Firewall Policy below for details.
        pub firewall_policy: pulumi_gestalt_rust::Output<
            super::super::types::networkfirewall::FirewallPolicyFirewallPolicy,
        >,
        /// A friendly name of the firewall policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A string token used when updating a firewall policy.
        pub update_token: pulumi_gestalt_rust::Output<String>,
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
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let firewall_policy_binding = args.firewall_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkfirewall/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallPolicyResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            firewall_policy: o.get_field("firewallPolicy"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_token: o.get_field("updateToken"),
        }
    }
}
