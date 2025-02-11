#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image_pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagePipelineArgs {
        /// ARN of the image pipeline.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the image pipeline.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImagePipelineResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the container recipe.
        pub container_recipe_arn: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline was last run.
        pub date_last_run: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline will run next.
        pub date_next_run: pulumi_gestalt_rust::Output<String>,
        /// Date the image pipeline was updated.
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description of the image pipeline.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether additional information about the image being created is collected.
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the image recipe.
        pub image_recipe_arn: pulumi_gestalt_rust::Output<String>,
        pub image_scanning_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImagePipelineImageScanningConfiguration,
            >,
        >,
        /// List of an object with image tests configuration.
        pub image_tests_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImagePipelineImageTestsConfiguration,
            >,
        >,
        /// ARN of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the image pipeline.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Platform of the image pipeline.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// List of an object with schedule settings.
        pub schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImagePipelineSchedule>,
        >,
        /// Status of the image pipeline.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the image pipeline.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImagePipelineArgs,
    ) -> GetImagePipelineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getImagePipeline:getImagePipeline".into(),
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
        GetImagePipelineResult {
            arn: o.get_field("arn"),
            container_recipe_arn: o.get_field("containerRecipeArn"),
            date_created: o.get_field("dateCreated"),
            date_last_run: o.get_field("dateLastRun"),
            date_next_run: o.get_field("dateNextRun"),
            date_updated: o.get_field("dateUpdated"),
            description: o.get_field("description"),
            distribution_configuration_arn: o.get_field("distributionConfigurationArn"),
            enhanced_image_metadata_enabled: o.get_field("enhancedImageMetadataEnabled"),
            id: o.get_field("id"),
            image_recipe_arn: o.get_field("imageRecipeArn"),
            image_scanning_configurations: o.get_field("imageScanningConfigurations"),
            image_tests_configurations: o.get_field("imageTestsConfigurations"),
            infrastructure_configuration_arn: o
                .get_field("infrastructureConfigurationArn"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            schedules: o.get_field("schedules"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
