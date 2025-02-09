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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcIpamPoolCidrAllocationArgs,
    ) -> VpcIpamPoolCidrAllocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cidr_binding_1 = args.cidr.get_output(context);
        let cidr_binding = cidr_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let disallowed_cidrs_binding_1 = args.disallowed_cidrs.get_output(context);
        let disallowed_cidrs_binding = disallowed_cidrs_binding_1.get_inner();
        let ipam_pool_id_binding_1 = args.ipam_pool_id.get_output(context);
        let ipam_pool_id_binding = ipam_pool_id_binding_1.get_inner();
        let netmask_length_binding_1 = args.netmask_length.get_output(context);
        let netmask_length_binding = netmask_length_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPoolCidrAllocation:VpcIpamPoolCidrAllocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disallowedCidrs".into(),
                    value: &disallowed_cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "netmaskLength".into(),
                    value: &netmask_length_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamPoolCidrAllocationResult {
            cidr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cidr")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disallowed_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disallowedCidrs"),
            ),
            ipam_pool_allocation_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolAllocationId"),
            ),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
            netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netmaskLength"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceOwner"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
        }
    }
}
