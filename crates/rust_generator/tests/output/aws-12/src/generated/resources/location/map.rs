/// Provides a Location Service Map.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MapArgs {
        /// Configuration block with the map style selected from an available data provider. Detailed below.
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::location::MapConfiguration,
        >,
        /// An optional description for the map resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the map resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub map_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the map. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MapResult {
        /// Configuration block with the map style selected from an available data provider. Detailed below.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::location::MapConfiguration,
        >,
        /// The timestamp for when the map resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An optional description for the map resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the map resource. Used to specify a resource across all AWS.
        pub map_arn: pulumi_gestalt_rust::Output<String>,
        /// The name for the map resource.
        ///
        /// The following arguments are optional:
        pub map_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the map. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the map resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MapArgs,
    ) -> MapResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let description_binding = args.description.get_output(context);
        let map_name_binding = args.map_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/map:Map".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mapName".into(),
                    value: map_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MapResult {
            configuration: o.get_field("configuration"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            map_arn: o.get_field("mapArn"),
            map_name: o.get_field("mapName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
