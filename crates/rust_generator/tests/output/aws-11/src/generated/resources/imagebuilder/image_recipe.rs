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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image_recipe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageRecipeArgs {
        /// Configuration block(s) with block device mappings for the image recipe. Detailed below.
        #[builder(into, default)]
        pub block_device_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::imagebuilder::ImageRecipeBlockDeviceMapping>>,
        >,
        /// Ordered configuration block(s) with components for the image recipe. Detailed below.
        #[builder(into)]
        pub components: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::imagebuilder::ImageRecipeComponent>,
        >,
        /// Description of the image recipe.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the image recipe.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The image recipe uses this image as a base from which to build your customized image. The value can be the base image ARN or an AMI ID.
        #[builder(into)]
        pub parent_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for the Systems Manager Agent installed by default by Image Builder. Detailed below.
        #[builder(into, default)]
        pub systems_manager_agent: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::ImageRecipeSystemsManagerAgent>,
        >,
        /// Key-value map of resource tags for the image recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Base64 encoded user data. Use this to provide commands or a command script to run when you launch your build instance.
        #[builder(into, default)]
        pub user_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The semantic version of the image recipe, which specifies the version in the following format, with numeric values in each position to indicate a specific version: major.minor.patch. For example: 1.0.0.
        ///
        /// The following attributes are optional:
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The working directory to be used during build and test workflows.
        #[builder(into, default)]
        pub working_directory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ImageRecipeResult {
        /// Amazon Resource Name (ARN) of the image recipe.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block(s) with block device mappings for the image recipe. Detailed below.
        pub block_device_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::imagebuilder::ImageRecipeBlockDeviceMapping>>,
        >,
        /// Ordered configuration block(s) with components for the image recipe. Detailed below.
        pub components: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::ImageRecipeComponent>,
        >,
        /// Date the image recipe was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the image recipe.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the image recipe.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the image recipe.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The image recipe uses this image as a base from which to build your customized image. The value can be the base image ARN or an AMI ID.
        pub parent_image: pulumi_gestalt_rust::Output<String>,
        /// Platform of the image recipe.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the Systems Manager Agent installed by default by Image Builder. Detailed below.
        pub systems_manager_agent: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ImageRecipeSystemsManagerAgent,
        >,
        /// Key-value map of resource tags for the image recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Base64 encoded user data. Use this to provide commands or a command script to run when you launch your build instance.
        pub user_data_base64: pulumi_gestalt_rust::Output<String>,
        /// The semantic version of the image recipe, which specifies the version in the following format, with numeric values in each position to indicate a specific version: major.minor.patch. For example: 1.0.0.
        ///
        /// The following attributes are optional:
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The working directory to be used during build and test workflows.
        pub working_directory: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageRecipeArgs,
    ) -> ImageRecipeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_device_mappings_binding = args
            .block_device_mappings
            .get_output(context);
        let components_binding = args.components.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_image_binding = args.parent_image.get_output(context);
        let systems_manager_agent_binding = args
            .systems_manager_agent
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_data_base64_binding = args.user_data_base64.get_output(context);
        let version_binding = args.version.get_output(context);
        let working_directory_binding = args.working_directory.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/imageRecipe:ImageRecipe".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockDeviceMappings".into(),
                    value: block_device_mappings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "components".into(),
                    value: components_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentImage".into(),
                    value: parent_image_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "systemsManagerAgent".into(),
                    value: systems_manager_agent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userDataBase64".into(),
                    value: user_data_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workingDirectory".into(),
                    value: working_directory_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImageRecipeResult {
            arn: o.get_field("arn"),
            block_device_mappings: o.get_field("blockDeviceMappings"),
            components: o.get_field("components"),
            date_created: o.get_field("dateCreated"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            parent_image: o.get_field("parentImage"),
            platform: o.get_field("platform"),
            systems_manager_agent: o.get_field("systemsManagerAgent"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_data_base64: o.get_field("userDataBase64"),
            version: o.get_field("version"),
            working_directory: o.get_field("workingDirectory"),
        }
    }
}
