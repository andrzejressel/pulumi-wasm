pub mod get_vpc_ipam_pool_cidrs {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIpamPoolCidrsArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcIpamPoolCidrsFilter>>,
        >,
        /// ID of the IPAM pool you would like the list of provisioned CIDRs.
        #[builder(into)]
        pub ipam_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpcIpamPoolCidrsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIpamPoolCidrsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The CIDRs provisioned into the IPAM pool, described below.
        pub ipam_pool_cidrs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcIpamPoolCidrsIpamPoolCidr>,
        >,
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVpcIpamPoolCidrsArgs,
    ) -> GetVpcIpamPoolCidrsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcIpamPoolCidrs:getVpcIpamPoolCidrs".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolCidrs".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcIpamPoolCidrsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipam_pool_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolCidrs").unwrap(),
            ),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
        }
    }
}
