/// Manages an inbound (ingress) rule for a security group.
///
/// When specifying an inbound rule for your security group in a VPC, the configuration must include a source for the traffic.
///
/// > **NOTE:** Using `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources is the current best practice. Avoid using the `aws.ec2.SecurityGroupRule` resource and the `ingress` and `egress` arguments of the `aws.ec2.SecurityGroup` resource for configuring in-line rules, as they struggle with managing multiple CIDR blocks, and tags and descriptions due to the historical lack of unique IDs.
///
/// !> **WARNING:** You should not use the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources in conjunction with the `aws.ec2.SecurityGroup` resource with _in-line rules_ (using the `ingress` and `egress` arguments of `aws.ec2.SecurityGroup`) or the `aws.ec2.SecurityGroupRule` resource. Doing so may cause rule conflicts, perpetual differences, and result in rules being overwritten.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:SecurityGroup
///     properties:
///       name: example
///       description: example
///       vpcId: ${main.id}
///       tags:
///         Name: example
///   exampleSecurityGroupIngressRule:
///     type: aws:vpc:SecurityGroupIngressRule
///     name: example
///     properties:
///       securityGroupId: ${example.id}
///       cidrIpv4: 10.0.0.0/8
///       fromPort: 80
///       ipProtocol: tcp
///       toPort: 80
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import security group ingress rules using the `security_group_rule_id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpc/securityGroupIngressRule:SecurityGroupIngressRule example sgr-02108b27edd666983
/// ```
pub mod security_group_ingress_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupIngressRuleArgs {
        /// The source IPv4 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv4: pulumi_wasm_rust::Output<Option<String>>,
        /// The source IPv6 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv6: pulumi_wasm_rust::Output<Option<String>>,
        /// The security group rule description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        #[builder(into, default)]
        pub from_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        #[builder(into)]
        pub ip_protocol: pulumi_wasm_rust::Output<String>,
        /// The ID of the source prefix list.
        #[builder(into, default)]
        pub prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The source security group that is referenced in the rule.
        #[builder(into, default)]
        pub referenced_security_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        #[builder(into, default)]
        pub to_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupIngressRuleResult {
        /// The Amazon Resource Name (ARN) of the security group rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The source IPv4 CIDR range.
        pub cidr_ipv4: pulumi_wasm_rust::Output<Option<String>>,
        /// The source IPv6 CIDR range.
        pub cidr_ipv6: pulumi_wasm_rust::Output<Option<String>>,
        /// The security group rule description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        pub from_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        pub ip_protocol: pulumi_wasm_rust::Output<String>,
        /// The ID of the source prefix list.
        pub prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The source security group that is referenced in the rule.
        pub referenced_security_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the security group.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group rule.
        pub security_group_rule_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        pub to_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SecurityGroupIngressRuleArgs,
    ) -> SecurityGroupIngressRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_ipv4_binding = args.cidr_ipv4.get_inner();
        let cidr_ipv6_binding = args.cidr_ipv6.get_inner();
        let description_binding = args.description.get_inner();
        let from_port_binding = args.from_port.get_inner();
        let ip_protocol_binding = args.ip_protocol.get_inner();
        let prefix_list_id_binding = args.prefix_list_id.get_inner();
        let referenced_security_group_id_binding = args
            .referenced_security_group_id
            .get_inner();
        let security_group_id_binding = args.security_group_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let to_port_binding = args.to_port.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/securityGroupIngressRule:SecurityGroupIngressRule".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cidrIpv4".into(),
                },
                register_interface::ResultField {
                    name: "cidrIpv6".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fromPort".into(),
                },
                register_interface::ResultField {
                    name: "ipProtocol".into(),
                },
                register_interface::ResultField {
                    name: "prefixListId".into(),
                },
                register_interface::ResultField {
                    name: "referencedSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupRuleId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "toPort".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityGroupIngressRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cidr_ipv4: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrIpv4").unwrap(),
            ),
            cidr_ipv6: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrIpv6").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            from_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromPort").unwrap(),
            ),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipProtocol").unwrap(),
            ),
            prefix_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixListId").unwrap(),
            ),
            referenced_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("referencedSecurityGroupId").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            security_group_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupRuleId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            to_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("toPort").unwrap(),
            ),
        }
    }
}