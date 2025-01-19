pub mod get_vpc_iam_pools {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIamPoolsArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolsFilter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcIamPoolsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of IPAM pools and their attributes. See below for details
        pub ipam_pools: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcIamPoolsIpamPool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcIamPoolsArgs) -> GetVpcIamPoolsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcIamPools:getVpcIamPools".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
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
                    name: "ipamPools".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcIamPoolsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipam_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPools").unwrap(),
            ),
        }
    }
}
