pub mod get_coip_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCoipPoolArgs {
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetCoipPoolFilter>>,
        >,
        /// Local Gateway Route Table Id assigned to desired COIP Pool
        #[builder(into, default)]
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the specific COIP Pool to retrieve.
        #[builder(into, default)]
        pub pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired COIP Pool.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCoipPoolResult {
        /// ARN of the COIP pool
        pub arn: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetCoipPoolFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Set of CIDR blocks in pool
        pub pool_cidrs: pulumi_wasm_rust::Output<Vec<String>>,
        pub pool_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCoipPoolArgs) -> GetCoipPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_inner();
        let pool_id_binding = args.pool_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getCoipPool:getCoipPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: &local_gateway_route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "poolId".into(),
                    value: &pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "poolCidrs".into(),
                },
                register_interface::ResultField {
                    name: "poolId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCoipPoolResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            local_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayRouteTableId").unwrap(),
            ),
            pool_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolCidrs").unwrap(),
            ),
            pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
