/// Provides an Elastic Container Registry Repository.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Encryption configuration for the repository. See below for schema.
        #[builder(into, default)]
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecr::RepositoryEncryptionConfiguration>>,
        >,
        /// If `true`, will delete the repository even if it contains images.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block that defines image scanning configuration for the repository. By default, image scanning must be manually triggered. See the [ECR User Guide](https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html) for more information about image scanning.
        #[builder(into, default)]
        pub image_scanning_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecr::RepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        #[builder(into, default)]
        pub image_tag_mutability: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the repository.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Encryption configuration for the repository. See below for schema.
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ecr::RepositoryEncryptionConfiguration>>,
        >,
        /// If `true`, will delete the repository even if it contains images.
        /// Defaults to `false`.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block that defines image scanning configuration for the repository. By default, image scanning must be manually triggered. See the [ECR User Guide](https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html) for more information about image scanning.
        pub image_scanning_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::ecr::RepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        pub image_tag_mutability: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the repository.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// The URL of the repository (in the form `aws_account_id.dkr.ecr.region.amazonaws.com/repositoryName`).
        pub repository_url: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RepositoryArgs) -> RepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_configurations_binding = args
            .encryption_configurations
            .get_inner();
        let force_delete_binding = args.force_delete.get_inner();
        let image_scanning_configuration_binding = args
            .image_scanning_configuration
            .get_inner();
        let image_tag_mutability_binding = args.image_tag_mutability.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/repository:Repository".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "forceDelete".into(),
                },
                register_interface::ResultField {
                    name: "imageScanningConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "imageTagMutability".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryUrl".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigurations").unwrap(),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDelete").unwrap(),
            ),
            image_scanning_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageScanningConfiguration").unwrap(),
            ),
            image_tag_mutability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTagMutability").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryUrl").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}