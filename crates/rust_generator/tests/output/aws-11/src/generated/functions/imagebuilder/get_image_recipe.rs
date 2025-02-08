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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetImageRecipeArgs,
    ) -> GetImageRecipeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImageRecipe:getImageRecipe".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImageRecipeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            block_device_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blockDeviceMappings"),
            ),
            components: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("components"),
            ),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            parent_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentImage"),
            ),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            user_data_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userDataBase64"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            working_directory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workingDirectory"),
            ),
        }
    }
}
