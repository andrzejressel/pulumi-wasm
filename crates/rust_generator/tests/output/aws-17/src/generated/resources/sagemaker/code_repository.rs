/// Provides a SageMaker Code Repository resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = code_repository::create(
///         "example",
///         CodeRepositoryArgs::builder()
///             .code_repository_name("example")
///             .git_config(
///                 CodeRepositoryGitConfig::builder()
///                     .repositoryUrl("https://github.com/github/docs.git")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example with Secret
///
/// ```yaml
/// resources:
///   example:
///     type: aws:secretsmanager:Secret
///     properties:
///       name: example
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${example.id}
///       secretString:
///         fn::toJSON:
///           username: example
///           password: example
///   exampleCodeRepository:
///     type: aws:sagemaker:CodeRepository
///     name: example
///     properties:
///       codeRepositoryName: example
///       gitConfig:
///         repositoryUrl: https://github.com/github/docs.git
///         secretArn: ${example.arn}
///     options:
///       dependsOn:
///         - ${exampleSecretVersion}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Code Repositories using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/codeRepository:CodeRepository test_code_repository my-code-repo
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod code_repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeRepositoryArgs {
        /// The name of the Code Repository (must be unique).
        #[builder(into)]
        pub code_repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies details about the repository. see Git Config details below.
        #[builder(into)]
        pub git_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::CodeRepositoryGitConfig,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CodeRepositoryResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Code Repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Code Repository (must be unique).
        pub code_repository_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies details about the repository. see Git Config details below.
        pub git_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::CodeRepositoryGitConfig,
        >,
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
        args: CodeRepositoryArgs,
    ) -> CodeRepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let code_repository_name_binding = args
            .code_repository_name
            .get_output(context)
            .get_inner();
        let git_config_binding = args.git_config.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/codeRepository:CodeRepository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "codeRepositoryName".into(),
                    value: &code_repository_name_binding,
                },
                register_interface::ObjectField {
                    name: "gitConfig".into(),
                    value: &git_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CodeRepositoryResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            code_repository_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("codeRepositoryName"),
            ),
            git_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gitConfig"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
