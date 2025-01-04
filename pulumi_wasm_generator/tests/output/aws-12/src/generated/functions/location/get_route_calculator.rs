pub mod get_route_calculator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteCalculatorArgs {
        /// Name of the route calculator resource.
        #[builder(into)]
        pub calculator_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the route calculator.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRouteCalculatorResult {
        /// ARN for the Route calculator resource. Use the ARN when you specify a resource across AWS.
        pub calculator_arn: pulumi_wasm_rust::Output<String>,
        pub calculator_name: pulumi_wasm_rust::Output<String>,
        /// Timestamp for when the route calculator resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Data provider of traffic and road network data.
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// Optional description of the route calculator resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the route calculator.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the route calculator resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRouteCalculatorArgs) -> GetRouteCalculatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let calculator_name_binding = args.calculator_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getRouteCalculator:getRouteCalculator".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "calculatorName".into(),
                    value: &calculator_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "calculatorArn".into(),
                },
                register_interface::ResultField {
                    name: "calculatorName".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataSource".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
        GetRouteCalculatorResult {
            calculator_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("calculatorArn").unwrap(),
            ),
            calculator_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("calculatorName").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
