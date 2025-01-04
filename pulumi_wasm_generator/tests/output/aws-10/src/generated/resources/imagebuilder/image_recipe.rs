/// Manages an Image Builder Image Recipe.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:imagebuilder:ImageRecipe
///     properties:
///       blockDeviceMappings:
///         - deviceName: /dev/xvdb
///           ebs:
///             deleteOnTermination: true
///             volumeSize: 100
///             volumeType: gp2
///       components:
///         - componentArn: ${exampleAwsImagebuilderComponent.arn}
///           parameters:
///             - name: Parameter1
///               value: Value1
///             - name: Parameter2
///               value: Value2
///       name: example
///       parentImage: arn:${current.partition}:imagebuilder:${currentAwsRegion.name}:aws:image/amazon-linux-2-x86/x.x.x
///       version: 1.0.0
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_image_recipe` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/imageRecipe:ImageRecipe example arn:aws:imagebuilder:us-east-1:123456789012:image-recipe/example/1.0.0
/// ```
pub mod image_recipe {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageRecipeArgs {
        /// Configuration block(s) with block device mappings for the image recipe. Detailed below.
        #[builder(into, default)]
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::ImageRecipeBlockDeviceMapping>>,
        >,
        /// Ordered configuration block(s) with components for the image recipe. Detailed below.
        #[builder(into)]
        pub components: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::ImageRecipeComponent>,
        >,
        /// Description of the image recipe.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the image recipe.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The image recipe uses this image as a base from which to build your customized image. The value can be the base image ARN or an AMI ID.
        #[builder(into)]
        pub parent_image: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Systems Manager Agent installed by default by Image Builder. Detailed below.
        #[builder(into, default)]
        pub systems_manager_agent: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::ImageRecipeSystemsManagerAgent>,
        >,
        /// Key-value map of resource tags for the image recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Base64 encoded user data. Use this to provide commands or a command script to run when you launch your build instance.
        #[builder(into, default)]
        pub user_data_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// The semantic version of the image recipe, which specifies the version in the following format, with numeric values in each position to indicate a specific version: major.minor.patch. For example: 1.0.0.
        ///
        /// The following attributes are optional:
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
        /// The working directory to be used during build and test workflows.
        #[builder(into, default)]
        pub working_directory: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ImageRecipeResult {
        /// Amazon Resource Name (ARN) of the image recipe.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with block device mappings for the image recipe. Detailed below.
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::ImageRecipeBlockDeviceMapping>>,
        >,
        /// Ordered configuration block(s) with components for the image recipe. Detailed below.
        pub components: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::ImageRecipeComponent>,
        >,
        /// Date the image recipe was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Description of the image recipe.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the image recipe.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the image recipe.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The image recipe uses this image as a base from which to build your customized image. The value can be the base image ARN or an AMI ID.
        pub parent_image: pulumi_wasm_rust::Output<String>,
        /// Platform of the image recipe.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Systems Manager Agent installed by default by Image Builder. Detailed below.
        pub systems_manager_agent: pulumi_wasm_rust::Output<
            super::super::types::imagebuilder::ImageRecipeSystemsManagerAgent,
        >,
        /// Key-value map of resource tags for the image recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Base64 encoded user data. Use this to provide commands or a command script to run when you launch your build instance.
        pub user_data_base64: pulumi_wasm_rust::Output<String>,
        /// The semantic version of the image recipe, which specifies the version in the following format, with numeric values in each position to indicate a specific version: major.minor.patch. For example: 1.0.0.
        ///
        /// The following attributes are optional:
        pub version: pulumi_wasm_rust::Output<String>,
        /// The working directory to be used during build and test workflows.
        pub working_directory: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ImageRecipeArgs) -> ImageRecipeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let block_device_mappings_binding = args.block_device_mappings.get_inner();
        let components_binding = args.components.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let parent_image_binding = args.parent_image.get_inner();
        let systems_manager_agent_binding = args.systems_manager_agent.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_data_base64_binding = args.user_data_base64.get_inner();
        let version_binding = args.version.get_inner();
        let working_directory_binding = args.working_directory.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/imageRecipe:ImageRecipe".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockDeviceMappings".into(),
                    value: &block_device_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "components".into(),
                    value: &components_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentImage".into(),
                    value: &parent_image_binding,
                },
                register_interface::ObjectField {
                    name: "systemsManagerAgent".into(),
                    value: &systems_manager_agent_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userDataBase64".into(),
                    value: &user_data_base64_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "workingDirectory".into(),
                    value: &working_directory_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "blockDeviceMappings".into(),
                },
                register_interface::ResultField {
                    name: "components".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "parentImage".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "systemsManagerAgent".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userDataBase64".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "workingDirectory".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ImageRecipeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            block_device_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockDeviceMappings").unwrap(),
            ),
            components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("components").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            parent_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentImage").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            systems_manager_agent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemsManagerAgent").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userDataBase64").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            working_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workingDirectory").unwrap(),
            ),
        }
    }
}
