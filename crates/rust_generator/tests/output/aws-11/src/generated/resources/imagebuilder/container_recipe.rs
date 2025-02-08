/// Manages an Image Builder Container Recipe.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = container_recipe::create(
///         "example",
///         ContainerRecipeArgs::builder()
///             .components(
///                 vec![
///                     ContainerRecipeComponent::builder()
///                     .componentArn("${exampleAwsImagebuilderComponent.arn}")
///                     .parameters(vec![ContainerRecipeComponentParameter::builder()
///                     .name("Parameter1").value("Value1").build_struct(),
///                     ContainerRecipeComponentParameter::builder().name("Parameter2")
///                     .value("Value2").build_struct(),]).build_struct(),
///                 ],
///             )
///             .container_type("DOCKER")
///             .dockerfile_template_data(
///                 "FROM {{{ imagebuilder:parentImage }}}\n{{{ imagebuilder:environments }}}\n{{{ imagebuilder:components }}}",
///             )
///             .name("example")
///             .parent_image(
///                 "arn:aws:imagebuilder:eu-central-1:aws:image/amazon-linux-x86-latest/x.x.x",
///             )
///             .target_repository(
///                 ContainerRecipeTargetRepository::builder()
///                     .repositoryName("${exampleAwsEcrRepository.name}")
///                     .service("ECR")
///                     .build_struct(),
///             )
///             .version("1.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_container_recipe` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/containerRecipe:ContainerRecipe example arn:aws:imagebuilder:us-east-1:123456789012:container-recipe/example/1.0.0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod container_recipe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerRecipeArgs {
        /// Ordered configuration block(s) with components for the container recipe. Detailed below.
        #[builder(into)]
        pub components: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::imagebuilder::ContainerRecipeComponent>,
        >,
        /// The type of the container to create. Valid values: `DOCKER`.
        #[builder(into)]
        pub container_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the container recipe.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Dockerfile template used to build the image as an inline data blob.
        #[builder(into, default)]
        pub dockerfile_template_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon S3 URI for the Dockerfile that will be used to build the container image.
        #[builder(into, default)]
        pub dockerfile_template_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block used to configure an instance for building and testing container images. Detailed below.
        #[builder(into, default)]
        pub instance_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::imagebuilder::ContainerRecipeInstanceConfiguration,
            >,
        >,
        /// The KMS key used to encrypt the container image.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the container recipe.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The base image for the container recipe.
        #[builder(into)]
        pub parent_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the operating system platform when you use a custom base image.
        #[builder(into, default)]
        pub platform_override: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the container recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The destination repository for the container image. Detailed below.
        #[builder(into)]
        pub target_repository: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::imagebuilder::ContainerRecipeTargetRepository,
        >,
        /// Version of the container recipe.
        ///
        /// The following attributes are optional:
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The working directory to be used during build and test workflows.
        #[builder(into, default)]
        pub working_directory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerRecipeResult {
        /// (Required) Amazon Resource Name (ARN) of the container recipe.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Ordered configuration block(s) with components for the container recipe. Detailed below.
        pub components: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::ContainerRecipeComponent>,
        >,
        /// The type of the container to create. Valid values: `DOCKER`.
        pub container_type: pulumi_gestalt_rust::Output<String>,
        /// Date the container recipe was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// The description of the container recipe.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Dockerfile template used to build the image as an inline data blob.
        pub dockerfile_template_data: pulumi_gestalt_rust::Output<String>,
        /// The Amazon S3 URI for the Dockerfile that will be used to build the container image.
        pub dockerfile_template_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// A flag that indicates if the target container is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Configuration block used to configure an instance for building and testing container images. Detailed below.
        pub instance_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::imagebuilder::ContainerRecipeInstanceConfiguration,
            >,
        >,
        /// The KMS key used to encrypt the container image.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the container recipe.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the container recipe.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The base image for the container recipe.
        pub parent_image: pulumi_gestalt_rust::Output<String>,
        /// Platform of the container recipe.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Specifies the operating system platform when you use a custom base image.
        pub platform_override: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the container recipe. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The destination repository for the container image. Detailed below.
        pub target_repository: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ContainerRecipeTargetRepository,
        >,
        /// Version of the container recipe.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ContainerRecipeArgs,
    ) -> ContainerRecipeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let components_binding = args.components.get_output(context).get_inner();
        let container_type_binding = args.container_type.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dockerfile_template_data_binding = args
            .dockerfile_template_data
            .get_output(context)
            .get_inner();
        let dockerfile_template_uri_binding = args
            .dockerfile_template_uri
            .get_output(context)
            .get_inner();
        let instance_configuration_binding = args
            .instance_configuration
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_image_binding = args.parent_image.get_output(context).get_inner();
        let platform_override_binding = args
            .platform_override
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_repository_binding = args
            .target_repository
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let working_directory_binding = args
            .working_directory
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/containerRecipe:ContainerRecipe".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "components".into(),
                    value: &components_binding,
                },
                register_interface::ObjectField {
                    name: "containerType".into(),
                    value: &container_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dockerfileTemplateData".into(),
                    value: &dockerfile_template_data_binding,
                },
                register_interface::ObjectField {
                    name: "dockerfileTemplateUri".into(),
                    value: &dockerfile_template_uri_binding,
                },
                register_interface::ObjectField {
                    name: "instanceConfiguration".into(),
                    value: &instance_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
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
                    name: "platformOverride".into(),
                    value: &platform_override_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetRepository".into(),
                    value: &target_repository_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ContainerRecipeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            components: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("components"),
            ),
            container_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerType"),
            ),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dockerfile_template_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerfileTemplateData"),
            ),
            dockerfile_template_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerfileTemplateUri"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            instance_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceConfiguration"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            parent_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentImage"),
            ),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            platform_override: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformOverride"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetRepository"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            working_directory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workingDirectory"),
            ),
        }
    }
}
