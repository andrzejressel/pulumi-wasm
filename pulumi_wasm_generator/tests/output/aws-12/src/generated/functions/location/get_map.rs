pub mod get_map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMapArgs {
        /// Name of the map resource.
        #[builder(into)]
        pub map_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the map.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMapResult {
        /// List of configurations that specify the map tile style selected from a partner data provider.
        pub configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::location::GetMapConfiguration>,
        >,
        /// Timestamp for when the map resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional description for the map resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN for the map resource.
        pub map_arn: pulumi_wasm_rust::Output<String>,
        pub map_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the map.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the map resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetMapArgs,
    ) -> GetMapResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let map_name_binding = args.map_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getMap:getMap".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mapName".into(),
                    value: &map_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "mapArn".into(),
                },
                register_interface::ResultField {
                    name: "mapName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMapResult {
            configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            map_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapArn").unwrap(),
            ),
            map_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapName").unwrap(),
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
