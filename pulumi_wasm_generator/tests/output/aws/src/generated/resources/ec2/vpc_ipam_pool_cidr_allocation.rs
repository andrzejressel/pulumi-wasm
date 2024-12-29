/// Allocates (reserves) a CIDR from an IPAM address pool, preventing usage by IPAM. Only works for private IPv4.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let example = vpc_ipam_pool_cidr_allocation::create(
///         "example",
///         VpcIpamPoolCidrAllocationArgs::builder()
///             .cidr("172.20.0.0/24")
///             .ipam_pool_id("${exampleVpcIpamPool.id}")
///             .build_struct(),
///     );
///     let exampleVpcIpam = vpc_ipam::create(
///         "exampleVpcIpam",
///         VpcIpamArgs::builder()
///             .operating_regions(
///                 vec![
///                     VpcIpamOperatingRegion::builder().regionName("${current.name}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVpcIpamPool = vpc_ipam_pool::create(
///         "exampleVpcIpamPool",
///         VpcIpamPoolArgs::builder()
///             .address_family("ipv4")
///             .ipam_scope_id("${exampleVpcIpam.privateDefaultScopeId}")
///             .locale("${current.name}")
///             .build_struct(),
///     );
///     let exampleVpcIpamPoolCidr = vpc_ipam_pool_cidr::create(
///         "exampleVpcIpamPoolCidr",
///         VpcIpamPoolCidrArgs::builder()
///             .cidr("172.20.0.0/16")
///             .ipam_pool_id("${exampleVpcIpamPool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With the `disallowed_cidrs` attribute:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let example = vpc_ipam_pool_cidr_allocation::create(
///         "example",
///         VpcIpamPoolCidrAllocationArgs::builder()
///             .disallowed_cidrs(vec!["172.20.0.0/28",])
///             .ipam_pool_id("${exampleVpcIpamPool.id}")
///             .netmask_length(28)
///             .build_struct(),
///     );
///     let exampleVpcIpam = vpc_ipam::create(
///         "exampleVpcIpam",
///         VpcIpamArgs::builder()
///             .operating_regions(
///                 vec![
///                     VpcIpamOperatingRegion::builder().regionName("${current.name}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVpcIpamPool = vpc_ipam_pool::create(
///         "exampleVpcIpamPool",
///         VpcIpamPoolArgs::builder()
///             .address_family("ipv4")
///             .ipam_scope_id("${exampleVpcIpam.privateDefaultScopeId}")
///             .locale("${current.name}")
///             .build_struct(),
///     );
///     let exampleVpcIpamPoolCidr = vpc_ipam_pool_cidr::create(
///         "exampleVpcIpamPoolCidr",
///         VpcIpamPoolCidrArgs::builder()
///             .cidr("172.20.0.0/16")
///             .ipam_pool_id("${exampleVpcIpamPool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAM allocations using the allocation `id` and `pool id`, separated by `_`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamPoolCidrAllocation:VpcIpamPoolCidrAllocation example ipam-pool-alloc-0dc6d196509c049ba8b549ff99f639736_ipam-pool-07cfb559e0921fcbe
/// ```
pub mod vpc_ipam_pool_cidr_allocation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrAllocationArgs {
        /// The CIDR you want to assign to the pool.
        #[builder(into, default)]
        pub cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the allocation.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// The netmask length of the CIDR you would like to allocate to the IPAM pool. Valid Values: `0-128`.
        #[builder(into, default)]
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrAllocationResult {
        /// The CIDR you want to assign to the pool.
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// The description for the allocation.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Exclude a particular CIDR range from being returned by the pool.
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub ipam_pool_allocation_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// The netmask length of the CIDR you would like to allocate to the IPAM pool. Valid Values: `0-128`.
        pub netmask_length: pulumi_wasm_rust::Output<i32>,
        /// The ID of the resource.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The owner of the resource.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// The type of the resource.
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcIpamPoolCidrAllocationArgs,
    ) -> VpcIpamPoolCidrAllocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_binding = args.cidr.get_inner();
        let description_binding = args.description.get_inner();
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let netmask_length_binding = args.netmask_length.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPoolCidrAllocation:VpcIpamPoolCidrAllocation".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidr".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disallowedCidrs".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolAllocationId".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "netmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamPoolCidrAllocationResult {
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disallowed_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disallowedCidrs").unwrap(),
            ),
            ipam_pool_allocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolAllocationId").unwrap(),
            ),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netmaskLength").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
        }
    }
}
