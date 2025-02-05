pub mod get_security_group_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityGroupRuleArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::vpc::GetSecurityGroupRuleFilter>>,
        >,
        /// ID of the security group rule to select.
        #[builder(into, default)]
        pub security_group_rule_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSecurityGroupRuleArgs,
    ) -> GetSecurityGroupRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let security_group_rule_id_binding = args
            .security_group_rule_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpc/getSecurityGroupRule:getSecurityGroupRule".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecurityGroupRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cidr_ipv4: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrIpv4"),
            ),
            cidr_ipv6: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrIpv6"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            from_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fromPort"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            is_egress: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isEgress"),
            ),
            prefix_list_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("prefixListId"),
            ),
            referenced_security_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("referencedSecurityGroupId"),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupId"),
            ),
            security_group_rule_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupRuleId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            to_port: pulumi_wasm_rust::__private::into_domain(o.extract_field("toPort")),
        }
    }
}
