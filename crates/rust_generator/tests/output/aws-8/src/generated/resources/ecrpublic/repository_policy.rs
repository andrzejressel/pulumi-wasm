/// Provides an Elastic Container Registry Public Repository Policy.
///
/// Note that currently only one policy may be applied to a repository.
///
/// > **NOTE:** This resource can only be used in the `us-east-1` region.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRepository:
///     type: aws:ecrpublic:Repository
///     name: example
///     properties:
///       repositoryName: example
///   exampleRepositoryPolicy:
///     type: aws:ecrpublic:RepositoryPolicy
///     name: example
///     properties:
///       repositoryName: ${exampleRepository.repositoryName}
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: new policy
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '123456789012'
///             actions:
///               - ecr:GetDownloadUrlForLayer
///               - ecr:BatchGetImage
///               - ecr:BatchCheckLayerAvailability
///               - ecr:PutImage
///               - ecr:InitiateLayerUpload
///               - ecr:UploadLayerPart
///               - ecr:CompleteLayerUpload
///               - ecr:DescribeRepositories
///               - ecr:GetRepositoryPolicy
///               - ecr:ListImages
///               - ecr:DeleteRepository
///               - ecr:BatchDeleteImage
///               - ecr:SetRepositoryPolicy
///               - ecr:DeleteRepositoryPolicy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Public Repository Policy using the repository name. For example:
///
/// ```sh
/// $ pulumi import aws:ecrpublic/repositoryPolicy:RepositoryPolicy example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryPolicyArgs {
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the repository to apply the policy.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryPolicyResult {
        /// The policy document. This is a JSON formatted string.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the repository to apply the policy.
        pub repository_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryPolicyArgs,
    ) -> RepositoryPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let repository_name_binding = args.repository_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecrpublic/repositoryPolicy:RepositoryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryName".into(),
                    value: repository_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryPolicyResult {
            policy: o.get_field("policy"),
            registry_id: o.get_field("registryId"),
            repository_name: o.get_field("repositoryName"),
        }
    }
}
