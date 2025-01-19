/// Previews a CIDR from an IPAM address pool. Only works for private IPv4.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamPreviewNextCidr
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       netmaskLength: 28
///       disallowedCidrs:
///         - 172.2.0.0/32
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
pub mod vpc_ipam_preview_next_cidr {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrArgs {
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        #[builder(into, default)]
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrResult {
        /// The previewed CIDR from the pool.
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// Exclude a particular CIDR range from being returned by the pool.
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcIpamPreviewNextCidrArgs,
    ) -> VpcIpamPreviewNextCidrResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let netmask_length_binding = args.netmask_length.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPreviewNextCidr:VpcIpamPreviewNextCidr".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "disallowedCidrs".into(),
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
        VpcIpamPreviewNextCidrResult {
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            disallowed_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disallowedCidrs").unwrap(),
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
