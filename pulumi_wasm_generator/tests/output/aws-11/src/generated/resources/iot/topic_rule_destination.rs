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
pub mod topic_rule_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicRuleDestinationArgs {
        /// Whether or not to enable the destination. Default: `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        #[builder(into)]
        pub vpc_configuration: pulumi_wasm_rust::Output<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicRuleDestinationResult {
        /// The ARN of the topic rule destination
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether or not to enable the destination. Default: `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        pub vpc_configuration: pulumi_wasm_rust::Output<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TopicRuleDestinationArgs,
    ) -> TopicRuleDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let vpc_configuration_binding = args.vpc_configuration.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicRuleDestinationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            vpc_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfiguration").unwrap(),
            ),
        }
    }
}
