#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image_recipe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageRecipeArgs {
        /// ARN of the image recipe.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the image recipe.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImageRecipeResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of objects with block device mappings for the image recipe.
        pub block_device_mappings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageRecipeBlockDeviceMapping,
            >,
        >,
        /// List of objects with components for the image recipe.
        pub components: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImageRecipeComponent>,
        >,
        /// Date the image recipe was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the image recipe.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the image recipe.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the image recipe.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Base image of the image recipe.
        pub parent_image: pulumi_gestalt_rust::Output<String>,
        /// Platform of the image recipe.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the image recipe.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Base64 encoded contents of user data. Commands or a command script to run when build instance is launched.
        pub user_data_base64: pulumi_gestalt_rust::Output<String>,
        /// Version of the image recipe.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Working directory used during build and test workflows.
        pub working_directory: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImageRecipeArgs,
    ) -> GetImageRecipeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getImageRecipe:getImageRecipe".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImageRecipeResult {
            arn: o.get_field("arn"),
            block_device_mappings: o.get_field("blockDeviceMappings"),
            components: o.get_field("components"),
            date_created: o.get_field("dateCreated"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            parent_image: o.get_field("parentImage"),
            platform: o.get_field("platform"),
            tags: o.get_field("tags"),
            user_data_base64: o.get_field("userDataBase64"),
            version: o.get_field("version"),
            working_directory: o.get_field("workingDirectory"),
        }
    }
}
