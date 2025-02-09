/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import IoT topic rule destinations using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iot/topicRuleDestination:TopicRuleDestination example arn:aws:iot:us-west-2:123456789012:ruledestination/vpc/2ce781c8-68a6-4c52-9c62-63fe489ecc60
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic_rule_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicRuleDestinationArgs {
        /// Whether or not to enable the destination. Default: `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        #[builder(into)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicRuleDestinationResult {
        /// The ARN of the topic rule destination
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to enable the destination. Default: `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TopicRuleDestinationArgs,
    ) -> TopicRuleDestinationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let vpc_configuration_binding_1 = args.vpc_configuration.get_output(context);
        let vpc_configuration_binding = vpc_configuration_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/topicRuleDestination:TopicRuleDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicRuleDestinationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            vpc_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfiguration"),
            ),
        }
    }
}
