#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// ARN of the image. The suffix can either be specified with wildcards (`x.x.x`) to fetch the latest build version or a full build version (e.g., `2020.11.26/1`) to fetch an exact version.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the image.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Build version ARN of the image. This will always have the `#.#.#/#` suffix.
        pub build_version_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the container recipe.
        pub container_recipe_arn: pulumi_gestalt_rust::Output<String>,
        /// Date the image was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether additional information about the image being created is collected.
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the image recipe.
        pub image_recipe_arn: pulumi_gestalt_rust::Output<String>,
        /// List of an object with image scanning configuration fields.
        pub image_scanning_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageImageScanningConfiguration,
            >,
        >,
        /// List of an object with image tests configuration.
        pub image_tests_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageImageTestsConfiguration,
            >,
        >,
        /// ARN of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Operating System version of the image.
        pub os_version: pulumi_gestalt_rust::Output<String>,
        /// List of objects with resources created by the image.
        pub output_resources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImageOutputResource>,
        >,
        /// Platform of the image.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the image.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Version of the image.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImageArgs,
    ) -> GetImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getImage:getImage".into(),
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
        GetImageResult {
            arn: o.get_field("arn"),
            build_version_arn: o.get_field("buildVersionArn"),
            container_recipe_arn: o.get_field("containerRecipeArn"),
            date_created: o.get_field("dateCreated"),
            distribution_configuration_arn: o.get_field("distributionConfigurationArn"),
            enhanced_image_metadata_enabled: o.get_field("enhancedImageMetadataEnabled"),
            id: o.get_field("id"),
            image_recipe_arn: o.get_field("imageRecipeArn"),
            image_scanning_configurations: o.get_field("imageScanningConfigurations"),
            image_tests_configurations: o.get_field("imageTestsConfigurations"),
            infrastructure_configuration_arn: o
                .get_field("infrastructureConfigurationArn"),
            name: o.get_field("name"),
            os_version: o.get_field("osVersion"),
            output_resources: o.get_field("outputResources"),
            platform: o.get_field("platform"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}
