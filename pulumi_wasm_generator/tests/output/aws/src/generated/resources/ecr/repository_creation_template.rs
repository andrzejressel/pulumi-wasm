/// Provides an Elastic Container Registry Repository Creation Template.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRepositoryCreationTemplate:
///     type: aws:ecr:RepositoryCreationTemplate
///     name: example
///     properties:
///       prefix: example
///       description: An example template
///       imageTagMutability: IMMUTABLE
///       customRoleArn: arn:aws:iam::123456789012:role/example
///       appliedFors:
///         - PULL_THROUGH_CACHE
///       encryptionConfigurations:
///         - encryptionType: AES256
///       repositoryPolicy: ${example.json}
///       lifecyclePolicy: |
///         {
///           "rules": [
///             {
///               "rulePriority": 1,
///               "description": "Expire images older than 14 days",
///               "selection": {
///                 "tagStatus": "untagged",
///                 "countType": "sinceImagePushed",
///                 "countUnit": "days",
///                 "countNumber": 14
///               },
///               "action": {
///                 "type": "expire"
///               }
///             }
///           ]
///         }
///       resourceTags:
///         Foo: Bar
/// variables:
///   example:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
/// Using `pulumi import`, import the ECR Repository Creating Templates using the `prefix`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/repositoryCreationTemplate:RepositoryCreationTemplate example example
/// ```
pub mod repository_creation_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryCreationTemplateArgs {
        /// Which features this template applies to. Must contain one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        #[builder(into)]
        pub applied_fors: pulumi_wasm_rust::Output<Vec<String>>,
        /// A custom IAM role to use for repository creation. Required if using repository tags or KMS encryption.
        #[builder(into, default)]
        pub custom_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for this template.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Encryption configuration for any created repositories. See below for schema.
        #[builder(into, default)]
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::ecr::RepositoryCreationTemplateEncryptionConfiguration,
                >,
            >,
        >,
        /// The tag mutability setting for any created repositories. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        #[builder(into, default)]
        pub image_tag_mutability: pulumi_wasm_rust::Output<Option<String>>,
        /// The lifecycle policy document to apply to any created repositories. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `lifecycle_policy` argument.
        #[builder(into, default)]
        pub lifecycle_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The repository name prefix to match against. Use `ROOT` to match any prefix that doesn't explicitly match another template.
        #[builder(into)]
        pub prefix: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub repository_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to any created repositories.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryCreationTemplateResult {
        /// Which features this template applies to. Must contain one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        pub applied_fors: pulumi_wasm_rust::Output<Vec<String>>,
        /// A custom IAM role to use for repository creation. Required if using repository tags or KMS encryption.
        pub custom_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for this template.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Encryption configuration for any created repositories. See below for schema.
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::ecr::RepositoryCreationTemplateEncryptionConfiguration,
                >,
            >,
        >,
        /// The tag mutability setting for any created repositories. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        pub image_tag_mutability: pulumi_wasm_rust::Output<Option<String>>,
        /// The lifecycle policy document to apply to any created repositories. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `lifecycle_policy` argument.
        pub lifecycle_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The repository name prefix to match against. Use `ROOT` to match any prefix that doesn't explicitly match another template.
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// The registry ID the repository creation template applies to.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        pub repository_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to any created repositories.
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RepositoryCreationTemplateArgs,
    ) -> RepositoryCreationTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let applied_fors_binding = args.applied_fors.get_inner();
        let custom_role_arn_binding = args.custom_role_arn.get_inner();
        let description_binding = args.description.get_inner();
        let encryption_configurations_binding = args
            .encryption_configurations
            .get_inner();
        let image_tag_mutability_binding = args.image_tag_mutability.get_inner();
        let lifecycle_policy_binding = args.lifecycle_policy.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let repository_policy_binding = args.repository_policy.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/repositoryCreationTemplate:RepositoryCreationTemplate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appliedFors".into(),
                    value: &applied_fors_binding,
                },
                register_interface::ObjectField {
                    name: "customRoleArn".into(),
                    value: &custom_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfigurations".into(),
                    value: &encryption_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "imageTagMutability".into(),
                    value: &image_tag_mutability_binding,
                },
                register_interface::ObjectField {
                    name: "lifecyclePolicy".into(),
                    value: &lifecycle_policy_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryPolicy".into(),
                    value: &repository_policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appliedFors".into(),
                },
                register_interface::ResultField {
                    name: "customRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "imageTagMutability".into(),
                },
                register_interface::ResultField {
                    name: "lifecyclePolicy".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryPolicy".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryCreationTemplateResult {
            applied_fors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appliedFors").unwrap(),
            ),
            custom_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRoleArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigurations").unwrap(),
            ),
            image_tag_mutability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTagMutability").unwrap(),
            ),
            lifecycle_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecyclePolicy").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryPolicy").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
        }
    }
}