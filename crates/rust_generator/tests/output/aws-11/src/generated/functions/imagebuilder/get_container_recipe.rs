#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetContainerRecipeArgs,
    ) -> GetContainerRecipeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getContainerRecipe:getContainerRecipe".into(),
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
        GetContainerRecipeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            components: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("components"),
            ),
            container_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerType"),
            ),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dockerfile_template_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerfileTemplateData"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceConfigurations"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            parent_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentImage"),
            ),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_repositories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetRepositories"),
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
