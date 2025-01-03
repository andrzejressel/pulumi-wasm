pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// ARN of the image. The suffix can either be specified with wildcards (`x.x.x`) to fetch the latest build version or a full build version (e.g., `2020.11.26/1`) to fetch an exact version.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Build version ARN of the image. This will always have the `#.#.#/#` suffix.
        pub build_version_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the container recipe.
        pub container_recipe_arn: pulumi_wasm_rust::Output<String>,
        /// Date the image was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// ARN of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Whether additional information about the image being created is collected.
        pub enhanced_image_metadata_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the image recipe.
        pub image_recipe_arn: pulumi_wasm_rust::Output<String>,
        /// List of an object with image scanning configuration fields.
        pub image_scanning_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageImageScanningConfiguration,
            >,
        >,
        /// List of an object with image tests configuration.
        pub image_tests_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageImageTestsConfiguration,
            >,
        >,
        /// ARN of the Image Builder Infrastructure Configuration.
        pub infrastructure_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the AMI.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Operating System version of the image.
        pub os_version: pulumi_wasm_rust::Output<String>,
        /// List of objects with resources created by the image.
        pub output_resources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImageOutputResource>,
        >,
        /// Platform of the image.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the image.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Version of the image.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetImageArgs) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImage:getImage".into(),
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
                    name: "buildVersionArn".into(),
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
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImageResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            build_version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildVersionArn").unwrap(),
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
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
