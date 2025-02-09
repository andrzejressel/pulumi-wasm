#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMapArgs {
        /// Name of the map resource.
        #[builder(into)]
        pub map_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the map.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMapResult {
        /// List of configurations that specify the map tile style selected from a partner data provider.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::location::GetMapConfiguration>,
        >,
        /// Timestamp for when the map resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional description for the map resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the map resource.
        pub map_arn: pulumi_gestalt_rust::Output<String>,
        pub map_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the map.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the map resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMapArgs,
    ) -> GetMapResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let map_name_binding = args.map_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getMap:getMap".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetMapResult {
            configurations: o.get_field("configurations"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            map_arn: o.get_field("mapArn"),
            map_name: o.get_field("mapName"),
            tags: o.get_field("tags"),
            update_time: o.get_field("updateTime"),
        }
    }
}
