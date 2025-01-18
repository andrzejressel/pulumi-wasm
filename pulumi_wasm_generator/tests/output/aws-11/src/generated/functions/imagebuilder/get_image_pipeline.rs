pub mod get_image_pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagePipelineArgs {
        /// ARN of the image pipeline.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the image pipeline.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImagePipelineResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the container recipe.
        pub container_recipe_arn: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline was last run.
        pub date_last_run: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline will run next.
        pub date_next_run: pulumi_wasm_rust::Output<String>,
        /// Date the image pipeline was updated.
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description of the image pipeline.
        pub description: pulumi_wasm_rust::Output<String>,
        /// ARN of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Whether additional information about the image being created is collected.
        pub enhanced_image_metadata_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the image recipe.
        pub image_recipe_arn: pulumi_wasm_rust::Output<String>,
        pub image_scanning_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImagePipelineImageScanningConfiguration,
            >,
        >,
        /// List of an object with image tests configuration.
        pub image_tests_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImagePipelineImageTestsConfiguration,
            >,
        >,
        /// ARN of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the image pipeline.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Platform of the image pipeline.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// List of an object with schedule settings.
        pub schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImagePipelineSchedule>,
        >,
        /// Status of the image pipeline.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the image pipeline.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetImagePipelineArgs) -> GetImagePipelineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let tags_binding = args.tags.get_inner();
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageRecipeArn".into(),
                },
                register_interface::ResultField {
                    name: "imageScanningConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "imageTestsConfigurations".into(),
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
                    name: "schedules".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImagePipelineResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageRecipeArn").unwrap(),
            ),
            image_scanning_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageScanningConfigurations").unwrap(),
            ),
            image_tests_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTestsConfigurations").unwrap(),
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
            schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedules").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
