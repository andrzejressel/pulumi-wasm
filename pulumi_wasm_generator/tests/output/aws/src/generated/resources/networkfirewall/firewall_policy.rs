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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod firewall_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// A friendly description of the firewall policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkfirewall::FirewallPolicyEncryptionConfiguration,
            >,
        >,
        /// A configuration block describing the rule groups and policy actions to use in the firewall policy. See Firewall Policy below for details.
        #[builder(into)]
        pub firewall_policy: pulumi_wasm_rust::Output<
            super::super::types::networkfirewall::FirewallPolicyFirewallPolicy,
        >,
        /// A friendly name of the firewall policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// The Amazon Resource Name (ARN) that identifies the firewall policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A friendly description of the firewall policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkfirewall::FirewallPolicyEncryptionConfiguration,
            >,
        >,
        /// A configuration block describing the rule groups and policy actions to use in the firewall policy. See Firewall Policy below for details.
        pub firewall_policy: pulumi_wasm_rust::Output<
            super::super::types::networkfirewall::FirewallPolicyFirewallPolicy,
        >,
        /// A friendly name of the firewall policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A string token used when updating a firewall policy.
        pub update_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FirewallPolicyArgs) -> FirewallPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let firewall_policy_binding = args.firewall_policy.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkfirewall/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            firewall_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateToken").unwrap(),
            ),
        }
    }
}
