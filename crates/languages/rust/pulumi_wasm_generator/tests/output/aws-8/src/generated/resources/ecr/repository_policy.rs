/// Provides an Elastic Container Registry Repository Policy.
///
/// Note that currently only one policy may be applied to a repository.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRepository:
///     type: aws:ecr:Repository
///     name: example
///     properties:
///       name: example-repo
///   exampleRepositoryPolicy:
///     type: aws:ecr:RepositoryPolicy
///     name: example
///     properties:
///       repository: ${exampleRepository.name}
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
/// Using `pulumi import`, import ECR Repository Policy using the repository name. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/repositoryPolicy:RepositoryPolicy example example
/// ```
pub mod repository_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryPolicyArgs {
        /// The policy document. This is a JSON formatted string.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the repository to apply the policy.
        #[builder(into)]
        pub repository: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryPolicyResult {
        /// The policy document. This is a JSON formatted string.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Name of the repository to apply the policy.
        pub repository: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RepositoryPolicyArgs,
    ) -> RepositoryPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let repository_binding = args.repository.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/repositoryPolicy:RepositoryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryPolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repository"),
            ),
        }
    }
}
