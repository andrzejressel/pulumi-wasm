#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_place_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlaceIndexArgs {
        /// Name of the place index resource.
        #[builder(into)]
        pub index_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the place index.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPlaceIndexResult {
        /// Timestamp for when the place index resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Data provider of geospatial data.
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// List of configurations that specify data storage option for requesting Places.
        pub data_source_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::location::GetPlaceIndexDataSourceConfiguration,
            >,
        >,
        /// Optional description for the place index resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the place index resource.
        pub index_arn: pulumi_gestalt_rust::Output<String>,
        pub index_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the place index.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the place index resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPlaceIndexArgs,
    ) -> GetPlaceIndexResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let index_name_binding = args.index_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getPlaceIndex:getPlaceIndex".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexName".into(),
                    value: index_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPlaceIndexResult {
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
            data_source_configurations: o.get_field("dataSourceConfigurations"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            index_arn: o.get_field("indexArn"),
            index_name: o.get_field("indexName"),
            tags: o.get_field("tags"),
            update_time: o.get_field("updateTime"),
        }
    }
}
