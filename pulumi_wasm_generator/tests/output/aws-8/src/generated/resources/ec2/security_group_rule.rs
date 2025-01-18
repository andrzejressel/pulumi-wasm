/// Provides a security group rule resource. Represents a single `ingress` or `egress` group rule, which can be added to external Security Groups.
///
/// > **NOTE:** Avoid using the `aws.ec2.SecurityGroupRule` resource, as it struggles with managing multiple CIDR blocks, and, due to the historical lack of unique IDs, tags and descriptions. To avoid these problems, use the current best practice of the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources with one CIDR block per rule.
///
/// !> **WARNING:** You should not use the `aws.ec2.SecurityGroupRule` resource in conjunction with `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources or with an `aws.ec2.SecurityGroup` resource that has in-line rules. Doing so may cause rule conflicts, perpetual differences, and result in rules being overwritten.
///
/// > **NOTE:** Setting `protocol = "all"` or `protocol = -1` with `from_port` and `to_port` will result in the EC2 API creating a security group rule with all ports open. This API behavior cannot be controlled by this provider and may generate warnings in the future.
///
/// > **NOTE:** Referencing Security Groups across VPC peering has certain restrictions. More information is available in the [VPC Peering User Guide](https://docs.aws.amazon.com/vpc/latest/peering/vpc-peering-security-groups.html).
///
/// ## Example Usage
///
/// Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group_rule::create(
///         "example",
///         SecurityGroupRuleArgs::builder()
///             .cidr_blocks(vec!["${exampleAwsVpc.cidrBlock}",])
///             .from_port(0)
///             .ipv_6_cidr_blocks(vec!["${exampleAwsVpc.ipv6CidrBlock}",])
///             .protocol("tcp")
///             .security_group_id("sg-123456")
///             .to_port(65535)
///             .type_("ingress")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage With Prefix List IDs
///
/// Prefix Lists are either managed by AWS internally, or created by the customer using a
/// Managed Prefix List resource. Prefix Lists provided by
/// AWS are associated with a prefix list name, or service name, that is linked to a specific region.
///
/// Prefix list IDs are exported on VPC Endpoints, so you can use this format:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let allowAll = security_group_rule::create(
///         "allowAll",
///         SecurityGroupRuleArgs::builder()
///             .from_port(0)
///             .prefix_list_ids(vec!["${myEndpoint.prefixListId}",])
///             .protocol("-1")
///             .security_group_id("sg-123456")
///             .to_port(0)
///             .type_("egress")
///             .build_struct(),
///     );
///     let myEndpoint = vpc_endpoint::create(
///         "myEndpoint",
///         VpcEndpointArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// You can also find a specific Prefix List using the `aws.ec2.getPrefixList`
/// or `ec2_managed_prefix_list` data sources:
///
/// ```yaml
/// resources:
///   s3GatewayEgress:
///     type: aws:ec2:SecurityGroupRule
///     name: s3_gateway_egress
///     properties:
///       description: S3 Gateway Egress
///       type: egress
///       securityGroupId: sg-123456
///       fromPort: 443
///       toPort: 443
///       protocol: tcp
///       prefixListIds:
///         - ${s3.id}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   s3:
///     fn::invoke:
///       function: aws:ec2:getPrefixList
///       arguments:
///         name: com.amazonaws.${current.name}.s3
/// ```
///
/// ## Import
///
/// Import a rule with various IPv4 and IPv6 source CIDR blocks:
///
/// Import a rule, applicable to all ports, with a protocol other than TCP/UDP/ICMP/ICMPV6/ALL, e.g., Multicast Transport Protocol (MTP), using the IANA protocol number. For example: 92.
///
/// Import a default any/any egress rule to 0.0.0.0/0:
///
/// Import an egress rule with a prefix list ID destination:
///
/// Import a rule applicable to all protocols and ports with a security group source:
///
/// Import a rule that has itself and an IPv6 CIDR block as sources:
///
/// __Using `pulumi import` to import__ Security Group Rules using the `security_group_id`, `type`, `protocol`, `from_port`, `to_port`, and source(s)/destination(s) (such as a `cidr_block`) separated by underscores (`_`). All parts are required. For example:
///
/// __NOTE:__ Not all rule permissions (e.g., not all of a rule's CIDR blocks) need to be imported for this provider to manage rule permissions. However, importing some of a rule's permissions but not others, and then making changes to the rule will result in the creation of an additional rule to capture the updated permissions. Rule permissions that were not imported are left intact in the original rule.
///
/// Import an ingress rule in security group `sg-6e616f6d69` for TCP port 8000 with an IPv4 destination CIDR of `10.0.3.0/24`:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule ingress sg-6e616f6d69_ingress_tcp_8000_8000_10.0.3.0/24
/// ```
/// Import a rule with various IPv4 and IPv6 source CIDR blocks:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule ingress sg-4973616163_ingress_tcp_100_121_10.1.0.0/16_2001:db8::/48_10.2.0.0/16_2002:db8::/48
/// ```
/// Import a rule, applicable to all ports, with a protocol other than TCP/UDP/ICMP/ICMPV6/ALL, e.g., Multicast Transport Protocol (MTP), using the IANA protocol number. For example: 92.
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule ingress sg-6777656e646f6c796e_ingress_92_0_65536_10.0.3.0/24_10.0.4.0/24
/// ```
/// Import a default any/any egress rule to 0.0.0.0/0:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule default_egress sg-6777656e646f6c796e_egress_all_0_0_0.0.0.0/0
/// ```
/// Import an egress rule with a prefix list ID destination:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule egress sg-62726f6479_egress_tcp_8000_8000_pl-6469726b
/// ```
/// Import a rule applicable to all protocols and ports with a security group source:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule ingress_rule sg-7472697374616e_ingress_all_0_65536_sg-6176657279
/// ```
/// Import a rule that has itself and an IPv6 CIDR block as sources:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroupRule:SecurityGroupRule rule_name sg-656c65616e6f72_ingress_tcp_80_80_self_2001:db8::/48
/// ```
pub mod security_group_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupRuleArgs {
        /// List of CIDR blocks. Cannot be specified with `source_security_group_id` or `self`.
        #[builder(into, default)]
        pub cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Start port (or ICMP type number if protocol is "icmp" or "icmpv6").
        #[builder(into)]
        pub from_port: pulumi_wasm_rust::Output<i32>,
        /// List of IPv6 CIDR blocks. Cannot be specified with `source_security_group_id` or `self`.
        #[builder(into, default)]
        pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Prefix List IDs.
        #[builder(into, default)]
        pub prefix_list_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Protocol. If not icmp, icmpv6, tcp, udp, or all use the [protocol number](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml)
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Security group to apply this rule to.
        #[builder(into)]
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// Whether the security group itself will be added as a source to this ingress rule. Cannot be specified with `cidr_blocks`, `ipv6_cidr_blocks`, or `source_security_group_id`.
        #[builder(into, default)]
        pub self_: pulumi_wasm_rust::Output<Option<bool>>,
        /// Security group id to allow access to/from, depending on the `type`. Cannot be specified with `cidr_blocks`, `ipv6_cidr_blocks`, or `self`.
        #[builder(into, default)]
        pub source_security_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// End port (or ICMP code if protocol is "icmp").
        #[builder(into)]
        pub to_port: pulumi_wasm_rust::Output<i32>,
        /// Type of rule being created. Valid options are `ingress` (inbound)
        /// or `egress` (outbound).
        ///
        /// The following arguments are optional:
        ///
        /// > **Note** Although `cidr_blocks`, `ipv6_cidr_blocks`, `prefix_list_ids`, and `source_security_group_id` are all marked as optional, you _must_ provide one of them in order to configure the source of the traffic.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupRuleResult {
        /// List of CIDR blocks. Cannot be specified with `source_security_group_id` or `self`.
        pub cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Start port (or ICMP type number if protocol is "icmp" or "icmpv6").
        pub from_port: pulumi_wasm_rust::Output<i32>,
        /// List of IPv6 CIDR blocks. Cannot be specified with `source_security_group_id` or `self`.
        pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Prefix List IDs.
        pub prefix_list_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Protocol. If not icmp, icmpv6, tcp, udp, or all use the [protocol number](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml)
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Security group to apply this rule to.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// If the `aws.ec2.SecurityGroupRule` resource has a single source or destination then this is the AWS Security Group Rule resource ID. Otherwise it is empty.
        pub security_group_rule_id: pulumi_wasm_rust::Output<String>,
        /// Whether the security group itself will be added as a source to this ingress rule. Cannot be specified with `cidr_blocks`, `ipv6_cidr_blocks`, or `source_security_group_id`.
        pub self_: pulumi_wasm_rust::Output<Option<bool>>,
        /// Security group id to allow access to/from, depending on the `type`. Cannot be specified with `cidr_blocks`, `ipv6_cidr_blocks`, or `self`.
        pub source_security_group_id: pulumi_wasm_rust::Output<String>,
        /// End port (or ICMP code if protocol is "icmp").
        pub to_port: pulumi_wasm_rust::Output<i32>,
        /// Type of rule being created. Valid options are `ingress` (inbound)
        /// or `egress` (outbound).
        ///
        /// The following arguments are optional:
        ///
        /// > **Note** Although `cidr_blocks`, `ipv6_cidr_blocks`, `prefix_list_ids`, and `source_security_group_id` are all marked as optional, you _must_ provide one of them in order to configure the source of the traffic.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SecurityGroupRuleArgs) -> SecurityGroupRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_blocks_binding = args.cidr_blocks.get_inner();
        let description_binding = args.description.get_inner();
        let from_port_binding = args.from_port.get_inner();
        let ipv6_cidr_blocks_binding = args.ipv6_cidr_blocks.get_inner();
        let prefix_list_ids_binding = args.prefix_list_ids.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let security_group_id_binding = args.security_group_id.get_inner();
        let self__binding = args.self_.get_inner();
        let source_security_group_id_binding = args.source_security_group_id.get_inner();
        let to_port_binding = args.to_port.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/securityGroupRule:SecurityGroupRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlocks".into(),
                    value: &cidr_blocks_binding,
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
                    name: "ipv6CidrBlocks".into(),
                    value: &ipv6_cidr_blocks_binding,
                },
                register_interface::ObjectField {
                    name: "prefixListIds".into(),
                    value: &prefix_list_ids_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "self".into(),
                    value: &self__binding,
                },
                register_interface::ObjectField {
                    name: "sourceSecurityGroupId".into(),
                    value: &source_security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "toPort".into(),
                    value: &to_port_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fromPort".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "prefixListIds".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupRuleId".into(),
                },
                register_interface::ResultField {
                    name: "self".into(),
                },
                register_interface::ResultField {
                    name: "sourceSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "toPort".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityGroupRuleResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocks").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            from_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromPort").unwrap(),
            ),
            ipv6_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlocks").unwrap(),
            ),
            prefix_list_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixListIds").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            security_group_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupRuleId").unwrap(),
            ),
            self_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("self").unwrap(),
            ),
            source_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSecurityGroupId").unwrap(),
            ),
            to_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("toPort").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
