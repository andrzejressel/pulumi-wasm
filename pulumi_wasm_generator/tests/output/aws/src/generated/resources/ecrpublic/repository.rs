/// Provides a Public Elastic Container Registry Repository.
///
/// > **NOTE:** This resource can only be used in the `us-east-1` region.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:ecrpublic:Repository
///     properties:
///       repositoryName: bar
///       catalogData:
///         aboutText: About Text
///         architectures:
///           - ARM
///         description: Description
///         logoImageBlob:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: ${png}
///             return: result
///         operatingSystems:
///           - Linux
///         usageText: Usage Text
///       tags:
///         env: production
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Public Repositories using the `repository_name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecrpublic/repository:Repository example example
/// ```
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Catalog data configuration for the repository. See below for schema.
        #[builder(into, default)]
        pub catalog_data: pulumi_wasm_rust::Output<
            Option<super::super::types::ecrpublic::RepositoryCatalogData>,
        >,
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the repository.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Catalog data configuration for the repository. See below for schema.
        pub catalog_data: pulumi_wasm_rust::Output<
            Option<super::super::types::ecrpublic::RepositoryCatalogData>,
        >,
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Name of the repository.
        pub repository_name: pulumi_wasm_rust::Output<String>,
        /// The URI of the repository.
        pub repository_uri: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        let catalog_data_binding = args.catalog_data.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let repository_name_binding = args.repository_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecrpublic/repository:Repository".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogData".into(),
                    value: &catalog_data_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
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
                    name: "catalogData".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
                register_interface::ResultField {
                    name: "repositoryUri".into(),
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
            catalog_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogData").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
            repository_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryUri").unwrap(),
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
