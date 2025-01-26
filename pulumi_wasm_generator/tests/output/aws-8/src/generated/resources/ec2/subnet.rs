/// Provides an VPC subnet resource.
///
/// > **NOTE:** Due to [AWS Lambda improved VPC networking changes that began deploying in September 2019](https://aws.amazon.com/blogs/compute/announcing-improved-vpc-networking-for-aws-lambda-functions/), subnets associated with Lambda Functions can take up to 45 minutes to successfully delete. To allow for successful deletion, the provider will wait for at least 45 minutes even if a shorter delete timeout is specified.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${mainAwsVpc.id}
///       cidrBlock: 10.0.1.0/24
///       tags:
///         Name: Main
/// ```
///
/// ### Subnets In Secondary VPC CIDR Blocks
///
/// When managing subnets in one of a VPC's secondary CIDR blocks created using a `aws.ec2.VpcIpv4CidrBlockAssociation`
/// resource, it is recommended to reference that resource's `vpc_id` attribute to ensure correct dependency ordering.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let inSecondaryCidr = subnet::create(
///         "inSecondaryCidr",
///         SubnetArgs::builder()
///             .cidr_block("172.20.0.0/24")
///             .vpc_id("${secondaryCidr.vpcId}")
///             .build_struct(),
///     );
///     let secondaryCidr = vpc_ipv_4_cidr_block_association::create(
///         "secondaryCidr",
///         VpcIpv4CidrBlockAssociationArgs::builder()
///             .cidr_block("172.20.0.0/16")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import subnets using the subnet `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/subnet:Subnet public_subnet subnet-9d4a7b6c
/// ```
pub mod subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetArgs {
        /// Specify true to indicate
        /// that network interfaces created in the specified subnet should be
        /// assigned an IPv6 address. Default is `false`
        #[builder(into, default)]
        pub assign_ipv6_address_on_creation: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// AZ for the subnet.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// AZ ID of the subnet. This argument is not supported in all regions or partitions. If necessary, use `availability_zone` instead.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IPv4 CIDR block for the subnet.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The customer owned IPv4 address pool. Typically used with the `map_customer_owned_ip_on_launch` argument. The `outpost_arn` argument must be specified when configured.
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations. Default: `false`.
        #[builder(into, default)]
        pub enable_dns64: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Indicates the device position for local network interfaces in this subnet. For example, 1 indicates local network interfaces in this subnet are the secondary network interface (eth1). A local network interface cannot be the primary network interface (eth0).
        #[builder(into, default)]
        pub enable_lni_at_device_index: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records. Default: `false`.
        #[builder(into, default)]
        pub enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records. Default: `false`.
        #[builder(into, default)]
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The IPv6 network range for the subnet,
        /// in CIDR notation. The subnet size must use a /64 prefix length.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to create an IPv6-only subnet. Default: `false`.
        #[builder(into, default)]
        pub ipv6_native: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specify `true` to indicate that network interfaces created in the subnet should be assigned a customer owned IP address. The `customer_owned_ipv4_pool` and `outpost_arn` arguments must be specified when set to `true`. Default is `false`.
        #[builder(into, default)]
        pub map_customer_owned_ip_on_launch: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specify true to indicate
        /// that instances launched into the subnet should be assigned
        /// a public IP address. Default is `false`.
        #[builder(into, default)]
        pub map_public_ip_on_launch: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the Outpost.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of hostnames to assign to instances in the subnet at launch. For IPv6-only subnets, an instance DNS name must be based on the instance ID. For dual-stack and IPv4-only subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID. Valid values: `ip-name`, `resource-name`.
        #[builder(into, default)]
        pub private_dns_hostname_type_on_launch: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC ID.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetResult {
        /// The ARN of the subnet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specify true to indicate
        /// that network interfaces created in the specified subnet should be
        /// assigned an IPv6 address. Default is `false`
        pub assign_ipv6_address_on_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// AZ for the subnet.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// AZ ID of the subnet. This argument is not supported in all regions or partitions. If necessary, use `availability_zone` instead.
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR block for the subnet.
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer owned IPv4 address pool. Typically used with the `map_customer_owned_ip_on_launch` argument. The `outpost_arn` argument must be specified when configured.
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations. Default: `false`.
        pub enable_dns64: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates the device position for local network interfaces in this subnet. For example, 1 indicates local network interfaces in this subnet are the secondary network interface (eth1). A local network interface cannot be the primary network interface (eth0).
        pub enable_lni_at_device_index: pulumi_wasm_rust::Output<Option<i32>>,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records. Default: `false`.
        pub enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records. Default: `false`.
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The IPv6 network range for the subnet,
        /// in CIDR notation. The subnet size must use a /64 prefix length.
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The association ID for the IPv6 CIDR block.
        pub ipv6_cidr_block_association_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to create an IPv6-only subnet. Default: `false`.
        pub ipv6_native: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specify `true` to indicate that network interfaces created in the subnet should be assigned a customer owned IP address. The `customer_owned_ipv4_pool` and `outpost_arn` arguments must be specified when set to `true`. Default is `false`.
        pub map_customer_owned_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specify true to indicate
        /// that instances launched into the subnet should be assigned
        /// a public IP address. Default is `false`.
        pub map_public_ip_on_launch: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the Outpost.
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the subnet.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The type of hostnames to assign to instances in the subnet at launch. For IPv6-only subnets, an instance DNS name must be based on the instance ID. For dual-stack and IPv4-only subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID. Valid values: `ip-name`, `resource-name`.
        pub private_dns_hostname_type_on_launch: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubnetArgs,
    ) -> SubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assign_ipv6_address_on_creation_binding = args
            .assign_ipv6_address_on_creation
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let availability_zone_id_binding = args
            .availability_zone_id
            .get_output(context)
            .get_inner();
        let cidr_block_binding = args.cidr_block.get_output(context).get_inner();
        let customer_owned_ipv4_pool_binding = args
            .customer_owned_ipv4_pool
            .get_output(context)
            .get_inner();
        let enable_dns64_binding = args.enable_dns64.get_output(context).get_inner();
        let enable_lni_at_device_index_binding = args
            .enable_lni_at_device_index
            .get_output(context)
            .get_inner();
        let enable_resource_name_dns_a_record_on_launch_binding = args
            .enable_resource_name_dns_a_record_on_launch
            .get_output(context)
            .get_inner();
        let enable_resource_name_dns_aaaa_record_on_launch_binding = args
            .enable_resource_name_dns_aaaa_record_on_launch
            .get_output(context)
            .get_inner();
        let ipv6_cidr_block_binding = args
            .ipv6_cidr_block
            .get_output(context)
            .get_inner();
        let ipv6_native_binding = args.ipv6_native.get_output(context).get_inner();
        let map_customer_owned_ip_on_launch_binding = args
            .map_customer_owned_ip_on_launch
            .get_output(context)
            .get_inner();
        let map_public_ip_on_launch_binding = args
            .map_public_ip_on_launch
            .get_output(context)
            .get_inner();
        let outpost_arn_binding = args.outpost_arn.get_output(context).get_inner();
        let private_dns_hostname_type_on_launch_binding = args
            .private_dns_hostname_type_on_launch
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/subnet:Subnet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
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
                    name: "enableLniAtDeviceIndex".into(),
                    value: &enable_lni_at_device_index_binding,
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
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsHostnameTypeOnLaunch".into(),
                    value: &private_dns_hostname_type_on_launch_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            assign_ipv6_address_on_creation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assignIpv6AddressOnCreation"),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrBlock"),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerOwnedIpv4Pool"),
            ),
            enable_dns64: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableDns64"),
            ),
            enable_lni_at_device_index: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableLniAtDeviceIndex"),
            ),
            enable_resource_name_dns_a_record_on_launch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableResourceNameDnsARecordOnLaunch"),
            ),
            enable_resource_name_dns_aaaa_record_on_launch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableResourceNameDnsAaaaRecordOnLaunch"),
            ),
            ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlock"),
            ),
            ipv6_cidr_block_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlockAssociationId"),
            ),
            ipv6_native: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6Native"),
            ),
            map_customer_owned_ip_on_launch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mapCustomerOwnedIpOnLaunch"),
            ),
            map_public_ip_on_launch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mapPublicIpOnLaunch"),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            private_dns_hostname_type_on_launch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDnsHostnameTypeOnLaunch"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
