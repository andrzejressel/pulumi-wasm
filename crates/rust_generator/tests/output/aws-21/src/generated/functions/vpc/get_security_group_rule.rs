#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_security_group_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityGroupRuleArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::vpc::GetSecurityGroupRuleFilter>>,
        >,
        /// ID of the security group rule to select.
        #[builder(into, default)]
        pub security_group_rule_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecurityGroupRuleResult {
        /// The Amazon Resource Name (ARN) of the security group rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The destination IPv4 CIDR range.
        pub cidr_ipv4: pulumi_gestalt_rust::Output<String>,
        /// The destination IPv6 CIDR range.
        pub cidr_ipv6: pulumi_gestalt_rust::Output<String>,
        /// The security group rule description.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::vpc::GetSecurityGroupRuleFilter>>,
        >,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        pub from_port: pulumi_gestalt_rust::Output<i32>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The IP protocol name or number. Use `-1` to specify all protocols.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the security group rule is an outbound rule.
        pub is_egress: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the destination prefix list.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
        /// The destination security group that is referenced in the rule.
        pub referenced_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        pub security_group_rule_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// (Optional) The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        pub to_port: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecurityGroupRuleArgs,
    ) -> GetSecurityGroupRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let security_group_rule_id_binding = args
            .security_group_rule_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:vpc/getSecurityGroupRule:getSecurityGroupRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupRuleId".into(),
                    value: security_group_rule_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecurityGroupRuleResult {
            arn: o.get_field("arn"),
            cidr_ipv4: o.get_field("cidrIpv4"),
            cidr_ipv6: o.get_field("cidrIpv6"),
            description: o.get_field("description"),
            filters: o.get_field("filters"),
            from_port: o.get_field("fromPort"),
            id: o.get_field("id"),
            ip_protocol: o.get_field("ipProtocol"),
            is_egress: o.get_field("isEgress"),
            prefix_list_id: o.get_field("prefixListId"),
            referenced_security_group_id: o.get_field("referencedSecurityGroupId"),
            security_group_id: o.get_field("securityGroupId"),
            security_group_rule_id: o.get_field("securityGroupRuleId"),
            tags: o.get_field("tags"),
            to_port: o.get_field("toPort"),
        }
    }
}
