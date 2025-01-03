/// Provides a resource to manage a [default subnet](http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/default-vpc.html#default-vpc-basics) in the current region.
///
/// **This is an advanced resource** and has special caveats to be aware of when using it. Please read this document in its entirety before using this resource.
///
/// The `aws.ec2.DefaultSubnet` resource behaves differently from normal resources in that if a default subnet exists in the specified Availability Zone, this provider does not _create_ this resource, but instead "adopts" it into management.
/// If no default subnet exists, this provider creates a new default subnet.
/// By default, `pulumi destroy` does not delete the default subnet but does remove the resource from the state.
/// Set the `force_destroy` argument to `true` to delete the default subnet.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   defaultAz1:
///     type: aws:ec2:DefaultSubnet
///     name: default_az1
///     properties:
///       availabilityZone: us-west-2a
///       tags:
///         Name: Default subnet for us-west-2a
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import subnets using the subnet `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultSubnet:DefaultSubnet public_subnet subnet-9d4a7b6c
/// ```
pub mod default_subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultSubnetArgs {
        #[builder(into, default)]
        pub assign_ipv6_address_on_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// is required
        /// * The `availability_zone_id`, `cidr_block` and `vpc_id` arguments become computed attributes
        /// * The default value for `map_public_ip_on_launch` is `true`
        ///
        /// This resource supports the following additional arguments:
        #[builder(into)]
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub enable_dns64: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Whether destroying the resource deletes the default subnet. Default: `false`
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub ipv6_native: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub map_customer_owned_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub map_public_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub private_dns_hostname_type_on_launch: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultSubnetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub assign_ipv6_address_on_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// is required
        /// * The `availability_zone_id`, `cidr_block` and `vpc_id` arguments become computed attributes
        /// * The default value for `map_public_ip_on_launch` is `true`
        ///
        /// This resource supports the following additional arguments:
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The AZ ID of the subnet
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR block assigned to the subnet
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        pub enable_dns64: pulumi_wasm_rust::Output<Option<bool>>,
        pub enable_lni_at_device_index: pulumi_wasm_rust::Output<i32>,
        pub enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        pub existing_default_subnet: pulumi_wasm_rust::Output<bool>,
        /// Whether destroying the resource deletes the default subnet. Default: `false`
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<String>,
        pub ipv6_cidr_block_association_id: pulumi_wasm_rust::Output<String>,
        pub ipv6_native: pulumi_wasm_rust::Output<Option<bool>>,
        pub map_customer_owned_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        pub map_public_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub private_dns_hostname_type_on_launch: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC the subnet is in
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DefaultSubnetArgs) -> DefaultSubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assign_ipv6_address_on_creation_binding = args
            .assign_ipv6_address_on_creation
            .get_inner();
        let availability_zone_binding = args.availability_zone.get_inner();
        let customer_owned_ipv4_pool_binding = args.customer_owned_ipv4_pool.get_inner();
        let enable_dns64_binding = args.enable_dns64.get_inner();
        let enable_resource_name_dns_a_record_on_launch_binding = args
            .enable_resource_name_dns_a_record_on_launch
            .get_inner();
        let enable_resource_name_dns_aaaa_record_on_launch_binding = args
            .enable_resource_name_dns_aaaa_record_on_launch
            .get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_inner();
        let ipv6_native_binding = args.ipv6_native.get_inner();
        let map_customer_owned_ip_on_launch_binding = args
            .map_customer_owned_ip_on_launch
            .get_inner();
        let map_public_ip_on_launch_binding = args.map_public_ip_on_launch.get_inner();
        let private_dns_hostname_type_on_launch_binding = args
            .private_dns_hostname_type_on_launch
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/defaultSubnet:DefaultSubnet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignIpv6AddressOnCreation".into(),
                    value: &assign_ipv6_address_on_creation_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: &customer_owned_ipv4_pool_binding,
                },
                register_interface::ObjectField {
                    name: "enableDns64".into(),
                    value: &enable_dns64_binding,
                },
                register_interface::ObjectField {
                    name: "enableResourceNameDnsARecordOnLaunch".into(),
                    value: &enable_resource_name_dns_a_record_on_launch_binding,
                },
                register_interface::ObjectField {
                    name: "enableResourceNameDnsAaaaRecordOnLaunch".into(),
                    value: &enable_resource_name_dns_aaaa_record_on_launch_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Native".into(),
                    value: &ipv6_native_binding,
                },
                register_interface::ObjectField {
                    name: "mapCustomerOwnedIpOnLaunch".into(),
                    value: &map_customer_owned_ip_on_launch_binding,
                },
                register_interface::ObjectField {
                    name: "mapPublicIpOnLaunch".into(),
                    value: &map_public_ip_on_launch_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsHostnameTypeOnLaunch".into(),
                    value: &private_dns_hostname_type_on_launch_binding,
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
                    name: "assignIpv6AddressOnCreation".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
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
                    name: "existingDefaultSubnet".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultSubnetResult {
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
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
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
            existing_default_subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("existingDefaultSubnet").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
