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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let encryption_configurations_binding_1 = args
            .encryption_configurations
            .get_output(context);
        let encryption_configurations_binding = encryption_configurations_binding_1
            .get_inner();
        let force_delete_binding_1 = args.force_delete.get_output(context);
        let force_delete_binding = force_delete_binding_1.get_inner();
        let image_scanning_configuration_binding_1 = args
            .image_scanning_configuration
            .get_output(context);
        let image_scanning_configuration_binding = image_scanning_configuration_binding_1
            .get_inner();
        let image_tag_mutability_binding_1 = args
            .image_tag_mutability
            .get_output(context);
        let image_tag_mutability_binding = image_tag_mutability_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionConfigurations".into(),
                    value: &encryption_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "imageScanningConfiguration".into(),
                    value: &image_scanning_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "imageTagMutability".into(),
                    value: &image_tag_mutability_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            encryption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfigurations"),
            ),
            force_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            image_scanning_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageScanningConfiguration"),
            ),
            image_tag_mutability: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageTagMutability"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            repository_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryUrl"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
