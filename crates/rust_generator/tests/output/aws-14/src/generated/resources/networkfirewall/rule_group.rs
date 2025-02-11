/// Provides an AWS Network Firewall Rule Group Resource
///
/// ## Example Usage
///
/// ### Stateful Inspection for denying access to a domain
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       capacity: 100
///       name: example
///       type: STATEFUL
///       ruleGroup:
///         rulesSource:
///           rulesSourceList:
///             generatedRulesType: DENYLIST
///             targetTypes:
///               - HTTP_HOST
///             targets:
///               - test.example.com
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ### Stateful Inspection for permitting packets from a source IP address
///
///
/// ### Stateful Inspection for blocking packets from going to an intended destination
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       capacity: 100
///       name: example
///       type: STATEFUL
///       ruleGroup:
///         rulesSource:
///           statefulRules:
///             - action: DROP
///               header:
///                 destination: 124.1.1.24/32
///                 destinationPort: 53
///                 direction: ANY
///                 protocol: TCP
///                 source: 1.2.3.4/32
///                 sourcePort: 53
///               ruleOptions:
///                 - keyword: sid
///                   settings:
///                     - '1'
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ### Stateful Inspection from rules specifications defined in Suricata flat format
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       capacity: 100
///       name: example
///       type: STATEFUL
///       rules:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: example.rules
///           return: result
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ### Stateful Inspection from rule group specifications using rule variables and Suricata format rules
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       capacity: 100
///       name: example
///       type: STATEFUL
///       ruleGroup:
///         ruleVariables:
///           ipSets:
///             - key: WEBSERVERS_HOSTS
///               ipSet:
///                 definitions:
///                   - 10.0.0.0/16
///                   - 10.0.1.0/24
///                   - 192.168.0.0/16
///             - key: EXTERNAL_HOST
///               ipSet:
///                 definitions:
///                   - 1.2.3.4/32
///           portSets:
///             - key: HTTP_PORTS
///               portSet:
///                 definitions:
///                   - '443'
///                   - '80'
///         rulesSource:
///           rulesString:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: suricata_rules_file
///               return: result
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ### Stateless Inspection with a Custom Action
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       description: Stateless Rate Limiting Rule
///       capacity: 100
///       name: example
///       type: STATELESS
///       ruleGroup:
///         rulesSource:
///           statelessRulesAndCustomActions:
///             customActions:
///               - actionDefinition:
///                   publishMetricAction:
///                     dimensions:
///                       - value: '2'
///                 actionName: ExampleMetricsAction
///             statelessRules:
///               - priority: 1
///                 ruleDefinition:
///                   actions:
///                     - aws:pass
///                     - ExampleMetricsAction
///                   matchAttributes:
///                     sources:
///                       - addressDefinition: 1.2.3.4/32
///                     sourcePorts:
///                       - fromPort: 443
///                         toPort: 443
///                     destinations:
///                       - addressDefinition: 124.1.1.5/32
///                     destinationPorts:
///                       - fromPort: 443
///                         toPort: 443
///                     protocols:
///                       - 6
///                     tcpFlags:
///                       - flags:
///                           - SYN
///                         masks:
///                           - SYN
///                           - ACK
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ### IP Set References to the Rule Group
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:RuleGroup
///     properties:
///       capacity: 100
///       name: example
///       type: STATEFUL
///       ruleGroup:
///         rulesSource:
///           rulesSourceList:
///             generatedRulesType: DENYLIST
///             targetTypes:
///               - HTTP_HOST
///             targets:
///               - test.example.com
///         referenceSets:
///           ipSetReferences:
///             - key: example
///               ipSetReferences:
///                 - referenceArn: ${this.arn}
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall Rule Groups using their `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:networkfirewall/ruleGroup:RuleGroup example arn:aws:network-firewall:us-west-1:123456789012:stateful-rulegroup/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rule_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupArgs {
        /// The maximum number of operating resources that this rule group can use. For a stateless rule group, the capacity required is the sum of the capacity requirements of the individual rules. For a stateful rule group, the minimum capacity required is the number of individual rules.
        #[builder(into)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A friendly description of the rule group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkfirewall::RuleGroupEncryptionConfiguration,
            >,
        >,
        /// A friendly name of the rule group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block that defines the rule group rules. Required unless `rules` is specified. See Rule Group below for details.
        #[builder(into, default)]
        pub rule_group: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkfirewall::RuleGroupRuleGroup>,
        >,
        /// The stateful rule group rules specifications in Suricata file format, with one rule per line. Use this to import your existing Suricata compatible rule groups. Required unless `rule_group` is specified.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether the rule group is stateless (containing stateless rules) or stateful (containing stateful rules). Valid values include: `STATEFUL` or `STATELESS`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RuleGroupResult {
        /// The Amazon Resource Name (ARN) that identifies the rule group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of operating resources that this rule group can use. For a stateless rule group, the capacity required is the sum of the capacity requirements of the individual rules. For a stateful rule group, the minimum capacity required is the number of individual rules.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// A friendly description of the rule group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkfirewall::RuleGroupEncryptionConfiguration,
            >,
        >,
        /// A friendly name of the rule group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that defines the rule group rules. Required unless `rules` is specified. See Rule Group below for details.
        pub rule_group: pulumi_gestalt_rust::Output<
            super::super::types::networkfirewall::RuleGroupRuleGroup,
        >,
        /// The stateful rule group rules specifications in Suricata file format, with one rule per line. Use this to import your existing Suricata compatible rule groups. Required unless `rule_group` is specified.
        pub rules: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the rule group is stateless (containing stateless rules) or stateful (containing stateful rules). Valid values include: `STATEFUL` or `STATELESS`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A string token used when updating the rule group.
        pub update_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RuleGroupArgs,
    ) -> RuleGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_binding = args.capacity.get_output(context);
        let description_binding = args.description.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let rule_group_binding = args.rule_group.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkfirewall/ruleGroup:RuleGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleGroup".into(),
                    value: &rule_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuleGroupResult {
            arn: o.get_field("arn"),
            capacity: o.get_field("capacity"),
            description: o.get_field("description"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            name: o.get_field("name"),
            rule_group: o.get_field("ruleGroup"),
            rules: o.get_field("rules"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            update_token: o.get_field("updateToken"),
        }
    }
}
