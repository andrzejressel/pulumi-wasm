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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcDhcpOptionsArgs,
    ) -> GetVpcDhcpOptionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dhcp_options_id_binding = args
            .dhcp_options_id
            .get_output(context)
            .get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcDhcpOptions:getVpcDhcpOptions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dhcpOptionsId".into(),
                    value: &dhcp_options_id_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcDhcpOptionsResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            dhcp_options_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dhcpOptionsId"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            domain_name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainNameServers"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipv6_address_preferred_lease_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6AddressPreferredLeaseTime"),
            ),
            netbios_name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netbiosNameServers"),
            ),
            netbios_node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netbiosNodeType"),
            ),
            ntp_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ntpServers"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
