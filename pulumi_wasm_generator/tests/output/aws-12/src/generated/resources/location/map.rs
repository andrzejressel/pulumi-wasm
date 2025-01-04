/// Provides a Location Service Map.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = map::create(
///         "example",
///         MapArgs::builder()
///             .configuration(
///                 MapConfiguration::builder().style("VectorHereBerlin").build_struct(),
///             )
///             .map_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_location_map` resources using the map name. For example:
///
/// ```sh
/// $ pulumi import aws:location/map:Map example example
/// ```
pub mod map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MapArgs {
        /// Configuration block with the map style selected from an available data provider. Detailed below.
        #[builder(into)]
        pub configuration: pulumi_wasm_rust::Output<
            super::super::types::location::MapConfiguration,
        >,
        /// An optional description for the map resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the map resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub map_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the map. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MapResult {
        /// Configuration block with the map style selected from an available data provider. Detailed below.
        pub configuration: pulumi_wasm_rust::Output<
            super::super::types::location::MapConfiguration,
        >,
        /// The timestamp for when the map resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description for the map resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the map resource. Used to specify a resource across all AWS.
        pub map_arn: pulumi_wasm_rust::Output<String>,
        /// The name for the map resource.
        ///
        /// The following arguments are optional:
        pub map_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the map. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the map resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MapArgs) -> MapResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let description_binding = args.description.get_inner();
        let map_name_binding = args.map_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:location/map:Map".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
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
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
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
        MapResult {
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            map_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapArn").unwrap(),
            ),
            map_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapName").unwrap(),
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
