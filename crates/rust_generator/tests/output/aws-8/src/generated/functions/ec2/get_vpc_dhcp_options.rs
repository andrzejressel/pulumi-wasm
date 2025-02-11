#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_dhcp_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcDhcpOptionsArgs {
        /// EC2 DHCP Options ID.
        #[builder(into, default)]
        pub dhcp_options_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of custom filters as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcDhcpOptionsFilter>>,
        >,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcDhcpOptionsResult {
        /// ARN of the DHCP Options Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// EC2 DHCP Options ID
        pub dhcp_options_id: pulumi_gestalt_rust::Output<String>,
        /// Suffix domain name to used when resolving non Fully Qualified Domain NamesE.g., the `search` value in the `/etc/resolv.conf` file.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// List of name servers.
        pub domain_name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcDhcpOptionsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal.
        pub ipv6_address_preferred_lease_time: pulumi_gestalt_rust::Output<String>,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// NetBIOS node type (1, 2, 4, or 8). For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_gestalt_rust::Output<String>,
        /// List of NTP servers.
        pub ntp_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcDhcpOptionsArgs,
    ) -> GetVpcDhcpOptionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dhcp_options_id_binding = args.dhcp_options_id.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpcDhcpOptions:getVpcDhcpOptions".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dhcpOptionsId".into(),
                    value: &dhcp_options_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcDhcpOptionsResult {
            arn: o.get_field("arn"),
            dhcp_options_id: o.get_field("dhcpOptionsId"),
            domain_name: o.get_field("domainName"),
            domain_name_servers: o.get_field("domainNameServers"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ipv6_address_preferred_lease_time: o
                .get_field("ipv6AddressPreferredLeaseTime"),
            netbios_name_servers: o.get_field("netbiosNameServers"),
            netbios_node_type: o.get_field("netbiosNodeType"),
            ntp_servers: o.get_field("ntpServers"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
        }
    }
}
