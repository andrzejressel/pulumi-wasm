/// Provides an Traffic mirror filter rule.
/// Read [limits and considerations](https://docs.aws.amazon.com/vpc/latest/mirroring/traffic-mirroring-considerations.html) for traffic mirroring
///
/// ## Example Usage
///
/// To create a basic traffic mirror session
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let filter = traffic_mirror_filter::create(
///         "filter",
///         TrafficMirrorFilterArgs::builder()
///             .description("traffic mirror filter - example")
///             .network_services(vec!["amazon-dns",])
///             .build_struct(),
///     );
///     let rulein = traffic_mirror_filter_rule::create(
///         "rulein",
///         TrafficMirrorFilterRuleArgs::builder()
///             .description("test rule")
///             .destination_cidr_block("10.0.0.0/8")
///             .destination_port_range(
///                 TrafficMirrorFilterRuleDestinationPortRange::builder()
///                     .fromPort(22)
///                     .toPort(53)
///                     .build_struct(),
///             )
///             .protocol(6)
///             .rule_action("accept")
///             .rule_number(1)
///             .source_cidr_block("10.0.0.0/8")
///             .source_port_range(
///                 TrafficMirrorFilterRuleSourcePortRange::builder()
///                     .fromPort(0)
///                     .toPort(10)
///                     .build_struct(),
///             )
///             .traffic_direction("ingress")
///             .traffic_mirror_filter_id("${filter.id}")
///             .build_struct(),
///     );
///     let ruleout = traffic_mirror_filter_rule::create(
///         "ruleout",
///         TrafficMirrorFilterRuleArgs::builder()
///             .description("test rule")
///             .destination_cidr_block("10.0.0.0/8")
///             .rule_action("accept")
///             .rule_number(1)
///             .source_cidr_block("10.0.0.0/8")
///             .traffic_direction("egress")
///             .traffic_mirror_filter_id("${filter.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import traffic mirror rules using the `traffic_mirror_filter_id` and `id` separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/trafficMirrorFilterRule:TrafficMirrorFilterRule rule tmf-0fbb93ddf38198f64:tmfr-05a458f06445d0aee
/// ```
pub mod traffic_mirror_filter_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficMirrorFilterRuleArgs {
        /// Description of the traffic mirror filter rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination CIDR block to assign to the Traffic Mirror rule.
        #[builder(into)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Destination port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        #[builder(into, default)]
        pub destination_port_range: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleDestinationPortRange>,
        >,
        /// Protocol number, for example 17 (UDP), to assign to the Traffic Mirror rule. For information about the protocol value, see [Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml) on the Internet Assigned Numbers Authority (IANA) website.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<i32>>,
        /// Action to take (accept | reject) on the filtered traffic. Valid values are `accept` and `reject`
        #[builder(into)]
        pub rule_action: pulumi_wasm_rust::Output<String>,
        /// Number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.
        #[builder(into)]
        pub rule_number: pulumi_wasm_rust::Output<i32>,
        /// Source CIDR block to assign to the Traffic Mirror rule.
        #[builder(into)]
        pub source_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Source port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        #[builder(into, default)]
        pub source_port_range: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleSourcePortRange>,
        >,
        /// Direction of traffic to be captured. Valid values are `ingress` and `egress`
        ///
        /// Traffic mirror port range support following attributes:
        #[builder(into)]
        pub traffic_direction: pulumi_wasm_rust::Output<String>,
        /// ID of the traffic mirror filter to which this rule should be added
        #[builder(into)]
        pub traffic_mirror_filter_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TrafficMirrorFilterRuleResult {
        /// ARN of the traffic mirror filter rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the traffic mirror filter rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination CIDR block to assign to the Traffic Mirror rule.
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Destination port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        pub destination_port_range: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleDestinationPortRange>,
        >,
        /// Protocol number, for example 17 (UDP), to assign to the Traffic Mirror rule. For information about the protocol value, see [Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml) on the Internet Assigned Numbers Authority (IANA) website.
        pub protocol: pulumi_wasm_rust::Output<Option<i32>>,
        /// Action to take (accept | reject) on the filtered traffic. Valid values are `accept` and `reject`
        pub rule_action: pulumi_wasm_rust::Output<String>,
        /// Number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.
        pub rule_number: pulumi_wasm_rust::Output<i32>,
        /// Source CIDR block to assign to the Traffic Mirror rule.
        pub source_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Source port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        pub source_port_range: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleSourcePortRange>,
        >,
        /// Direction of traffic to be captured. Valid values are `ingress` and `egress`
        ///
        /// Traffic mirror port range support following attributes:
        pub traffic_direction: pulumi_wasm_rust::Output<String>,
        /// ID of the traffic mirror filter to which this rule should be added
        pub traffic_mirror_filter_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TrafficMirrorFilterRuleArgs,
    ) -> TrafficMirrorFilterRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let destination_cidr_block_binding = args.destination_cidr_block.get_inner();
        let destination_port_range_binding = args.destination_port_range.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let rule_action_binding = args.rule_action.get_inner();
        let rule_number_binding = args.rule_number.get_inner();
        let source_cidr_block_binding = args.source_cidr_block.get_inner();
        let source_port_range_binding = args.source_port_range.get_inner();
        let traffic_direction_binding = args.traffic_direction.get_inner();
        let traffic_mirror_filter_id_binding = args.traffic_mirror_filter_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/trafficMirrorFilterRule:TrafficMirrorFilterRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPortRange".into(),
                    value: &destination_port_range_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "ruleAction".into(),
                    value: &rule_action_binding,
                },
                register_interface::ObjectField {
                    name: "ruleNumber".into(),
                    value: &rule_number_binding,
                },
                register_interface::ObjectField {
                    name: "sourceCidrBlock".into(),
                    value: &source_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePortRange".into(),
                    value: &source_port_range_binding,
                },
                register_interface::ObjectField {
                    name: "trafficDirection".into(),
                    value: &traffic_direction_binding,
                },
                register_interface::ObjectField {
                    name: "trafficMirrorFilterId".into(),
                    value: &traffic_mirror_filter_id_binding,
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
                    name: "destinationCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "destinationPortRange".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "ruleAction".into(),
                },
                register_interface::ResultField {
                    name: "ruleNumber".into(),
                },
                register_interface::ResultField {
                    name: "sourceCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "sourcePortRange".into(),
                },
                register_interface::ResultField {
                    name: "trafficDirection".into(),
                },
                register_interface::ResultField {
                    name: "trafficMirrorFilterId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrafficMirrorFilterRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationCidrBlock").unwrap(),
            ),
            destination_port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPortRange").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            rule_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleAction").unwrap(),
            ),
            rule_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleNumber").unwrap(),
            ),
            source_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCidrBlock").unwrap(),
            ),
            source_port_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePortRange").unwrap(),
            ),
            traffic_direction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficDirection").unwrap(),
            ),
            traffic_mirror_filter_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficMirrorFilterId").unwrap(),
            ),
        }
    }
}