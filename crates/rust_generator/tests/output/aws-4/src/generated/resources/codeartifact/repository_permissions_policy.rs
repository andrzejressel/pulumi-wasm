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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_permissions_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryPermissionsPolicyArgs {
        /// The name of the domain on which to set the resource policy.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        #[builder(into)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        #[builder(into, default)]
        pub policy_revision: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the repository to set the resource policy on.
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryPermissionsPolicyResult {
        /// The name of the domain on which to set the resource policy.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_gestalt_rust::Output<String>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        pub policy_document: pulumi_gestalt_rust::Output<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        pub policy_revision: pulumi_gestalt_rust::Output<String>,
        /// The name of the repository to set the resource policy on.
        pub repository: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the resource associated with the resource policy.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryPermissionsPolicyArgs,
    ) -> RepositoryPermissionsPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let domain_owner_binding = args.domain_owner.get_output(context);
        let policy_document_binding = args.policy_document.get_output(context);
        let policy_revision_binding = args.policy_revision.get_output(context);
        let repository_binding = args.repository.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codeartifact/repositoryPermissionsPolicy:RepositoryPermissionsPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainOwner".into(),
                    value: domain_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: policy_document_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyRevision".into(),
                    value: policy_revision_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repository".into(),
                    value: repository_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryPermissionsPolicyResult {
            domain: o.get_field("domain"),
            domain_owner: o.get_field("domainOwner"),
            policy_document: o.get_field("policyDocument"),
            policy_revision: o.get_field("policyRevision"),
            repository: o.get_field("repository"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
