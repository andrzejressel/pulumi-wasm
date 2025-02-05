pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// ARN of the image. The suffix can either be specified with wildcards (`x.x.x`) to fetch the latest build version or a full build version (e.g., `2020.11.26/1`) to fetch an exact version.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImage:getImage".into(),
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
        GetImageResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            build_version_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("buildVersionArn"),
            ),
            container_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerRecipeArn"),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            distribution_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("distributionConfigurationArn"),
            ),
            enhanced_image_metadata_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enhancedImageMetadataEnabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_recipe_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageRecipeArn"),
            ),
            image_scanning_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageScanningConfigurations"),
            ),
            image_tests_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageTestsConfigurations"),
            ),
            infrastructure_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("infrastructureConfigurationArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            os_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osVersion"),
            ),
            output_resources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputResources"),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
