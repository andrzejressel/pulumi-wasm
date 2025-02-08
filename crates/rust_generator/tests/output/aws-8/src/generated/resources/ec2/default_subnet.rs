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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_subnet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultSubnetArgs {
        #[builder(into, default)]
        pub assign_ipv6_address_on_creation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// is required
        /// * The `availability_zone_id`, `cidr_block` and `vpc_id` arguments become computed attributes
        /// * The default value for `map_public_ip_on_launch` is `true`
        ///
        /// This resource supports the following additional arguments:
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub enable_dns64: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub enable_resource_name_dns_a_record_on_launch: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether destroying the resource deletes the default subnet. Default: `false`
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub ipv6_native: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub map_customer_owned_ip_on_launch: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub map_public_ip_on_launch: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub private_dns_hostname_type_on_launch: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultSubnetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub assign_ipv6_address_on_creation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// is required
        /// * The `availability_zone_id`, `cidr_block` and `vpc_id` arguments become computed attributes
        /// * The default value for `map_public_ip_on_launch` is `true`
        ///
        /// This resource supports the following additional arguments:
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The AZ ID of the subnet
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 CIDR block assigned to the subnet
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<Option<String>>,
        pub enable_dns64: pulumi_gestalt_rust::Output<Option<bool>>,
        pub enable_lni_at_device_index: pulumi_gestalt_rust::Output<i32>,
        pub enable_resource_name_dns_a_record_on_launch: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        pub existing_default_subnet: pulumi_gestalt_rust::Output<bool>,
        /// Whether destroying the resource deletes the default subnet. Default: `false`
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        pub ipv6_cidr_block_association_id: pulumi_gestalt_rust::Output<String>,
        pub ipv6_native: pulumi_gestalt_rust::Output<Option<bool>>,
        pub map_customer_owned_ip_on_launch: pulumi_gestalt_rust::Output<Option<bool>>,
        pub map_public_ip_on_launch: pulumi_gestalt_rust::Output<Option<bool>>,
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub private_dns_hostname_type_on_launch: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC the subnet is in
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DefaultSubnetArgs,
    ) -> DefaultSubnetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let assign_ipv6_address_on_creation_binding = args
            .assign_ipv6_address_on_creation
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let customer_owned_ipv4_pool_binding = args
            .customer_owned_ipv4_pool
            .get_output(context)
            .get_inner();
        let enable_dns64_binding = args.enable_dns64.get_output(context).get_inner();
        let enable_resource_name_dns_a_record_on_launch_binding = args
            .enable_resource_name_dns_a_record_on_launch
            .get_output(context)
            .get_inner();
        let enable_resource_name_dns_aaaa_record_on_launch_binding = args
            .enable_resource_name_dns_aaaa_record_on_launch
            .get_output(context)
            .get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
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
        let private_dns_hostname_type_on_launch_binding = args
            .private_dns_hostname_type_on_launch
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/defaultSubnet:DefaultSubnet".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefaultSubnetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            assign_ipv6_address_on_creation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignIpv6AddressOnCreation"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            availability_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrBlock"),
            ),
            customer_owned_ipv4_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerOwnedIpv4Pool"),
            ),
            enable_dns64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDns64"),
            ),
            enable_lni_at_device_index: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableLniAtDeviceIndex"),
            ),
            enable_resource_name_dns_a_record_on_launch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableResourceNameDnsARecordOnLaunch"),
            ),
            enable_resource_name_dns_aaaa_record_on_launch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableResourceNameDnsAaaaRecordOnLaunch"),
            ),
            existing_default_subnet: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("existingDefaultSubnet"),
            ),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            ipv6_cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlock"),
            ),
            ipv6_cidr_block_association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlockAssociationId"),
            ),
            ipv6_native: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Native"),
            ),
            map_customer_owned_ip_on_launch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapCustomerOwnedIpOnLaunch"),
            ),
            map_public_ip_on_launch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapPublicIpOnLaunch"),
            ),
            outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            private_dns_hostname_type_on_launch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsHostnameTypeOnLaunch"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
