/// Provides a CodeCommit Repository Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// The default branch of the repository. The branch specified here needs to exist.
        #[builder(into, default)]
        pub default_branch: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the repository. This needs to be less than 1000 characters
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the encryption key. If no key is specified, the default `aws/codecommit` Amazon Web Services managed key is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// The ARN of the repository
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The URL to use for cloning the repository over HTTPS.
        pub clone_url_http: pulumi_gestalt_rust::Output<String>,
        /// The URL to use for cloning the repository over SSH.
        pub clone_url_ssh: pulumi_gestalt_rust::Output<String>,
        /// The default branch of the repository. The branch specified here needs to exist.
        pub default_branch: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the repository. This needs to be less than 1000 characters
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the encryption key. If no key is specified, the default `aws/codecommit` Amazon Web Services managed key is used.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the repository
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        /// The name for the repository. This needs to be less than 100 characters.
        pub repository_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        let default_branch_binding_1 = args.default_branch.get_output(context);
        let default_branch_binding = default_branch_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let kms_key_id_binding_1 = args.kms_key_id.get_output(context);
        let kms_key_id_binding = kms_key_id_binding_1.get_inner();
        let repository_name_binding_1 = args.repository_name.get_output(context);
        let repository_name_binding = repository_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecommit/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            clone_url_http: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloneUrlHttp"),
            ),
            clone_url_ssh: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloneUrlSsh"),
            ),
            default_branch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultBranch"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            repository_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
            repository_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
