pub mod get_subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Availability zone where the subnet must reside.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the Availability Zone for the subnet. This argument is not supported in all regions or partitions. If necessary, use `availability_zone` instead.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// CIDR block of the desired subnet.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the desired subnet must be the default subnet for its associated availability zone.
        #[builder(into, default)]
        pub default_for_az: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSubnetFilter>>,
        >,
        /// ID of the specific subnet to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// IPv6 CIDR block of the desired subnet.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// State that the desired subnet must have.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired subnet.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the desired subnet belongs to.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        /// ARN of the subnet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether an IPv6 address is assigned on creation.
        pub assign_ipv6_address_on_creation: pulumi_wasm_rust::Output<bool>,
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// Available IP addresses of the subnet.
        pub available_ip_address_count: pulumi_wasm_rust::Output<i32>,
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// Identifier of customer owned IPv4 address pool.
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<String>,
        pub default_for_az: pulumi_wasm_rust::Output<bool>,
        /// Whether DNS queries made to the Amazon-provided DNS Resolver in this subnet return synthetic IPv6 addresses for IPv4-only destinations.
        pub enable_dns64: pulumi_wasm_rust::Output<bool>,
        /// Indicates the device position for local network interfaces in this subnet. For example, 1 indicates local network interfaces in this subnet are the secondary network interface (eth1). A local network interface cannot be the primary network interface (eth0).
        pub enable_lni_at_device_index: pulumi_wasm_rust::Output<i32>,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records.
        pub enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::Output<bool>,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::Output<
            bool,
        >,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSubnetFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Association ID of the IPv6 CIDR block.
        pub ipv6_cidr_block_association_id: pulumi_wasm_rust::Output<String>,
        /// Whether this is an IPv6-only subnet.
        pub ipv6_native: pulumi_wasm_rust::Output<bool>,
        /// Whether customer owned IP addresses are assigned on network interface creation.
        pub map_customer_owned_ip_on_launch: pulumi_wasm_rust::Output<bool>,
        /// Whether public IP addresses are assigned on instance launch.
        pub map_public_ip_on_launch: pulumi_wasm_rust::Output<bool>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns the subnet.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The type of hostnames assigned to instances in the subnet at launch.
        pub private_dns_hostname_type_on_launch: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSubnetArgs) -> GetSubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args.availability_zone.get_inner();
        let availability_zone_id_binding = args.availability_zone_id.get_inner();
        let cidr_block_binding = args.cidr_block.get_inner();
        let default_for_az_binding = args.default_for_az.get_inner();
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_inner();
        let state_binding = args.state.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getSubnet:getSubnet".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "defaultForAz".into(),
                    value: &default_for_az_binding,
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
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "assignIpv6AddressOnCreation".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "availableIpAddressCount".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "defaultForAz".into(),
                },
                register_interface::ResultField {
                    name: "enableDns64".into(),
                },
                register_interface::ResultField {
                    name: "enableLniAtDeviceIndex".into(),
                },
                register_interface::ResultField {
                    name: "enableResourceNameDnsARecordOnLaunch".into(),
                },
                register_interface::ResultField {
                    name: "enableResourceNameDnsAaaaRecordOnLaunch".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlockAssociationId".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Native".into(),
                },
                register_interface::ResultField {
                    name: "mapCustomerOwnedIpOnLaunch".into(),
                },
                register_interface::ResultField {
                    name: "mapPublicIpOnLaunch".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsHostnameTypeOnLaunch".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSubnetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            assign_ipv6_address_on_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignIpv6AddressOnCreation").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            available_ip_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableIpAddressCount").unwrap(),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
            ),
            default_for_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultForAz").unwrap(),
            ),
            enable_dns64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDns64").unwrap(),
            ),
            enable_lni_at_device_index: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableLniAtDeviceIndex").unwrap(),
            ),
            enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableResourceNameDnsARecordOnLaunch").unwrap(),
            ),
            enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableResourceNameDnsAaaaRecordOnLaunch").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlock").unwrap(),
            ),
            ipv6_cidr_block_association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlockAssociationId").unwrap(),
            ),
            ipv6_native: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Native").unwrap(),
            ),
            map_customer_owned_ip_on_launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapCustomerOwnedIpOnLaunch").unwrap(),
            ),
            map_public_ip_on_launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapPublicIpOnLaunch").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            private_dns_hostname_type_on_launch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsHostnameTypeOnLaunch").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
