/// Creates an entry (a rule) in a network ACL with the specified rule number.
///
/// > **NOTE on Network ACLs and Network ACL Rules:** This provider currently
/// provides both a standalone Network ACL Rule resource and a Network ACL resource with rules
/// defined in-line. At this time you cannot use a Network ACL with in-line rules
/// in conjunction with any Network ACL Rule resources. Doing so will cause
/// a conflict of rule settings and will overwrite rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = network_acl::create(
///         "bar",
///         NetworkAclArgs::builder().vpc_id("${foo.id}").build_struct(),
///     );
///     let barNetworkAclRule = network_acl_rule::create(
///         "barNetworkAclRule",
///         NetworkAclRuleArgs::builder()
///             .cidr_block("${foo.cidrBlock}")
///             .egress(false)
///             .from_port(22)
///             .network_acl_id("${bar.id}")
///             .protocol("tcp")
///             .rule_action("allow")
///             .rule_number(200)
///             .to_port(22)
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** One of either `cidr_block` or `ipv6_cidr_block` is required.
///
/// ## Import
///
/// Using the procotol's decimal value:
///
/// __Using `pulumi import` to import__ individual rules using `NETWORK_ACL_ID:RULE_NUMBER:PROTOCOL:EGRESS`, where `PROTOCOL` can be a decimal (such as "6") or string (such as "tcp") value. For example:
///
/// Using the procotol's string value:
///
/// ```sh
/// $ pulumi import aws:ec2/networkAclRule:NetworkAclRule my_rule acl-7aaabd18:100:tcp:false
/// ```
/// Using the procotol's decimal value:
///
/// ```sh
/// $ pulumi import aws:ec2/networkAclRule:NetworkAclRule my_rule acl-7aaabd18:100:6:false
/// ```
pub mod network_acl_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAclRuleArgs {
        /// The network range to allow or deny, in CIDR notation (for example 172.16.0.0/24 ).
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet). Default `false`.
        #[builder(into, default)]
        pub egress: pulumi_wasm_rust::Output<Option<bool>>,
        /// The from port to match.
        #[builder(into, default)]
        pub from_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// ICMP protocol: The ICMP code. Required if specifying ICMP for the protocolE.g., -1
        ///
        /// > **NOTE:** If the value of `protocol` is `-1` or `all`, the `from_port` and `to_port` values will be ignored and the rule will apply to all ports.
        ///
        /// > **NOTE:** If the value of `icmp_type` is `-1` (which results in a wildcard ICMP type), the `icmp_code` must also be set to `-1` (wildcard ICMP code).
        ///
        /// > Note: For more information on ICMP types and codes, see here: https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
        #[builder(into, default)]
        pub icmp_code: pulumi_wasm_rust::Output<Option<i32>>,
        /// ICMP protocol: The ICMP type. Required if specifying ICMP for the protocolE.g., -1
        #[builder(into, default)]
        pub icmp_type: pulumi_wasm_rust::Output<Option<i32>>,
        /// The IPv6 CIDR block to allow or deny.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the network ACL.
        #[builder(into)]
        pub network_acl_id: pulumi_wasm_rust::Output<String>,
        /// The protocol. A value of -1 means all protocols.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to allow or deny the traffic that matches the rule. Accepted values: `allow` | `deny`
        #[builder(into)]
        pub rule_action: pulumi_wasm_rust::Output<String>,
        /// The rule number for the entry (for example, 100). ACL entries are processed in ascending order by rule number.
        #[builder(into)]
        pub rule_number: pulumi_wasm_rust::Output<i32>,
        /// The to port to match.
        #[builder(into, default)]
        pub to_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct NetworkAclRuleResult {
        /// The network range to allow or deny, in CIDR notation (for example 172.16.0.0/24 ).
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet). Default `false`.
        pub egress: pulumi_wasm_rust::Output<Option<bool>>,
        /// The from port to match.
        pub from_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// ICMP protocol: The ICMP code. Required if specifying ICMP for the protocolE.g., -1
        ///
        /// > **NOTE:** If the value of `protocol` is `-1` or `all`, the `from_port` and `to_port` values will be ignored and the rule will apply to all ports.
        ///
        /// > **NOTE:** If the value of `icmp_type` is `-1` (which results in a wildcard ICMP type), the `icmp_code` must also be set to `-1` (wildcard ICMP code).
        ///
        /// > Note: For more information on ICMP types and codes, see here: https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
        pub icmp_code: pulumi_wasm_rust::Output<Option<i32>>,
        /// ICMP protocol: The ICMP type. Required if specifying ICMP for the protocolE.g., -1
        pub icmp_type: pulumi_wasm_rust::Output<Option<i32>>,
        /// The IPv6 CIDR block to allow or deny.
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the network ACL.
        pub network_acl_id: pulumi_wasm_rust::Output<String>,
        /// The protocol. A value of -1 means all protocols.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to allow or deny the traffic that matches the rule. Accepted values: `allow` | `deny`
        pub rule_action: pulumi_wasm_rust::Output<String>,
        /// The rule number for the entry (for example, 100). ACL entries are processed in ascending order by rule number.
        pub rule_number: pulumi_wasm_rust::Output<i32>,
        /// The to port to match.
        pub to_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkAclRuleArgs) -> NetworkAclRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_block_binding = args.cidr_block.get_inner();
        let egress_binding = args.egress.get_inner();
        let from_port_binding = args.from_port.get_inner();
        let icmp_code_binding = args.icmp_code.get_inner();
        let icmp_type_binding = args.icmp_type.get_inner();
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_inner();
        let network_acl_id_binding = args.network_acl_id.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let rule_action_binding = args.rule_action.get_inner();
        let rule_number_binding = args.rule_number.get_inner();
        let to_port_binding = args.to_port.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkAclRule:NetworkAclRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "egress".into(),
                    value: &egress_binding,
                },
                register_interface::ObjectField {
                    name: "fromPort".into(),
                    value: &from_port_binding,
                },
                register_interface::ObjectField {
                    name: "icmpCode".into(),
                    value: &icmp_code_binding,
                },
                register_interface::ObjectField {
                    name: "icmpType".into(),
                    value: &icmp_type_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "networkAclId".into(),
                    value: &network_acl_id_binding,
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
                    name: "toPort".into(),
                    value: &to_port_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "egress".into(),
                },
                register_interface::ResultField {
                    name: "fromPort".into(),
                },
                register_interface::ResultField {
                    name: "icmpCode".into(),
                },
                register_interface::ResultField {
                    name: "icmpType".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "networkAclId".into(),
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
        NetworkAclRuleResult {
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            egress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egress").unwrap(),
            ),
            from_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromPort").unwrap(),
            ),
            icmp_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("icmpCode").unwrap(),
            ),
            icmp_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("icmpType").unwrap(),
            ),
            ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlock").unwrap(),
            ),
            network_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAclId").unwrap(),
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
            to_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("toPort").unwrap(),
            ),
        }
    }
}