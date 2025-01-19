pub mod get_ipam_preview_next_cidr {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpamPreviewNextCidrArgs {
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// Netmask length of the CIDR you would like to preview from the IPAM pool.
        #[builder(into, default)]
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetIpamPreviewNextCidrResult {
        /// Previewed CIDR from the pool.
        pub cidr: pulumi_wasm_rust::Output<String>,
        pub disallowed_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        pub netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetIpamPreviewNextCidrArgs) -> GetIpamPreviewNextCidrResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let netmask_length_binding = args.netmask_length.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getIpamPreviewNextCidr:getIpamPreviewNextCidr".into(),
            version: super::super::super::get_version(),
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "netmaskLength".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIpamPreviewNextCidrResult {
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            disallowed_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disallowedCidrs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netmaskLength").unwrap(),
            ),
        }
    }
}
