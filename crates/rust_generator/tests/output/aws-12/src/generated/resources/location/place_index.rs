/// Provides a Location Service Place Index.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod place_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlaceIndexArgs {
        /// Specifies the geospatial data provider for the new place index.
        #[builder(into)]
        pub data_source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block with the data storage option chosen for requesting Places. Detailed below.
        #[builder(into, default)]
        pub data_source_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::location::PlaceIndexDataSourceConfiguration>,
        >,
        /// The optional description for the place index resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the place index resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub index_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlaceIndexResult {
        /// The timestamp for when the place index resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the geospatial data provider for the new place index.
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with the data storage option chosen for requesting Places. Detailed below.
        pub data_source_configuration: pulumi_gestalt_rust::Output<
            super::super::types::location::PlaceIndexDataSourceConfiguration,
        >,
        /// The optional description for the place index resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the place index resource. Used to specify a resource across AWS.
        pub index_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the place index resource.
        ///
        /// The following arguments are optional:
        pub index_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the place index. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the place index resource was last update in ISO 8601.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PlaceIndexArgs,
    ) -> PlaceIndexResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_source_binding = args.data_source.get_output(context);
        let data_source_configuration_binding = args
            .data_source_configuration
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let index_name_binding = args.index_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/placeIndex:PlaceIndex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceConfiguration".into(),
                    value: &data_source_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexName".into(),
                    value: &index_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PlaceIndexResult {
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
            data_source_configuration: o.get_field("dataSourceConfiguration"),
            description: o.get_field("description"),
            index_arn: o.get_field("indexArn"),
            index_name: o.get_field("indexName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
