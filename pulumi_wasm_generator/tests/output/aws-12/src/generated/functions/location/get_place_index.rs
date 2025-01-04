pub mod get_place_index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlaceIndexArgs {
        /// Name of the place index resource.
        #[builder(into)]
        pub index_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the place index.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPlaceIndexResult {
        /// Timestamp for when the place index resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Data provider of geospatial data.
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// List of configurations that specify data storage option for requesting Places.
        pub data_source_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::location::GetPlaceIndexDataSourceConfiguration,
            >,
        >,
        /// Optional description for the place index resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN for the place index resource.
        pub index_arn: pulumi_wasm_rust::Output<String>,
        pub index_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the place index.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the place index resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPlaceIndexArgs) -> GetPlaceIndexResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let index_name_binding = args.index_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getPlaceIndex:getPlaceIndex".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "indexName".into(),
                    value: &index_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataSource".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "indexArn".into(),
                },
                register_interface::ResultField {
                    name: "indexName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPlaceIndexResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            data_source_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceConfigurations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            index_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexArn").unwrap(),
            ),
            index_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
