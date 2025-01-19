pub mod get_vpc_dhcp_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcDhcpOptionsArgs {
        /// EC2 DHCP Options ID.
        #[builder(into, default)]
        pub dhcp_options_id: pulumi_wasm_rust::Output<Option<String>>,
        /// List of custom filters as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcDhcpOptionsFilter>>,
        >,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcDhcpOptionsResult {
        /// ARN of the DHCP Options Set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// EC2 DHCP Options ID
        pub dhcp_options_id: pulumi_wasm_rust::Output<String>,
        /// Suffix domain name to used when resolving non Fully Qualified Domain NamesE.g., the `search` value in the `/etc/resolv.conf` file.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// List of name servers.
        pub domain_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcDhcpOptionsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal.
        pub ipv6_address_preferred_lease_time: pulumi_wasm_rust::Output<String>,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// NetBIOS node type (1, 2, 4, or 8). For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_wasm_rust::Output<String>,
        /// List of NTP servers.
        pub ntp_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcDhcpOptionsArgs) -> GetVpcDhcpOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dhcp_options_id_binding = args.dhcp_options_id.get_inner();
        let filters_binding = args.filters.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dhcpOptionsId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainNameServers".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AddressPreferredLeaseTime".into(),
                },
                register_interface::ResultField {
                    name: "netbiosNameServers".into(),
                },
                register_interface::ResultField {
                    name: "netbiosNodeType".into(),
                },
                register_interface::ResultField {
                    name: "ntpServers".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcDhcpOptionsResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            dhcp_options_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dhcpOptionsId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNameServers").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv6_address_preferred_lease_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AddressPreferredLeaseTime").unwrap(),
            ),
            netbios_name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netbiosNameServers").unwrap(),
            ),
            netbios_node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netbiosNodeType").unwrap(),
            ),
            ntp_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ntpServers").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
