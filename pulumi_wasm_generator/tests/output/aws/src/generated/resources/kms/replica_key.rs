/// Manages a KMS multi-Region replica key.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = key::create(
///         "primary",
///         KeyArgs::builder()
///             .deletion_window_in_days(30)
///             .description("Multi-Region primary key")
///             .multi_region(true)
///             .build_struct(),
///     );
///     let replica = replica_key::create(
///         "replica",
///         ReplicaKeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Multi-Region replica key")
///             .primary_key_arn("${primary.arn}")
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
/// $ pulumi import aws:kms/replicaKey:ReplicaKey example 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
pub mod replica_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicaKeyArgs {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A description of the KMS key.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the replica key is enabled. Disabled KMS keys cannot be used in cryptographic operations. The default value is `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The key policy to attach to the KMS key. If you do not specify a key policy, AWS KMS attaches the [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) to the KMS key.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the multi-Region primary key to replicate. The primary key must be in a different AWS Region of the same AWS Partition. You can create only one replica of a given primary key in each AWS Region.
        #[builder(into)]
        pub primary_key_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the replica key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicaKeyResult {
        /// The Amazon Resource Name (ARN) of the replica key. The key ARNs of related multi-Region keys differ only in the Region value.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A description of the KMS key.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the replica key is enabled. Disabled KMS keys cannot be used in cryptographic operations. The default value is `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The key ID of the replica key. Related multi-Region keys have the same key ID.
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// A Boolean value that specifies whether key rotation is enabled. This is a shared property of multi-Region keys.
        pub key_rotation_enabled: pulumi_wasm_rust::Output<bool>,
        /// The type of key material in the KMS key. This is a shared property of multi-Region keys.
        pub key_spec: pulumi_wasm_rust::Output<String>,
        /// The [cryptographic operations](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations) for which you can use the KMS key. This is a shared property of multi-Region keys.
        pub key_usage: pulumi_wasm_rust::Output<String>,
        /// The key policy to attach to the KMS key. If you do not specify a key policy, AWS KMS attaches the [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) to the KMS key.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The ARN of the multi-Region primary key to replicate. The primary key must be in a different AWS Region of the same AWS Partition. You can create only one replica of a given primary key in each AWS Region.
        pub primary_key_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the replica key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: ReplicaKeyArgs) -> ReplicaKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_inner();
        let deletion_window_in_days_binding = args.deletion_window_in_days.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let policy_binding = args.policy.get_inner();
        let primary_key_arn_binding = args.primary_key_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/replicaKey:ReplicaKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                    value: &bypass_policy_lockout_safety_check_binding,
                },
                register_interface::ObjectField {
                    name: "deletionWindowInDays".into(),
                    value: &deletion_window_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "primaryKeyArn".into(),
                    value: &primary_key_arn_binding,
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
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                },
                register_interface::ResultField {
                    name: "deletionWindowInDays".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "keyId".into(),
                },
                register_interface::ResultField {
                    name: "keyRotationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "keySpec".into(),
                },
                register_interface::ResultField {
                    name: "keyUsage".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "primaryKeyArn".into(),
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
        ReplicaKeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bypass_policy_lockout_safety_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bypassPolicyLockoutSafetyCheck").unwrap(),
            ),
            deletion_window_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionWindowInDays").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyId").unwrap(),
            ),
            key_rotation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRotationEnabled").unwrap(),
            ),
            key_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySpec").unwrap(),
            ),
            key_usage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyUsage").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            primary_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKeyArn").unwrap(),
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