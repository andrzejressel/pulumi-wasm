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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetImagePipelineArgs,
    ) -> GetImagePipelineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImagePipeline:getImagePipeline".into(),
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
        GetImagePipelineResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            container_recipe_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerRecipeArn"),
            ),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            date_last_run: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateLastRun"),
            ),
            date_next_run: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateNextRun"),
            ),
            date_updated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateUpdated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            distribution_configuration_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distributionConfigurationArn"),
            ),
            enhanced_image_metadata_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enhancedImageMetadataEnabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_recipe_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageRecipeArn"),
            ),
            image_scanning_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageScanningConfigurations"),
            ),
            image_tests_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageTestsConfigurations"),
            ),
            infrastructure_configuration_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureConfigurationArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            schedules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedules"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
