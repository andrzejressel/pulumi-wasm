/// Manages an Image Builder Image.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = image::create(
///         "example",
///         ImageArgs::builder()
///             .distribution_configuration_arn(
///                 "${exampleAwsImagebuilderDistributionConfiguration.arn}",
///             )
///             .image_recipe_arn("${exampleAwsImagebuilderImageRecipe.arn}")
///             .infrastructure_configuration_arn(
///                 "${exampleAwsImagebuilderInfrastructureConfiguration.arn}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_image` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/image:Image example arn:aws:imagebuilder:us-east-1:123456789012:image/example/1.0.0/1
/// ```
pub mod image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// Amazon Resource Name (ARN) of the container recipe.
        #[builder(into, default)]
        pub container_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
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
            Option<super::super::types::imagebuilder::ImageImageScanningConfiguration>,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        #[builder(into, default)]
        pub image_tests_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::ImageImageTestsConfiguration>,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the Image Builder Image. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        #[builder(into, default)]
        pub workflows: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::ImageWorkflow>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// Amazon Resource Name (ARN) of the image.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the container recipe.
        pub container_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Date the image was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        pub enhanced_image_metadata_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        pub execution_role: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the image recipe.
        pub image_recipe_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        pub image_scanning_configuration: pulumi_wasm_rust::Output<
            super::super::types::imagebuilder::ImageImageScanningConfiguration,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        pub image_tests_configuration: pulumi_wasm_rust::Output<
            super::super::types::imagebuilder::ImageImageTestsConfiguration,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        ///
        /// The following arguments are optional:
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the AMI.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Operating System version of the image.
        pub os_version: pulumi_wasm_rust::Output<String>,
        /// List of objects with resources created by the image.
        pub output_resources: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::ImageOutputResource>,
        >,
        /// Platform of the image.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the Image Builder Image. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version of the image.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Configuration block with the workflow configuration. Detailed below.
        pub workflows: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::ImageWorkflow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ImageArgs) -> ImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_recipe_arn_binding = args.container_recipe_arn.get_inner();
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
        let tags_binding = args.tags.get_inner();
        let workflows_binding = args.workflows.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/image:Image".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRecipeArn".into(),
                    value: &container_recipe_arn_binding,
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
                    name: "osVersion".into(),
                },
                register_interface::ResultField {
                    name: "outputResources".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
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
        ImageResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            container_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRecipeArn").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
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
            os_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osVersion").unwrap(),
            ),
            output_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputResources").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            workflows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflows").unwrap(),
            ),
        }
    }
}
