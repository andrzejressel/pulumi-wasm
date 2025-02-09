/// Provides an Elastic Container Registry Repository.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = repository::create(
///         "foo",
///         RepositoryArgs::builder()
///             .image_scanning_configuration(
///                 RepositoryImageScanningConfiguration::builder()
///                     .scanOnPush(true)
///                     .build_struct(),
///             )
///             .image_tag_mutability("MUTABLE")
///             .name("bar")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Repositories using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/repository:Repository service test-service
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Encryption configuration for the repository. See below for schema.
        #[builder(into, default)]
        pub encryption_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ecr::RepositoryEncryptionConfiguration>>,
        >,
        /// If `true`, will delete the repository even if it contains images.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block that defines image scanning configuration for the repository. By default, image scanning must be manually triggered. See the [ECR User Guide](https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html) for more information about image scanning.
        #[builder(into, default)]
        pub image_scanning_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ecr::RepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        #[builder(into, default)]
        pub image_tag_mutability: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the repository.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Encryption configuration for the repository. See below for schema.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ecr::RepositoryEncryptionConfiguration>>,
        >,
        /// If `true`, will delete the repository even if it contains images.
        /// Defaults to `false`.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block that defines image scanning configuration for the repository. By default, image scanning must be manually triggered. See the [ECR User Guide](https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html) for more information about image scanning.
        pub image_scanning_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::ecr::RepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        pub image_tag_mutability: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the repository.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// The URL of the repository (in the form `aws_account_id.dkr.ecr.region.amazonaws.com/repositoryName`).
        pub repository_url: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encryption_configurations_binding = args
            .encryption_configurations
            .get_output(context);
        let force_delete_binding = args.force_delete.get_output(context);
        let image_scanning_configuration_binding = args
            .image_scanning_configuration
            .get_output(context);
        let image_tag_mutability_binding = args.image_tag_mutability.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecr/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfigurations".into(),
                    value: encryption_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDelete".into(),
                    value: force_delete_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageScanningConfiguration".into(),
                    value: image_scanning_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageTagMutability".into(),
                    value: image_tag_mutability_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryResult {
            arn: o.get_field("arn"),
            encryption_configurations: o.get_field("encryptionConfigurations"),
            force_delete: o.get_field("forceDelete"),
            image_scanning_configuration: o.get_field("imageScanningConfiguration"),
            image_tag_mutability: o.get_field("imageTagMutability"),
            name: o.get_field("name"),
            registry_id: o.get_field("registryId"),
            repository_url: o.get_field("repositoryUrl"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
