/// Provides an Traffic mirror filter rule.
/// Read [limits and considerations](https://docs.aws.amazon.com/vpc/latest/mirroring/traffic-mirroring-considerations.html) for traffic mirroring
///
/// ## Example Usage
///
/// To create a basic traffic mirror session
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_mirror_filter_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficMirrorFilterRuleArgs {
        /// Description of the traffic mirror filter rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Destination CIDR block to assign to the Traffic Mirror rule.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Destination port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        #[builder(into, default)]
        pub destination_port_range: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleDestinationPortRange>,
        >,
        /// Protocol number, for example 17 (UDP), to assign to the Traffic Mirror rule. For information about the protocol value, see [Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml) on the Internet Assigned Numbers Authority (IANA) website.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Action to take (accept | reject) on the filtered traffic. Valid values are `accept` and `reject`
        #[builder(into)]
        pub rule_action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.
        #[builder(into)]
        pub rule_number: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Source CIDR block to assign to the Traffic Mirror rule.
        #[builder(into)]
        pub source_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Source port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        #[builder(into, default)]
        pub source_port_range: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleSourcePortRange>,
        >,
        /// Direction of traffic to be captured. Valid values are `ingress` and `egress`
        ///
        /// Traffic mirror port range support following attributes:
        #[builder(into)]
        pub traffic_direction: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the traffic mirror filter to which this rule should be added
        #[builder(into)]
        pub traffic_mirror_filter_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrafficMirrorFilterRuleResult {
        /// ARN of the traffic mirror filter rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the traffic mirror filter rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Destination CIDR block to assign to the Traffic Mirror rule.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Destination port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        pub destination_port_range: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleDestinationPortRange>,
        >,
        /// Protocol number, for example 17 (UDP), to assign to the Traffic Mirror rule. For information about the protocol value, see [Protocol Numbers](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml) on the Internet Assigned Numbers Authority (IANA) website.
        pub protocol: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Action to take (accept | reject) on the filtered traffic. Valid values are `accept` and `reject`
        pub rule_action: pulumi_gestalt_rust::Output<String>,
        /// Number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.
        pub rule_number: pulumi_gestalt_rust::Output<i32>,
        /// Source CIDR block to assign to the Traffic Mirror rule.
        pub source_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Source port range. Supported only when the protocol is set to TCP(6) or UDP(17). See Traffic mirror port range documented below
        pub source_port_range: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::TrafficMirrorFilterRuleSourcePortRange>,
        >,
        /// Direction of traffic to be captured. Valid values are `ingress` and `egress`
        ///
        /// Traffic mirror port range support following attributes:
        pub traffic_direction: pulumi_gestalt_rust::Output<String>,
        /// ID of the traffic mirror filter to which this rule should be added
        pub traffic_mirror_filter_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficMirrorFilterRuleArgs,
    ) -> TrafficMirrorFilterRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context);
        let destination_port_range_binding = args
            .destination_port_range
            .get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let rule_action_binding = args.rule_action.get_output(context);
        let rule_number_binding = args.rule_number.get_output(context);
        let source_cidr_block_binding = args.source_cidr_block.get_output(context);
        let source_port_range_binding = args.source_port_range.get_output(context);
        let traffic_direction_binding = args.traffic_direction.get_output(context);
        let traffic_mirror_filter_id_binding = args
            .traffic_mirror_filter_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/trafficMirrorFilterRule:TrafficMirrorFilterRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationPortRange".into(),
                    value: &destination_port_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleAction".into(),
                    value: &rule_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleNumber".into(),
                    value: &rule_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCidrBlock".into(),
                    value: &source_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourcePortRange".into(),
                    value: &source_port_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficDirection".into(),
                    value: &traffic_direction_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficMirrorFilterId".into(),
                    value: &traffic_mirror_filter_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficMirrorFilterRuleResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            destination_port_range: o.get_field("destinationPortRange"),
            protocol: o.get_field("protocol"),
            rule_action: o.get_field("ruleAction"),
            rule_number: o.get_field("ruleNumber"),
            source_cidr_block: o.get_field("sourceCidrBlock"),
            source_port_range: o.get_field("sourcePortRange"),
            traffic_direction: o.get_field("trafficDirection"),
            traffic_mirror_filter_id: o.get_field("trafficMirrorFilterId"),
        }
    }
}
