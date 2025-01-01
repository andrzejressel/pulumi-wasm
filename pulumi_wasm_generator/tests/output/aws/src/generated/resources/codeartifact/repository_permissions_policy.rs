/// Provides a CodeArtifact Repostory Permissions Policy Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       description: domain key
///   exampleDomain:
///     type: aws:codeartifact:Domain
///     name: example
///     properties:
///       domain: example
///       encryptionKey: ${exampleKey.arn}
///   exampleRepository:
///     type: aws:codeartifact:Repository
///     name: example
///     properties:
///       repository: example
///       domain: ${exampleDomain.domain}
///   exampleRepositoryPermissionsPolicy:
///     type: aws:codeartifact:RepositoryPermissionsPolicy
///     name: example
///     properties:
///       repository: ${exampleRepository.repository}
///       domain: ${exampleDomain.domain}
///       policyDocument: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - codeartifact:ReadFromRepository
///             resources:
///               - ${exampleRepository.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Repository Permissions Policies using the CodeArtifact Repository ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/repositoryPermissionsPolicy:RepositoryPermissionsPolicy example arn:aws:codeartifact:us-west-2:012345678912:repository/tf-acc-test-6968272603913957763/tf-acc-test-6968272603913957763
/// ```
pub mod repository_permissions_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryPermissionsPolicyArgs {
        /// The name of the domain on which to set the resource policy.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        #[builder(into)]
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        #[builder(into, default)]
        pub policy_revision: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the repository to set the resource policy on.
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryPermissionsPolicyResult {
        /// The name of the domain on which to set the resource policy.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_wasm_rust::Output<String>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        pub policy_revision: pulumi_wasm_rust::Output<String>,
        /// The name of the repository to set the resource policy on.
        pub repository: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource associated with the resource policy.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RepositoryPermissionsPolicyArgs,
    ) -> RepositoryPermissionsPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_inner();
        let domain_owner_binding = args.domain_owner.get_inner();
        let policy_document_binding = args.policy_document.get_inner();
        let policy_revision_binding = args.policy_revision.get_inner();
        let repository_binding = args.repository.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeartifact/repositoryPermissionsPolicy:RepositoryPermissionsPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding,
                },
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "policyRevision".into(),
                    value: &policy_revision_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "domainOwner".into(),
                },
                register_interface::ResultField {
                    name: "policyDocument".into(),
                },
                register_interface::ResultField {
                    name: "policyRevision".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryPermissionsPolicyResult {
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            domain_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainOwner").unwrap(),
            ),
            policy_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDocument").unwrap(),
            ),
            policy_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyRevision").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
        }
    }
}
