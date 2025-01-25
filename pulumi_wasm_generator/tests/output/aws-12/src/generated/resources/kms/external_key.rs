/// Manages a single-Region or multi-Region primary KMS key that uses external key material.
/// To instead manage a single-Region or multi-Region primary KMS key where AWS automatically generates and potentially rotates key material, see the `aws.kms.Key` resource.
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = external_key::create(
///         "example",
///         ExternalKeyArgs::builder()
///             .description("KMS EXTERNAL for AMI encryption")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS External Keys using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/externalKey:ExternalKey a arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
pub mod external_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExternalKeyArgs {
        /// Specifies whether to disable the policy lockout check performed when creating or updating the key's policy. Setting this value to `true` increases the risk that the key becomes unmanageable. For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the AWS Key Management Service Developer Guide. Defaults to `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Duration in days after which the key is deleted after destruction of the resource. Must be between `7` and `30` days. Defaults to `30`.
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Description of the key.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the key is enabled. Keys pending import can only be `false`. Imported keys default to `true` unless expired.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Base64 encoded 256-bit symmetric encryption key material to import. The CMK is permanently associated with this key material. The same key material can be reimported, but you cannot import different key material.
        #[builder(into, default)]
        pub key_material_base64: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        #[builder(into, default)]
        pub multi_region: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A key policy JSON document. If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A key-value map of tags to assign to the key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. If not specified, key material does not expire. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub valid_to: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ExternalKeyResult {
        /// The Amazon Resource Name (ARN) of the key.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to disable the policy lockout check performed when creating or updating the key's policy. Setting this value to `true` increases the risk that the key becomes unmanageable. For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the AWS Key Management Service Developer Guide. Defaults to `false`.
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// Duration in days after which the key is deleted after destruction of the resource. Must be between `7` and `30` days. Defaults to `30`.
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Description of the key.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the key is enabled. Keys pending import can only be `false`. Imported keys default to `true` unless expired.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether the key material expires. Empty when pending key material import, otherwise `KEY_MATERIAL_EXPIRES` or `KEY_MATERIAL_DOES_NOT_EXPIRE`.
        pub expiration_model: pulumi_wasm_rust::Output<String>,
        /// Base64 encoded 256-bit symmetric encryption key material to import. The CMK is permanently associated with this key material. The same key material can be reimported, but you cannot import different key material.
        pub key_material_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of the CMK.
        pub key_state: pulumi_wasm_rust::Output<String>,
        /// The cryptographic operations for which you can use the CMK.
        pub key_usage: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        pub multi_region: pulumi_wasm_rust::Output<bool>,
        /// A key policy JSON document. If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// A key-value map of tags to assign to the key. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. If not specified, key material does not expire. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub valid_to: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExternalKeyArgs,
    ) -> ExternalKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_output(context)
            .get_inner();
        let deletion_window_in_days_binding = args
            .deletion_window_in_days
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let key_material_base64_binding = args
            .key_material_base64
            .get_output(context)
            .get_inner();
        let multi_region_binding = args.multi_region.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let valid_to_binding = args.valid_to.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/externalKey:ExternalKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "keyMaterialBase64".into(),
                    value: &key_material_base64_binding,
                },
                register_interface::ObjectField {
                    name: "multiRegion".into(),
                    value: &multi_region_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validTo".into(),
                    value: &valid_to_binding,
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
                    name: "expirationModel".into(),
                },
                register_interface::ResultField {
                    name: "keyMaterialBase64".into(),
                },
                register_interface::ResultField {
                    name: "keyState".into(),
                },
                register_interface::ResultField {
                    name: "keyUsage".into(),
                },
                register_interface::ResultField {
                    name: "multiRegion".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "validTo".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExternalKeyResult {
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
            expiration_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationModel").unwrap(),
            ),
            key_material_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyMaterialBase64").unwrap(),
            ),
            key_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyState").unwrap(),
            ),
            key_usage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyUsage").unwrap(),
            ),
            multi_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegion").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            valid_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validTo").unwrap(),
            ),
        }
    }
}
