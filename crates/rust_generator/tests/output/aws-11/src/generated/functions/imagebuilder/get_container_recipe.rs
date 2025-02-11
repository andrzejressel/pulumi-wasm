#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_container_recipe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerRecipeArgs {
        /// ARN of the container recipe.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the container recipe.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetContainerRecipeResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of objects with components for the container recipe.
        pub components: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetContainerRecipeComponent>,
        >,
        /// Type of the container.
        pub container_type: pulumi_gestalt_rust::Output<String>,
        /// Date the container recipe was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the container recipe.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Dockerfile template used to build the image.
        pub dockerfile_template_data: pulumi_gestalt_rust::Output<String>,
        /// Whether to encrypt the volume. Defaults to unset, which is the value inherited from the parent image.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of objects with instance configurations for building and testing container images.
        pub instance_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetContainerRecipeInstanceConfiguration,
            >,
        >,
        /// KMS key used to encrypt the container image.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the container recipe.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the container recipe.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Base image for the container recipe.
        pub parent_image: pulumi_gestalt_rust::Output<String>,
        /// Platform of the container recipe.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the container recipe.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Destination repository for the container image.
        pub target_repositories: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetContainerRecipeTargetRepository,
            >,
        >,
        /// Version of the container recipe.
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
        args: GetContainerRecipeArgs,
    ) -> GetContainerRecipeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getContainerRecipe:getContainerRecipe".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContainerRecipeResult {
            arn: o.get_field("arn"),
            components: o.get_field("components"),
            container_type: o.get_field("containerType"),
            date_created: o.get_field("dateCreated"),
            description: o.get_field("description"),
            dockerfile_template_data: o.get_field("dockerfileTemplateData"),
            encrypted: o.get_field("encrypted"),
            id: o.get_field("id"),
            instance_configurations: o.get_field("instanceConfigurations"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            parent_image: o.get_field("parentImage"),
            platform: o.get_field("platform"),
            tags: o.get_field("tags"),
            target_repositories: o.get_field("targetRepositories"),
            version: o.get_field("version"),
            working_directory: o.get_field("workingDirectory"),
        }
    }
}
