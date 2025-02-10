/// Provides a resource to manage AWS Secrets Manager secret metadata. To manage secret rotation, see the `aws.secretsmanager.SecretRotation` resource. To manage a secret value, see the `aws.secretsmanager.SecretVersion` resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = secret::create(
///         "example",
///         SecretArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_secretsmanager_secret` using the secret Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:secretsmanager/secret:Secret example arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretArgs {
        /// Description of the secret.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Accepts boolean value to specify whether to overwrite a secret with the same name in the destination Region.
        #[builder(into, default)]
        pub force_overwrite_replica_secret: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ARN or Id of the AWS KMS key to be used to encrypt the secret values in the versions stored in this secret. If you need to reference a CMK in a different account, you can use only the key ARN. If you don't specify this value, then Secrets Manager defaults to using the AWS account's default KMS key (the one named `aws/secretsmanager`). If the default KMS key with that name doesn't yet exist, then AWS Secrets Manager creates it for you automatically the first time.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the new secret. The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: `/_+=.@-` Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.secretsmanager.SecretPolicy`. To delete the `policy`, set it to `"{}"` (an empty JSON document).
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of days that AWS Secrets Manager waits before it can delete the secret. This value can be `0` to force deletion without recovery or range from `7` to `30` days. The default value is `30`.
        #[builder(into, default)]
        pub recovery_window_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block to support secret replication. See details below.
        #[builder(into, default)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::secretsmanager::SecretReplica>>,
        >,
        /// Key-value map of user-defined tags that are attached to the secret. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecretResult {
        /// ARN of the secret.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the secret.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Accepts boolean value to specify whether to overwrite a secret with the same name in the destination Region.
        pub force_overwrite_replica_secret: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN or Id of the AWS KMS key to be used to encrypt the secret values in the versions stored in this secret. If you need to reference a CMK in a different account, you can use only the key ARN. If you don't specify this value, then Secrets Manager defaults to using the AWS account's default KMS key (the one named `aws/secretsmanager`). If the default KMS key with that name doesn't yet exist, then AWS Secrets Manager creates it for you automatically the first time.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Friendly name of the new secret. The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: `/_+=.@-` Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Valid JSON document representing a [resource policy](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-based-policies.html). Removing `policy` from your configuration or setting `policy` to null or an empty string (i.e., `policy = ""`) _will not_ delete the policy since it could have been set by `aws.secretsmanager.SecretPolicy`. To delete the `policy`, set it to `"{}"` (an empty JSON document).
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Number of days that AWS Secrets Manager waits before it can delete the secret. This value can be `0` to force deletion without recovery or range from `7` to `30` days. The default value is `30`.
        pub recovery_window_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration block to support secret replication. See details below.
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::secretsmanager::SecretReplica>,
        >,
        /// Key-value map of user-defined tags that are attached to the secret. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretArgs,
    ) -> SecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let force_overwrite_replica_secret_binding = args
            .force_overwrite_replica_secret
            .get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let recovery_window_in_days_binding = args
            .recovery_window_in_days
            .get_output(context);
        let replicas_binding = args.replicas.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:secretsmanager/secret:Secret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceOverwriteReplicaSecret".into(),
                    value: force_overwrite_replica_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryWindowInDays".into(),
                    value: recovery_window_in_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicas".into(),
                    value: replicas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            force_overwrite_replica_secret: o.get_field("forceOverwriteReplicaSecret"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            policy: o.get_field("policy"),
            recovery_window_in_days: o.get_field("recoveryWindowInDays"),
            replicas: o.get_field("replicas"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
