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
pub mod image_pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImagePipelineArgs {
        /// Amazon Resource Name (ARN) of the container recipe.
        #[builder(into, default)]
        pub container_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the image pipeline.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        #[builder(into, default)]
        pub distribution_configuration_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        #[builder(into, default)]
        pub enhanced_image_metadata_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        #[builder(into, default)]
        pub execution_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the image recipe.
        #[builder(into, default)]
        pub image_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        #[builder(into, default)]
        pub image_scanning_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::imagebuilder::ImagePipelineImageScanningConfiguration,
            >,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        #[builder(into, default)]
        pub image_tests_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::imagebuilder::ImagePipelineImageTestsConfiguration,
            >,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        #[builder(into)]
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the image pipeline.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with schedule settings. Detailed below.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::ImagePipelineSchedule>,
        >,
        /// Status of the image pipeline. Valid values are `DISABLED` and `ENABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the image pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        #[builder(into, default)]
        pub workflows: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::ImagePipelineWorkflow>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImagePipelineResult {
        /// Amazon Resource Name (ARN) of the image pipeline.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the container recipe.
        pub container_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Date the image pipeline was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline was last run.
        pub date_last_run: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline will run next.
        pub date_next_run: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline was updated.
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description of the image pipeline.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        pub enhanced_image_metadata_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        pub execution_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the image recipe.
        pub image_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        pub image_scanning_configuration: pulumi_wasm_rust::Output<
            super::super::types::imagebuilder::ImagePipelineImageScanningConfiguration,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        pub image_tests_configuration: pulumi_wasm_rust::Output<
            super::super::types::imagebuilder::ImagePipelineImageTestsConfiguration,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the image pipeline.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Platform of the image pipeline.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Configuration block with schedule settings. Detailed below.
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::ImagePipelineSchedule>,
        >,
        /// Status of the image pipeline. Valid values are `DISABLED` and `ENABLED`. Defaults to `ENABLED`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the image pipeline. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        pub workflows: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::ImagePipelineWorkflow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ImagePipelineArgs) -> ImagePipelineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_recipe_arn_binding = args.container_recipe_arn.get_inner();
        let description_binding = args.description.get_inner();
        let distribution_configuration_arn_binding = args
            .distribution_configuration_arn
            .get_inner();
        let enhanced_image_metadata_enabled_binding = args
            .enhanced_image_metadata_enabled
            .get_inner();
        let execution_role_binding = args.execution_role.get_inner();
        let image_recipe_arn_binding = args.image_recipe_arn.get_inner();
        let image_scanning_configuration_binding = args
            .image_scanning_configuration
            .get_inner();
        let image_tests_configuration_binding = args
            .image_tests_configuration
            .get_inner();
        let infrastructure_configuration_arn_binding = args
            .infrastructure_configuration_arn
            .get_inner();
        let name_binding = args.name.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let workflows_binding = args.workflows.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/imagePipeline:ImagePipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRecipeArn".into(),
                    value: &container_recipe_arn_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "distributionConfigurationArn".into(),
                    value: &distribution_configuration_arn_binding,
                },
                register_interface::ObjectField {
                    name: "enhancedImageMetadataEnabled".into(),
                    value: &enhanced_image_metadata_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "executionRole".into(),
                    value: &execution_role_binding,
                },
                register_interface::ObjectField {
                    name: "imageRecipeArn".into(),
                    value: &image_recipe_arn_binding,
                },
                register_interface::ObjectField {
                    name: "imageScanningConfiguration".into(),
                    value: &image_scanning_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "imageTestsConfiguration".into(),
                    value: &image_tests_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureConfigurationArn".into(),
                    value: &infrastructure_configuration_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workflows".into(),
                    value: &workflows_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "containerRecipeArn".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "dateLastRun".into(),
                },
                register_interface::ResultField {
                    name: "dateNextRun".into(),
                },
                register_interface::ResultField {
                    name: "dateUpdated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "distributionConfigurationArn".into(),
                },
                register_interface::ResultField {
                    name: "enhancedImageMetadataEnabled".into(),
                },
                register_interface::ResultField {
                    name: "executionRole".into(),
                },
                register_interface::ResultField {
                    name: "imageRecipeArn".into(),
                },
                register_interface::ResultField {
                    name: "imageScanningConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "imageTestsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureConfigurationArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "workflows".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ImagePipelineResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            container_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRecipeArn").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            date_last_run: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateLastRun").unwrap(),
            ),
            date_next_run: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateNextRun").unwrap(),
            ),
            date_updated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateUpdated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            distribution_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributionConfigurationArn").unwrap(),
            ),
            enhanced_image_metadata_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedImageMetadataEnabled").unwrap(),
            ),
            execution_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRole").unwrap(),
            ),
            image_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageRecipeArn").unwrap(),
            ),
            image_scanning_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageScanningConfiguration").unwrap(),
            ),
            image_tests_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTestsConfiguration").unwrap(),
            ),
            infrastructure_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureConfigurationArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            workflows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflows").unwrap(),
            ),
        }
    }
}
