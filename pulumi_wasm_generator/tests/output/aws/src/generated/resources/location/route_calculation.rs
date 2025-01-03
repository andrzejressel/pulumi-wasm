/// Provides a Location Service Route Calculator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route_calculation::create(
///         "example",
///         RouteCalculationArgs::builder()
///             .calculator_name("example")
///             .data_source("Here")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_location_route_calculator` using the route calculator name. For example:
///
/// ```sh
/// $ pulumi import aws:location/routeCalculation:RouteCalculation example example
/// ```
pub mod route_calculation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteCalculationArgs {
        /// The name of the route calculator resource.
        #[builder(into)]
        pub calculator_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the data provider of traffic and road network data.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// The optional description for the route calculator resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the route calculator. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteCalculationResult {
        /// The Amazon Resource Name (ARN) for the Route calculator resource. Use the ARN when you specify a resource across AWS.
        pub calculator_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the route calculator resource.
        pub calculator_name: pulumi_wasm_rust::Output<String>,
        /// The timestamp for when the route calculator resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Specifies the data provider of traffic and road network data.
        ///
        /// The following arguments are optional:
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// The optional description for the route calculator resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the route calculator. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the route calculator resource was last update in ISO 8601.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouteCalculationArgs) -> RouteCalculationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let calculator_name_binding = args.calculator_name.get_inner();
        let data_source_binding = args.data_source.get_inner();
        let description_binding = args.description.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:location/routeCalculation:RouteCalculation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "calculatorName".into(),
                    value: &calculator_name_binding,
                },
                register_interface::ObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteCalculationResult {
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
