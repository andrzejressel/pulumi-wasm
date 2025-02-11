/// Manages an Image Builder Image Pipeline.
///
/// > **NOTE:** Starting with version `5.74.0`, lifecycle meta-argument `replace_triggered_by` must be used in order to prevent a dependency error on destroy.
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_image_pipeline` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/imagePipeline:ImagePipeline example arn:aws:imagebuilder:us-east-1:123456789012:image-pipeline/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image_pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImagePipelineArgs {
        /// Amazon Resource Name (ARN) of the container recipe.
        #[builder(into, default)]
        pub container_recipe_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the image pipeline.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        #[builder(into, default)]
        pub distribution_configuration_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        #[builder(into, default)]
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        #[builder(into, default)]
        pub execution_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the image recipe.
        #[builder(into, default)]
        pub image_recipe_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        #[builder(into, default)]
        pub image_scanning_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::imagebuilder::ImagePipelineImageScanningConfiguration,
            >,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        #[builder(into, default)]
        pub image_tests_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::imagebuilder::ImagePipelineImageTestsConfiguration,
            >,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        #[builder(into)]
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the image pipeline.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with schedule settings. Detailed below.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::ImagePipelineSchedule>,
        >,
        /// Status of the image pipeline. Valid values are `DISABLED` and `ENABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the image pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        #[builder(into, default)]
        pub workflows: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::imagebuilder::ImagePipelineWorkflow>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImagePipelineResult {
        /// Amazon Resource Name (ARN) of the image pipeline.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the container recipe.
        pub container_recipe_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date the image pipeline was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline was last run.
        pub date_last_run: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline will run next.
        pub date_next_run: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline was updated.
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description of the image pipeline.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        pub execution_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the image recipe.
        pub image_recipe_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        pub image_scanning_configuration: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ImagePipelineImageScanningConfiguration,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        pub image_tests_configuration: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ImagePipelineImageTestsConfiguration,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the image pipeline.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Platform of the image pipeline.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with schedule settings. Detailed below.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::imagebuilder::ImagePipelineSchedule>,
        >,
        /// Status of the image pipeline. Valid values are `DISABLED` and `ENABLED`. Defaults to `ENABLED`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the image pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        pub workflows: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::ImagePipelineWorkflow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImagePipelineArgs,
    ) -> ImagePipelineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_recipe_arn_binding = args.container_recipe_arn.get_output(context);
        let description_binding = args.description.get_output(context);
        let distribution_configuration_arn_binding = args
            .distribution_configuration_arn
            .get_output(context);
        let enhanced_image_metadata_enabled_binding = args
            .enhanced_image_metadata_enabled
            .get_output(context);
        let execution_role_binding = args.execution_role.get_output(context);
        let image_recipe_arn_binding = args.image_recipe_arn.get_output(context);
        let image_scanning_configuration_binding = args
            .image_scanning_configuration
            .get_output(context);
        let image_tests_configuration_binding = args
            .image_tests_configuration
            .get_output(context);
        let infrastructure_configuration_arn_binding = args
            .infrastructure_configuration_arn
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workflows_binding = args.workflows.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/imagePipeline:ImagePipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRecipeArn".into(),
                    value: &container_recipe_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distributionConfigurationArn".into(),
                    value: &distribution_configuration_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedImageMetadataEnabled".into(),
                    value: &enhanced_image_metadata_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRole".into(),
                    value: &execution_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageRecipeArn".into(),
                    value: &image_recipe_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageScanningConfiguration".into(),
                    value: &image_scanning_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageTestsConfiguration".into(),
                    value: &image_tests_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "infrastructureConfigurationArn".into(),
                    value: &infrastructure_configuration_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflows".into(),
                    value: &workflows_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImagePipelineResult {
            arn: o.get_field("arn"),
            container_recipe_arn: o.get_field("containerRecipeArn"),
            date_created: o.get_field("dateCreated"),
            date_last_run: o.get_field("dateLastRun"),
            date_next_run: o.get_field("dateNextRun"),
            date_updated: o.get_field("dateUpdated"),
            description: o.get_field("description"),
            distribution_configuration_arn: o.get_field("distributionConfigurationArn"),
            enhanced_image_metadata_enabled: o.get_field("enhancedImageMetadataEnabled"),
            execution_role: o.get_field("executionRole"),
            image_recipe_arn: o.get_field("imageRecipeArn"),
            image_scanning_configuration: o.get_field("imageScanningConfiguration"),
            image_tests_configuration: o.get_field("imageTestsConfiguration"),
            infrastructure_configuration_arn: o
                .get_field("infrastructureConfigurationArn"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            schedule: o.get_field("schedule"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            workflows: o.get_field("workflows"),
        }
    }
}
