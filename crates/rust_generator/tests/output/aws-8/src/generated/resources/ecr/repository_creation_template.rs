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
/// Using `pulumi import`, import the ECR Repository Creating Templates using the `prefix`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/repositoryCreationTemplate:RepositoryCreationTemplate example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_creation_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryCreationTemplateArgs {
        /// Which features this template applies to. Must contain one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        #[builder(into)]
        pub applied_fors: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A custom IAM role to use for repository creation. Required if using repository tags or KMS encryption.
        #[builder(into, default)]
        pub custom_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for this template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Encryption configuration for any created repositories. See below for schema.
        #[builder(into, default)]
        pub encryption_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::ecr::RepositoryCreationTemplateEncryptionConfiguration,
                >,
            >,
        >,
        /// The tag mutability setting for any created repositories. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        #[builder(into, default)]
        pub image_tag_mutability: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The lifecycle policy document to apply to any created repositories. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `lifecycle_policy` argument.
        #[builder(into, default)]
        pub lifecycle_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The repository name prefix to match against. Use `ROOT` to match any prefix that doesn't explicitly match another template.
        #[builder(into)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub repository_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to any created repositories.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryCreationTemplateResult {
        /// Which features this template applies to. Must contain one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        pub applied_fors: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A custom IAM role to use for repository creation. Required if using repository tags or KMS encryption.
        pub custom_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for this template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Encryption configuration for any created repositories. See below for schema.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::ecr::RepositoryCreationTemplateEncryptionConfiguration,
                >,
            >,
        >,
        /// The tag mutability setting for any created repositories. Must be one of: `MUTABLE` or `IMMUTABLE`. Defaults to `MUTABLE`.
        pub image_tag_mutability: pulumi_gestalt_rust::Output<Option<String>>,
        /// The lifecycle policy document to apply to any created repositories. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `lifecycle_policy` argument.
        pub lifecycle_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The repository name prefix to match against. Use `ROOT` to match any prefix that doesn't explicitly match another template.
        pub prefix: pulumi_gestalt_rust::Output<String>,
        /// The registry ID the repository creation template applies to.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        pub repository_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to any created repositories.
        pub resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryCreationTemplateArgs,
    ) -> RepositoryCreationTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let applied_fors_binding = args.applied_fors.get_output(context);
        let custom_role_arn_binding = args.custom_role_arn.get_output(context);
        let description_binding = args.description.get_output(context);
        let encryption_configurations_binding = args
            .encryption_configurations
            .get_output(context);
        let image_tag_mutability_binding = args.image_tag_mutability.get_output(context);
        let lifecycle_policy_binding = args.lifecycle_policy.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let repository_policy_binding = args.repository_policy.get_output(context);
        let resource_tags_binding = args.resource_tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecr/repositoryCreationTemplate:RepositoryCreationTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appliedFors".into(),
                    value: &applied_fors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customRoleArn".into(),
                    value: &custom_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfigurations".into(),
                    value: &encryption_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageTagMutability".into(),
                    value: &image_tag_mutability_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifecyclePolicy".into(),
                    value: &lifecycle_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryPolicy".into(),
                    value: &repository_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryCreationTemplateResult {
            applied_fors: o.get_field("appliedFors"),
            custom_role_arn: o.get_field("customRoleArn"),
            description: o.get_field("description"),
            encryption_configurations: o.get_field("encryptionConfigurations"),
            image_tag_mutability: o.get_field("imageTagMutability"),
            lifecycle_policy: o.get_field("lifecyclePolicy"),
            prefix: o.get_field("prefix"),
            registry_id: o.get_field("registryId"),
            repository_policy: o.get_field("repositoryPolicy"),
            resource_tags: o.get_field("resourceTags"),
        }
    }
}
