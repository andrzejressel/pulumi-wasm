pub mod get_vpc {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcArgs {
        /// Cidr block of the desired VPC.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean constraint on whether the desired VPC is
        /// the default VPC for the region.
        #[builder(into, default)]
        pub default: pulumi_wasm_rust::Output<Option<bool>>,
        /// DHCP options id of the desired VPC.
        #[builder(into, default)]
        pub dhcp_options_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcFilter>>,
        >,
        /// ID of the specific VPC to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Current state of the desired VPC.
        /// Can be either `"pending"` or `"available"`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPC.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcResult {
        /// ARN of VPC
        pub arn: pulumi_wasm_rust::Output<String>,
        /// CIDR block for the association.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        pub cidr_block_associations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcCidrBlockAssociation>,
        >,
        pub default: pulumi_wasm_rust::Output<bool>,
        pub dhcp_options_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not the VPC has DNS hostname support
        pub enable_dns_hostnames: pulumi_wasm_rust::Output<bool>,
        /// Whether or not the VPC has DNS support
        pub enable_dns_support: pulumi_wasm_rust::Output<bool>,
        /// Whether Network Address Usage metrics are enabled for your VPC
        pub enable_network_address_usage_metrics: pulumi_wasm_rust::Output<bool>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Allowed tenancy of instances launched into the
        /// selected VPC. May be any of `"default"`, `"dedicated"`, or `"host"`.
        pub instance_tenancy: pulumi_wasm_rust::Output<String>,
        /// Association ID for the IPv6 CIDR block.
        pub ipv6_association_id: pulumi_wasm_rust::Output<String>,
        /// IPv6 CIDR block.
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<String>,
        /// ID of the main route table associated with this VPC.
        pub main_route_table_id: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns the VPC.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// State of the association.
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcArgs) -> GetVpcResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_block_binding = args.cidr_block.get_inner();
        let default_binding = args.default.get_inner();
        let dhcp_options_id_binding = args.dhcp_options_id.get_inner();
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let state_binding = args.state.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpc:getVpc".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "default".into(),
                    value: &default_binding,
                },
                register_interface::ObjectField {
                    name: "dhcpOptionsId".into(),
                    value: &dhcp_options_id_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
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
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlockAssociations".into(),
                },
                register_interface::ResultField {
                    name: "default".into(),
                },
                register_interface::ResultField {
                    name: "dhcpOptionsId".into(),
                },
                register_interface::ResultField {
                    name: "enableDnsHostnames".into(),
                },
                register_interface::ResultField {
                    name: "enableDnsSupport".into(),
                },
                register_interface::ResultField {
                    name: "enableNetworkAddressUsageMetrics".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceTenancy".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AssociationId".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "mainRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        GetVpcResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            cidr_block_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlockAssociations").unwrap(),
            ),
            default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("default").unwrap(),
            ),
            dhcp_options_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dhcpOptionsId").unwrap(),
            ),
            enable_dns_hostnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDnsHostnames").unwrap(),
            ),
            enable_dns_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDnsSupport").unwrap(),
            ),
            enable_network_address_usage_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableNetworkAddressUsageMetrics").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTenancy").unwrap(),
            ),
            ipv6_association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AssociationId").unwrap(),
            ),
            ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlock").unwrap(),
            ),
            main_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mainRouteTableId").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
