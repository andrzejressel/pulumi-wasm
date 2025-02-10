/// Manages a KMS multi-Region replica key that uses external key material.
/// See the [AWS KMS Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-import.html) for more information on importing key material into multi-Region keys.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = external_key::create(
///         "primary",
///         ExternalKeyArgs::builder()
///             .deletion_window_in_days(30)
///             .description("Multi-Region primary key")
///             .enabled(true)
///             .key_material_base_64("...")
///             .multi_region(true)
///             .build_struct(),
///     );
///     let replica = replica_external_key::create(
///         "replica",
///         ReplicaExternalKeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Multi-Region replica key")
///             .key_material_base_64("...")
///             .primary_key_arn("${primaryAwsKmsExternal.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS multi-Region replica keys using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/replicaExternalKey:ReplicaExternalKey example 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replica_external_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicaExternalKeyArgs {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A description of the KMS key.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the replica key is enabled. Disabled KMS keys cannot be used in cryptographic operations. Keys pending import can only be `false`. Imported keys default to `true` unless expired.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Base64 encoded 256-bit symmetric encryption key material to import. The KMS key is permanently associated with this key material. The same key material can be [reimported](https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material), but you cannot import different key material.
        #[builder(into, default)]
        pub key_material_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The key policy to attach to the KMS key. If you do not specify a key policy, AWS KMS attaches the [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) to the KMS key.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the multi-Region primary key to replicate. The primary key must be in a different AWS Region of the same AWS Partition. You can create only one replica of a given primary key in each AWS Region.
        #[builder(into)]
        pub primary_key_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the replica key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the key becomes unusable. If not specified, key material does not expire. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub valid_to: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ReplicaExternalKeyResult {
        /// The Amazon Resource Name (ARN) of the replica key. The key ARNs of related multi-Region keys differ only in the Region value.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        pub deletion_window_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A description of the KMS key.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the replica key is enabled. Disabled KMS keys cannot be used in cryptographic operations. Keys pending import can only be `false`. Imported keys default to `true` unless expired.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether the key material expires. Empty when pending key material import, otherwise `KEY_MATERIAL_EXPIRES` or `KEY_MATERIAL_DOES_NOT_EXPIRE`.
        pub expiration_model: pulumi_gestalt_rust::Output<String>,
        /// The key ID of the replica key. Related multi-Region keys have the same key ID.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Base64 encoded 256-bit symmetric encryption key material to import. The KMS key is permanently associated with this key material. The same key material can be [reimported](https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material), but you cannot import different key material.
        pub key_material_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of the replica key.
        pub key_state: pulumi_gestalt_rust::Output<String>,
        /// The [cryptographic operations](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations) for which you can use the KMS key. This is a shared property of multi-Region keys.
        pub key_usage: pulumi_gestalt_rust::Output<String>,
        /// The key policy to attach to the KMS key. If you do not specify a key policy, AWS KMS attaches the [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) to the KMS key.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the multi-Region primary key to replicate. The primary key must be in a different AWS Region of the same AWS Partition. You can create only one replica of a given primary key in each AWS Region.
        pub primary_key_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the replica key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the key becomes unusable. If not specified, key material does not expire. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub valid_to: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicaExternalKeyArgs,
    ) -> ReplicaExternalKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_output(context);
        let deletion_window_in_days_binding = args
            .deletion_window_in_days
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let key_material_base64_binding = args.key_material_base64.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let primary_key_arn_binding = args.primary_key_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let valid_to_binding = args.valid_to.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/replicaExternalKey:ReplicaExternalKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                    value: bypass_policy_lockout_safety_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionWindowInDays".into(),
                    value: deletion_window_in_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyMaterialBase64".into(),
                    value: key_material_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryKeyArn".into(),
                    value: primary_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validTo".into(),
                    value: valid_to_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicaExternalKeyResult {
            arn: o.get_field("arn"),
            bypass_policy_lockout_safety_check: o
                .get_field("bypassPolicyLockoutSafetyCheck"),
            deletion_window_in_days: o.get_field("deletionWindowInDays"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            expiration_model: o.get_field("expirationModel"),
            key_id: o.get_field("keyId"),
            key_material_base64: o.get_field("keyMaterialBase64"),
            key_state: o.get_field("keyState"),
            key_usage: o.get_field("keyUsage"),
            policy: o.get_field("policy"),
            primary_key_arn: o.get_field("primaryKeyArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            valid_to: o.get_field("validTo"),
        }
    }
}
