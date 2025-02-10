#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_images {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagesArgs {
        /// The name of the Resource Group in which the Image exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to filter the list of images against.
        #[builder(into, default)]
        pub tags_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImagesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `images` blocks as defined below:
        pub images: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetImagesImage>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub tags_filter: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImagesArgs,
    ) -> GetImagesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_filter_binding = args.tags_filter.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getImages:getImages".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagsFilter".into(),
                    value: tags_filter_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImagesResult {
            id: o.get_field("id"),
            images: o.get_field("images"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags_filter: o.get_field("tagsFilter"),
        }
    }
}
