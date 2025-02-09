/// Manages an outbound (egress) rule for a security group.
///
/// When specifying an outbound rule for your security group in a VPC, the configuration must include a destination for the traffic.
///
/// > **NOTE:** Using `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources is the current best practice. Avoid using the `aws.ec2.SecurityGroupRule` resource and the `ingress` and `egress` arguments of the `aws.ec2.SecurityGroup` resource for configuring in-line rules, as they struggle with managing multiple CIDR blocks, and tags and descriptions due to the historical lack of unique IDs.
///
/// !> **WARNING:** You should not use the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources in conjunction with the `aws.ec2.SecurityGroup` resource with _in-line rules_ (using the `ingress` and `egress` arguments of `aws.ec2.SecurityGroup`) or the `aws.ec2.SecurityGroupRule` resource. Doing so may cause rule conflicts, perpetual differences, and result in rules being overwritten.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group_egress_rule::create(
///         "example",
///         SecurityGroupEgressRuleArgs::builder()
///             .cidr_ipv_4("10.0.0.0/8")
///             .from_port(80)
///             .ip_protocol("tcp")
///             .security_group_id("${exampleAwsSecurityGroup.id}")
///             .to_port(80)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import security group egress rules using the `security_group_rule_id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpc/securityGroupEgressRule:SecurityGroupEgressRule example sgr-02108b27edd666983
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_group_egress_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupEgressRuleArgs {
        /// The destination IPv4 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv4: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination IPv6 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security group rule description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        #[builder(into, default)]
        pub from_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        #[builder(into)]
        pub ip_protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the destination prefix list.
        #[builder(into, default)]
        pub prefix_list_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination security group that is referenced in the rule.
        #[builder(into, default)]
        pub referenced_security_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        #[builder(into, default)]
        pub to_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupEgressRuleResult {
        /// The Amazon Resource Name (ARN) of the security group rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The destination IPv4 CIDR range.
        pub cidr_ipv4: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination IPv6 CIDR range.
        pub cidr_ipv6: pulumi_gestalt_rust::Output<Option<String>>,
        /// The security group rule description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        pub from_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// The ID of the destination prefix list.
        pub prefix_list_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination security group that is referenced in the rule.
        pub referenced_security_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the security group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group rule.
        pub security_group_rule_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        pub to_port: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SecurityGroupEgressRuleArgs,
    ) -> SecurityGroupEgressRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cidr_ipv4_binding_1 = args.cidr_ipv4.get_output(context);
        let cidr_ipv4_binding = cidr_ipv4_binding_1.get_inner();
        let cidr_ipv6_binding_1 = args.cidr_ipv6.get_output(context);
        let cidr_ipv6_binding = cidr_ipv6_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let from_port_binding_1 = args.from_port.get_output(context);
        let from_port_binding = from_port_binding_1.get_inner();
        let ip_protocol_binding_1 = args.ip_protocol.get_output(context);
        let ip_protocol_binding = ip_protocol_binding_1.get_inner();
        let prefix_list_id_binding_1 = args.prefix_list_id.get_output(context);
        let prefix_list_id_binding = prefix_list_id_binding_1.get_inner();
        let referenced_security_group_id_binding_1 = args
            .referenced_security_group_id
            .get_output(context);
        let referenced_security_group_id_binding = referenced_security_group_id_binding_1
            .get_inner();
        let security_group_id_binding_1 = args.security_group_id.get_output(context);
        let security_group_id_binding = security_group_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let to_port_binding_1 = args.to_port.get_output(context);
        let to_port_binding = to_port_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/securityGroupEgressRule:SecurityGroupEgressRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrIpv4".into(),
                    value: &cidr_ipv4_binding,
                },
                register_interface::ObjectField {
                    name: "cidrIpv6".into(),
                    value: &cidr_ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fromPort".into(),
                    value: &from_port_binding,
                },
                register_interface::ObjectField {
                    name: "ipProtocol".into(),
                    value: &ip_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding,
                },
                register_interface::ObjectField {
                    name: "referencedSecurityGroupId".into(),
                    value: &referenced_security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "toPort".into(),
                    value: &to_port_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecurityGroupEgressRuleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cidr_ipv4: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrIpv4"),
            ),
            cidr_ipv6: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrIpv6"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            from_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fromPort"),
            ),
            ip_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            prefix_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefixListId"),
            ),
            referenced_security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("referencedSecurityGroupId"),
            ),
            security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupId"),
            ),
            security_group_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupRuleId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            to_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("toPort"),
            ),
        }
    }
}
