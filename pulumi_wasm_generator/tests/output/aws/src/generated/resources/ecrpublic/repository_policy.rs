/// Provides an Elastic Container Registry Public Repository Policy.
///
/// Note that currently only one policy may be applied to a repository.
///
/// > **NOTE:** This resource can only be used in the `us-east-1` region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["ecr:GetDownloadUrlForLayer", "ecr:BatchGetImage",
///                     "ecr:BatchCheckLayerAvailability", "ecr:PutImage",
///                     "ecr:InitiateLayerUpload", "ecr:UploadLayerPart",
///                     "ecr:CompleteLayerUpload", "ecr:DescribeRepositories",
///                     "ecr:GetRepositoryPolicy", "ecr:ListImages", "ecr:DeleteRepository",
///                     "ecr:BatchDeleteImage", "ecr:SetRepositoryPolicy",
///                     "ecr:DeleteRepositoryPolicy",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["123456789012",]). type ("AWS").build_struct(),])
///                     .sid("new policy").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleRepository = repository::create(
///         "exampleRepository",
///         RepositoryArgs::builder().repository_name("example").build_struct(),
///     );
///     let exampleRepositoryPolicy = repository_policy::create(
///         "exampleRepositoryPolicy",
///         RepositoryPolicyArgs::builder()
///             .policy("${example.json}")
///             .repository_name("${exampleRepository.repositoryName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Public Repository Policy using the repository name. For example:
///
/// ```sh
/// $ pulumi import aws:ecrpublic/repositoryPolicy:RepositoryPolicy example example
/// ```
pub mod repository_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryPolicyArgs {
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Name of the repository to apply the policy.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryPolicyResult {
        /// The policy document. This is a JSON formatted string.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Name of the repository to apply the policy.
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RepositoryPolicyArgs) -> RepositoryPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let repository_name_binding = args.repository_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecrpublic/repositoryPolicy:RepositoryPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryPolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
        }
    }
}