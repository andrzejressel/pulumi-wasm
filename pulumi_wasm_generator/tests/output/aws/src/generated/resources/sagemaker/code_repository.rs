/// Provides a SageMaker Code Repository resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod code_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeRepositoryArgs {
        /// The name of the Code Repository (must be unique).
        #[builder(into)]
        pub code_repository_name: pulumi_wasm_rust::Output<String>,
        /// Specifies details about the repository. see Git Config details below.
        #[builder(into)]
        pub git_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::CodeRepositoryGitConfig,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CodeRepositoryResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Code Repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Code Repository (must be unique).
        pub code_repository_name: pulumi_wasm_rust::Output<String>,
        /// Specifies details about the repository. see Git Config details below.
        pub git_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::CodeRepositoryGitConfig,
        >,
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
    pub fn create(name: &str, args: CodeRepositoryArgs) -> CodeRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let code_repository_name_binding = args.code_repository_name.get_inner();
        let git_config_binding = args.git_config.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/codeRepository:CodeRepository".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "codeRepositoryName".into(),
                },
                register_interface::ResultField {
                    name: "gitConfig".into(),
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
        CodeRepositoryResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code_repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeRepositoryName").unwrap(),
            ),
            git_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gitConfig").unwrap(),
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
