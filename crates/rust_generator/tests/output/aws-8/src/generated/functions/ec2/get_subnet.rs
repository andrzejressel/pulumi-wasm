#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subnet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Availability zone where the subnet must reside.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the Availability Zone for the subnet. This argument is not supported in all regions or partitions. If necessary, use `availability_zone` instead.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CIDR block of the desired subnet.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the desired subnet must be the default subnet for its associated availability zone.
        #[builder(into, default)]
        pub default_for_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetSubnetFilter>>,
        >,
        /// ID of the specific subnet to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IPv6 CIDR block of the desired subnet.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// State that the desired subnet must have.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired subnet.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the desired subnet belongs to.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        /// ARN of the subnet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether an IPv6 address is assigned on creation.
        pub assign_ipv6_address_on_creation: pulumi_gestalt_rust::Output<bool>,
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Available IP addresses of the subnet.
        pub available_ip_address_count: pulumi_gestalt_rust::Output<i32>,
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Identifier of customer owned IPv4 address pool.
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<String>,
        pub default_for_az: pulumi_gestalt_rust::Output<bool>,
        /// Whether DNS queries made to the Amazon-provided DNS Resolver in this subnet return synthetic IPv6 addresses for IPv4-only destinations.
        pub enable_dns64: pulumi_gestalt_rust::Output<bool>,
        /// Indicates the device position for local network interfaces in this subnet. For example, 1 indicates local network interfaces in this subnet are the secondary network interface (eth1). A local network interface cannot be the primary network interface (eth0).
        pub enable_lni_at_device_index: pulumi_gestalt_rust::Output<i32>,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS A records.
        pub enable_resource_name_dns_a_record_on_launch: pulumi_gestalt_rust::Output<
            bool,
        >,
        /// Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.
        pub enable_resource_name_dns_aaaa_record_on_launch: pulumi_gestalt_rust::Output<
            bool,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSubnetFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Association ID of the IPv6 CIDR block.
        pub ipv6_cidr_block_association_id: pulumi_gestalt_rust::Output<String>,
        /// Whether this is an IPv6-only subnet.
        pub ipv6_native: pulumi_gestalt_rust::Output<bool>,
        /// Whether customer owned IP addresses are assigned on network interface creation.
        pub map_customer_owned_ip_on_launch: pulumi_gestalt_rust::Output<bool>,
        /// Whether public IP addresses are assigned on instance launch.
        pub map_public_ip_on_launch: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the subnet.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The type of hostnames assigned to instances in the subnet at launch.
        pub private_dns_hostname_type_on_launch: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubnetArgs,
    ) -> GetSubnetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let availability_zone_id_binding = args.availability_zone_id.get_output(context);
        let cidr_block_binding = args.cidr_block.get_output(context);
        let default_for_az_binding = args.default_for_az.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getSubnet:getSubnet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZoneId".into(),
                    value: availability_zone_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultForAz".into(),
                    value: default_for_az_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: ipv6_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubnetResult {
            arn: o.get_field("arn"),
            assign_ipv6_address_on_creation: o.get_field("assignIpv6AddressOnCreation"),
            availability_zone: o.get_field("availabilityZone"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            available_ip_address_count: o.get_field("availableIpAddressCount"),
            cidr_block: o.get_field("cidrBlock"),
            customer_owned_ipv4_pool: o.get_field("customerOwnedIpv4Pool"),
            default_for_az: o.get_field("defaultForAz"),
            enable_dns64: o.get_field("enableDns64"),
            enable_lni_at_device_index: o.get_field("enableLniAtDeviceIndex"),
            enable_resource_name_dns_a_record_on_launch: o
                .get_field("enableResourceNameDnsARecordOnLaunch"),
            enable_resource_name_dns_aaaa_record_on_launch: o
                .get_field("enableResourceNameDnsAaaaRecordOnLaunch"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ipv6_cidr_block: o.get_field("ipv6CidrBlock"),
            ipv6_cidr_block_association_id: o.get_field("ipv6CidrBlockAssociationId"),
            ipv6_native: o.get_field("ipv6Native"),
            map_customer_owned_ip_on_launch: o.get_field("mapCustomerOwnedIpOnLaunch"),
            map_public_ip_on_launch: o.get_field("mapPublicIpOnLaunch"),
            outpost_arn: o.get_field("outpostArn"),
            owner_id: o.get_field("ownerId"),
            private_dns_hostname_type_on_launch: o
                .get_field("privateDnsHostnameTypeOnLaunch"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
