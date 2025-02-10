#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_shared_image_gallery {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageGalleryArgs {
        /// The name of the Shared Image Gallery.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageGalleryResult {
        /// A description for the Shared Image Gallery.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of Shared Image names within this Shared Image Gallery.
        pub image_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which are assigned to the Shared Image Gallery.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The unique name assigned to the Shared Image Gallery.
        pub unique_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSharedImageGalleryArgs,
    ) -> GetSharedImageGalleryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getSharedImageGallery:getSharedImageGallery".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSharedImageGalleryResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            image_names: o.get_field("imageNames"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            unique_name: o.get_field("uniqueName"),
        }
    }
}
