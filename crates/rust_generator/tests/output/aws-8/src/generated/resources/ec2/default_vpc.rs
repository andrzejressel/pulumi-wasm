/// Provides a resource to manage the [default AWS VPC](http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/default-vpc.html)
/// in the current AWS Region.
///
/// If you created your AWS account after 2013-12-04 you have a default VPC in each AWS Region.
///
/// **This is an advanced resource** and has special caveats to be aware of when using it. Please read this document in its entirety before using this resource.
///
/// The `aws.ec2.DefaultVpc` resource behaves differently from normal resources in that if a default VPC exists, this provider does not _create_ this resource, but instead "adopts" it into management.
/// If no default VPC exists, the provider creates a new default VPC, which leads to the implicit creation of [other resources](https://docs.aws.amazon.com/vpc/latest/userguide/default-vpc.html#default-vpc-components).
/// By default, `pulumi destroy` does not delete the default VPC but does remove the resource from the state.
/// Set the `force_destroy` argument to `true` to delete the default VPC.
///
/// ## Example Usage
///
/// Basic usage with tags:
///
/// ```yaml
/// resources:
///   default:
///     type: aws:ec2:DefaultVpc
///     properties:
///       tags:
///         Name: Default VPC
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Default VPCs using the VPC `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultVpc:DefaultVpc default vpc-a01106c2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_vpc {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultVpcArgs {
        #[builder(into, default)]
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub enable_dns_hostnames: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub enable_dns_support: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub enable_network_address_usage_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether destroying the resource deletes the default VPC. Default: `false`
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub ipv6_cidr_block_network_border_group: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub ipv6_netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultVpcResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The primary IPv4 CIDR block for the VPC
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        pub default_network_acl_id: pulumi_gestalt_rust::Output<String>,
        pub default_route_table_id: pulumi_gestalt_rust::Output<String>,
        pub default_security_group_id: pulumi_gestalt_rust::Output<String>,
        pub dhcp_options_id: pulumi_gestalt_rust::Output<String>,
        pub enable_dns_hostnames: pulumi_gestalt_rust::Output<Option<bool>>,
        pub enable_dns_support: pulumi_gestalt_rust::Output<Option<bool>>,
        pub enable_network_address_usage_metrics: pulumi_gestalt_rust::Output<bool>,
        pub existing_default_vpc: pulumi_gestalt_rust::Output<bool>,
        /// Whether destroying the resource deletes the default VPC. Default: `false`
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The allowed tenancy of instances launched into the VPC
        pub instance_tenancy: pulumi_gestalt_rust::Output<String>,
        pub ipv6_association_id: pulumi_gestalt_rust::Output<String>,
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        pub ipv6_cidr_block_network_border_group: pulumi_gestalt_rust::Output<String>,
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub ipv6_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        pub main_route_table_id: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultVpcArgs,
    ) -> DefaultVpcResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assign_generated_ipv6_cidr_block_binding = args
            .assign_generated_ipv6_cidr_block
            .get_output(context);
        let enable_dns_hostnames_binding = args.enable_dns_hostnames.get_output(context);
        let enable_dns_support_binding = args.enable_dns_support.get_output(context);
        let enable_network_address_usage_metrics_binding = args
            .enable_network_address_usage_metrics
            .get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_output(context);
        let ipv6_cidr_block_network_border_group_binding = args
            .ipv6_cidr_block_network_border_group
            .get_output(context);
        let ipv6_ipam_pool_id_binding = args.ipv6_ipam_pool_id.get_output(context);
        let ipv6_netmask_length_binding = args.ipv6_netmask_length.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/defaultVpc:DefaultVpc".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignGeneratedIpv6CidrBlock".into(),
                    value: assign_generated_ipv6_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDnsHostnames".into(),
                    value: enable_dns_hostnames_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDnsSupport".into(),
                    value: enable_dns_support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableNetworkAddressUsageMetrics".into(),
                    value: enable_network_address_usage_metrics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: ipv6_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlockNetworkBorderGroup".into(),
                    value: ipv6_cidr_block_network_border_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6IpamPoolId".into(),
                    value: ipv6_ipam_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6NetmaskLength".into(),
                    value: ipv6_netmask_length_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultVpcResult {
            arn: o.get_field("arn"),
            assign_generated_ipv6_cidr_block: o
                .get_field("assignGeneratedIpv6CidrBlock"),
            cidr_block: o.get_field("cidrBlock"),
            default_network_acl_id: o.get_field("defaultNetworkAclId"),
            default_route_table_id: o.get_field("defaultRouteTableId"),
            default_security_group_id: o.get_field("defaultSecurityGroupId"),
            dhcp_options_id: o.get_field("dhcpOptionsId"),
            enable_dns_hostnames: o.get_field("enableDnsHostnames"),
            enable_dns_support: o.get_field("enableDnsSupport"),
            enable_network_address_usage_metrics: o
                .get_field("enableNetworkAddressUsageMetrics"),
            existing_default_vpc: o.get_field("existingDefaultVpc"),
            force_destroy: o.get_field("forceDestroy"),
            instance_tenancy: o.get_field("instanceTenancy"),
            ipv6_association_id: o.get_field("ipv6AssociationId"),
            ipv6_cidr_block: o.get_field("ipv6CidrBlock"),
            ipv6_cidr_block_network_border_group: o
                .get_field("ipv6CidrBlockNetworkBorderGroup"),
            ipv6_ipam_pool_id: o.get_field("ipv6IpamPoolId"),
            ipv6_netmask_length: o.get_field("ipv6NetmaskLength"),
            main_route_table_id: o.get_field("mainRouteTableId"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
