pub mod get_local_gateway_virtual_interface_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupsArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaceGroups.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupsFilter,
                >,
            >,
        >,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupsFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of EC2 Local Gateway Virtual Interface Group identifiers.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of EC2 Local Gateway Virtual Interface identifiers.
        pub local_gateway_virtual_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLocalGatewayVirtualInterfaceGroupsArgs,
    ) -> GetLocalGatewayVirtualInterfaceGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLocalGatewayVirtualInterfaceGroups:getLocalGatewayVirtualInterfaceGroups"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayVirtualInterfaceIds".into(),
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
        GetLocalGatewayVirtualInterfaceGroupsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            local_gateway_virtual_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayVirtualInterfaceIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
