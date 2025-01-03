pub mod get_security_group_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityGroupRuleArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::vpc::GetSecurityGroupRuleFilter>>,
        >,
        /// ID of the security group rule to select.
        #[builder(into, default)]
        pub security_group_rule_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecurityGroupRuleResult {
        /// The Amazon Resource Name (ARN) of the security group rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The destination IPv4 CIDR range.
        pub cidr_ipv4: pulumi_wasm_rust::Output<String>,
        /// The destination IPv6 CIDR range.
        pub cidr_ipv6: pulumi_wasm_rust::Output<String>,
        /// The security group rule description.
        pub description: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::vpc::GetSecurityGroupRuleFilter>>,
        >,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        pub from_port: pulumi_wasm_rust::Output<i32>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The IP protocol name or number. Use `-1` to specify all protocols.
        pub ip_protocol: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the security group rule is an outbound rule.
        pub is_egress: pulumi_wasm_rust::Output<bool>,
        /// The ID of the destination prefix list.
        pub prefix_list_id: pulumi_wasm_rust::Output<String>,
        /// The destination security group that is referenced in the rule.
        pub referenced_security_group_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        pub security_group_rule_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// (Optional) The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        pub to_port: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSecurityGroupRuleArgs) -> GetSecurityGroupRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let security_group_rule_id_binding = args.security_group_rule_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpc/getSecurityGroupRule:getSecurityGroupRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupRuleId".into(),
                    value: &security_group_rule_id_binding,
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "fromPort".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipProtocol".into(),
                },
                register_interface::ResultField {
                    name: "isEgress".into(),
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
                    name: "toPort".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecurityGroupRuleResult {
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
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            from_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromPort").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipProtocol").unwrap(),
            ),
            is_egress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isEgress").unwrap(),
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
            to_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("toPort").unwrap(),
            ),
        }
    }
}
