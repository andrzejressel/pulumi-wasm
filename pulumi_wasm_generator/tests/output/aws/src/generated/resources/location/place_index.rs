/// Provides a Location Service Place Index.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = place_index::create(
///         "example",
///         PlaceIndexArgs::builder()
///             .data_source("Here")
///             .index_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_location_place_index` resources using the place index name. For example:
///
/// ```sh
/// $ pulumi import aws:location/placeIndex:PlaceIndex example example
/// ```
pub mod place_index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlaceIndexArgs {
        /// Specifies the geospatial data provider for the new place index.
        #[builder(into)]
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// Configuration block with the data storage option chosen for requesting Places. Detailed below.
        #[builder(into, default)]
        pub data_source_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::location::PlaceIndexDataSourceConfiguration>,
        >,
        /// The optional description for the place index resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the place index resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub index_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlaceIndexResult {
        /// The timestamp for when the place index resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Specifies the geospatial data provider for the new place index.
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// Configuration block with the data storage option chosen for requesting Places. Detailed below.
        pub data_source_configuration: pulumi_wasm_rust::Output<
            super::super::types::location::PlaceIndexDataSourceConfiguration,
        >,
        /// The optional description for the place index resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the place index resource. Used to specify a resource across AWS.
        pub index_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the place index resource.
        ///
        /// The following arguments are optional:
        pub index_name: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the place index resource was last update in ISO 8601.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PlaceIndexArgs) -> PlaceIndexResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_source_binding = args.data_source.get_inner();
        let data_source_configuration_binding = args
            .data_source_configuration
            .get_inner();
        let description_binding = args.description.get_inner();
        let index_name_binding = args.index_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:location/placeIndex:PlaceIndex".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceConfiguration".into(),
                    value: &data_source_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
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
                    name: "dataSourceConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
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
        PlaceIndexResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            data_source_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceConfiguration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            index_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexArn").unwrap(),
            ),
            index_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexName").unwrap(),
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