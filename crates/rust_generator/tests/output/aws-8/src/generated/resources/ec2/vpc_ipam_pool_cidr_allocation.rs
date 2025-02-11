/// Allocates (reserves) a CIDR from an IPAM address pool, preventing usage by IPAM. Only works for private IPv4.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamPoolCidrAllocation
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/24
///     options:
///       dependsOn:
///         - ${exampleVpcIpamPoolCidr}
///   exampleVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: example
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${exampleVpcIpam.privateDefaultScopeId}
///       locale: ${current.name}
///   exampleVpcIpam:
///     type: aws:ec2:VpcIpam
///     name: example
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// With the `disallowed_cidrs` attribute:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamPoolCidrAllocation
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       netmaskLength: 28
///       disallowedCidrs:
///         - 172.20.0.0/28
///     options:
///       dependsOn:
///         - ${exampleVpcIpamPoolCidr}
///   exampleVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: example
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${exampleVpcIpam.privateDefaultScopeId}
///       locale: ${current.name}
///   exampleVpcIpam:
///     type: aws:ec2:VpcIpam
///     name: example
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAM allocations using the allocation `id` and `pool id`, separated by `_`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamPoolCidrAllocation:VpcIpamPoolCidrAllocation example ipam-pool-alloc-0dc6d196509c049ba8b549ff99f639736_ipam-pool-07cfb559e0921fcbe
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_pool_cidr_allocation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrAllocationArgs {
        /// The CIDR you want to assign to the pool.
        #[builder(into, default)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the allocation.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The netmask length of the CIDR you would like to allocate to the IPAM pool. Valid Values: `0-128`.
        #[builder(into, default)]
        pub netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrAllocationResult {
        /// The CIDR you want to assign to the pool.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// The description for the allocation.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Exclude a particular CIDR range from being returned by the pool.
        pub disallowed_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub ipam_pool_allocation_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The netmask length of the CIDR you would like to allocate to the IPAM pool. Valid Values: `0-128`.
        pub netmask_length: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the resource.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The owner of the resource.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// The type of the resource.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamPoolCidrAllocationArgs,
    ) -> VpcIpamPoolCidrAllocationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_binding = args.cidr.get_output(context);
        let description_binding = args.description.get_output(context);
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let netmask_length_binding = args.netmask_length.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPoolCidrAllocation:VpcIpamPoolCidrAllocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disallowedCidrs".into(),
                    value: &disallowed_cidrs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netmaskLength".into(),
                    value: &netmask_length_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamPoolCidrAllocationResult {
            cidr: o.get_field("cidr"),
            description: o.get_field("description"),
            disallowed_cidrs: o.get_field("disallowedCidrs"),
            ipam_pool_allocation_id: o.get_field("ipamPoolAllocationId"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            netmask_length: o.get_field("netmaskLength"),
            resource_id: o.get_field("resourceId"),
            resource_owner: o.get_field("resourceOwner"),
            resource_type: o.get_field("resourceType"),
        }
    }
}
