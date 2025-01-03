/// Provides a CodeCommit Repository Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder()
///             .description("This is the Sample App Repository")
///             .repository_name("MyTestRepository")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS KMS Customer Managed Keys (CMK)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder()
///             .description("This is the Sample App Repository")
///             .kms_key_id("${testKey.arn}")
///             .repository_name("MyTestRepository")
///             .build_struct(),
///     );
///     let testKey = key::create(
///         "testKey",
///         KeyArgs::builder().deletion_window_in_days(7).description("test").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeCommit repository using repository name. For example:
///
/// ```sh
/// $ pulumi import aws:codecommit/repository:Repository imported ExistingRepo
/// ```
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// The default branch of the repository. The branch specified here needs to exist.
        #[builder(into, default)]
        pub default_branch: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the repository. This needs to be less than 1000 characters
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the encryption key. If no key is specified, the default `aws/codecommit` Amazon Web Services managed key is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// The ARN of the repository
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The URL to use for cloning the repository over HTTPS.
        pub clone_url_http: pulumi_wasm_rust::Output<String>,
        /// The URL to use for cloning the repository over SSH.
        pub clone_url_ssh: pulumi_wasm_rust::Output<String>,
        /// The default branch of the repository. The branch specified here needs to exist.
        pub default_branch: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the repository. This needs to be less than 1000 characters
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the encryption key. If no key is specified, the default `aws/codecommit` Amazon Web Services managed key is used.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the repository
        pub repository_id: pulumi_wasm_rust::Output<String>,
        /// The name for the repository. This needs to be less than 100 characters.
        pub repository_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        let default_branch_binding = args.default_branch.get_inner();
        let description_binding = args.description.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let repository_name_binding = args.repository_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecommit/repository:Repository".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultBranch".into(),
                    value: &default_branch_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
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
                    name: "cloneUrlHttp".into(),
                },
                register_interface::ResultField {
                    name: "cloneUrlSsh".into(),
                },
                register_interface::ResultField {
                    name: "defaultBranch".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
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
            clone_url_http: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloneUrlHttp").unwrap(),
            ),
            clone_url_ssh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloneUrlSsh").unwrap(),
            ),
            default_branch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultBranch").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
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
