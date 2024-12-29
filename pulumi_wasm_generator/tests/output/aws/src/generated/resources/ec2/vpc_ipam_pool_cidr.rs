/// Provisions a CIDR from an IPAM address pool.
///
/// > **NOTE:** Provisioning Public IPv4 or Public IPv6 require [steps outside the scope of this resource](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html#prepare-for-byoip). The resource accepts `message` and `signature` as part of the `cidr_authorization_context` attribute but those must be generated ahead of time. Public IPv6 CIDRs that are provisioned into a Pool with `publicly_advertisable = true` and all public IPv4 CIDRs also require creating a Route Origin Authorization (ROA) object in your Regional Internet Registry (RIR).
///
/// > **NOTE:** In order to deprovision CIDRs all Allocations must be released. Allocations created by a VPC take up to 30 minutes to be released. However, for IPAM to properly manage the removal of allocation records created by VPCs and other resources, you must [grant it permissions](https://docs.aws.amazon.com/vpc/latest/ipam/choose-single-user-or-orgs-ipam.html) in
/// either a single account or organizationally. If you are unable to deprovision a cidr after waiting over 30 minutes, you may be missing the Service Linked Role.
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
///     let example = vpc_ipam::create(
///         "example",
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
///             .ipam_scope_id("${example.privateDefaultScopeId}")
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
/// Provision Public IPv6 Pool CIDRs:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let example = vpc_ipam::create(
///         "example",
///         VpcIpamArgs::builder()
///             .operating_regions(
///                 vec![
///                     VpcIpamOperatingRegion::builder().regionName("${current.name}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let ipv6TestPublic = vpc_ipam_pool::create(
///         "ipv6TestPublic",
///         VpcIpamPoolArgs::builder()
///             .address_family("ipv6")
///             .aws_service("ec2")
///             .description("public ipv6")
///             .ipam_scope_id("${example.publicDefaultScopeId}")
///             .locale("us-east-1")
///             .public_ip_source("amazon")
///             .publicly_advertisable(false)
///             .build_struct(),
///     );
///     let ipv6TestPublicVpcIpamPoolCidr = vpc_ipam_pool_cidr::create(
///         "ipv6TestPublicVpcIpamPoolCidr",
///         VpcIpamPoolCidrArgs::builder()
///             .ipam_pool_id("${ipv6TestPublic.id}")
///             .netmask_length(52)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the `<cidr>_<ipam-pool-id>`. For example:
///
/// __NOTE:__ Do not use the IPAM Pool Cidr ID as this was introduced after the resource already existed.
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamPoolCidr:VpcIpamPoolCidr example 172.20.0.0/24_ipam-pool-0e634f5a1517cccdc
/// ```
pub mod vpc_ipam_pool_cidr {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrArgs {
        /// The CIDR you want to assign to the pool. Conflicts with `netmask_length`.
        #[builder(into, default)]
        pub cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP. This is not stored in the state file. See cidr_authorization_context for more information.
        #[builder(into, default)]
        pub cidr_authorization_context: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcIpamPoolCidrCidrAuthorizationContext>,
        >,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// If provided, the cidr provisioned into the specified pool will be the next available cidr given this declared netmask length. Conflicts with `cidr`.
        #[builder(into, default)]
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolCidrResult {
        /// The CIDR you want to assign to the pool. Conflicts with `netmask_length`.
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP. This is not stored in the state file. See cidr_authorization_context for more information.
        pub cidr_authorization_context: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcIpamPoolCidrCidrAuthorizationContext>,
        >,
        /// The unique ID generated by AWS for the pool cidr. Typically this is the resource `id` but this attribute was added to the API calls after the fact and is therefore not used as the resource id.
        pub ipam_pool_cidr_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// If provided, the cidr provisioned into the specified pool will be the next available cidr given this declared netmask length. Conflicts with `cidr`.
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcIpamPoolCidrArgs) -> VpcIpamPoolCidrResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_binding = args.cidr.get_inner();
        let cidr_authorization_context_binding = args
            .cidr_authorization_context
            .get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let netmask_length_binding = args.netmask_length.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPoolCidr:VpcIpamPoolCidr".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "cidrAuthorizationContext".into(),
                    value: &cidr_authorization_context_binding,
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
                    name: "cidrAuthorizationContext".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolCidrId".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "netmaskLength".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamPoolCidrResult {
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            cidr_authorization_context: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrAuthorizationContext").unwrap(),
            ),
            ipam_pool_cidr_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolCidrId").unwrap(),
            ),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netmaskLength").unwrap(),
            ),
        }
    }
}
