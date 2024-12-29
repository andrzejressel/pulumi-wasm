/// Manages a single-Region or multi-Region primary KMS key.
///
/// > **NOTE on KMS Key Policy:** KMS Key Policy can be configured in either the standalone resource `aws.kms.KeyPolicy`
/// or with the parameter `policy` in this resource.
/// Configuring with both will cause inconsistencies and may overwrite configuration.
///
/// ## Example Usage
///
/// ### Symmetric Encryption KMS Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: An example symmetric encryption KMS key
///       enableKeyRotation: true
///       deletionWindowInDays: 20
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: key-default-1
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Action: kms:*
///               Resource: '*'
///             - Sid: Allow administration of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:user/Alice
///               Action:
///                 - kms:ReplicateKey
///                 - kms:Create*
///                 - kms:Describe*
///                 - kms:Enable*
///                 - kms:List*
///                 - kms:Put*
///                 - kms:Update*
///                 - kms:Revoke*
///                 - kms:Disable*
///                 - kms:Get*
///                 - kms:Delete*
///                 - kms:ScheduleKeyDeletion
///                 - kms:CancelKeyDeletion
///               Resource: '*'
///             - Sid: Allow use of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:user/Bob
///               Action:
///                 - kms:DescribeKey
///                 - kms:Encrypt
///                 - kms:Decrypt
///                 - kms:ReEncrypt*
///                 - kms:GenerateDataKey
///                 - kms:GenerateDataKeyWithoutPlaintext
///               Resource: '*'
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ### Symmetric Encryption KMS Key With Standalone Policy Resource
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: An example symmetric encryption KMS key
///       enableKeyRotation: true
///       deletionWindowInDays: 20
///   exampleKeyPolicy:
///     type: aws:kms:KeyPolicy
///     name: example
///     properties:
///       keyId: ${example.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: key-default-1
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Action: kms:*
///               Resource: '*'
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ### Asymmetric KMS Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: RSA-3072 asymmetric KMS key for signing and verification
///       customerMasterKeySpec: RSA_3072
///       keyUsage: SIGN_VERIFY
///       enableKeyRotation: false
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: key-default-1
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Action: kms:*
///               Resource: '*'
///             - Sid: Allow administration of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:role/Admin
///               Action:
///                 - kms:Create*
///                 - kms:Describe*
///                 - kms:Enable*
///                 - kms:List*
///                 - kms:Put*
///                 - kms:Update*
///                 - kms:Revoke*
///                 - kms:Disable*
///                 - kms:Get*
///                 - kms:Delete*
///                 - kms:ScheduleKeyDeletion
///                 - kms:CancelKeyDeletion
///               Resource: '*'
///             - Sid: Allow use of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:role/Developer
///               Action:
///                 - kms:Sign
///                 - kms:Verify
///                 - kms:DescribeKey
///               Resource: '*'
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ### HMAC KMS key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: HMAC_384 key for tokens
///       customerMasterKeySpec: HMAC_384
///       keyUsage: GENERATE_VERIFY_MAC
///       enableKeyRotation: false
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: key-default-1
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Action: kms:*
///               Resource: '*'
///             - Sid: Allow administration of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:role/Admin
///               Action:
///                 - kms:Create*
///                 - kms:Describe*
///                 - kms:Enable*
///                 - kms:List*
///                 - kms:Put*
///                 - kms:Update*
///                 - kms:Revoke*
///                 - kms:Disable*
///                 - kms:Get*
///                 - kms:Delete*
///                 - kms:ScheduleKeyDeletion
///                 - kms:CancelKeyDeletion
///               Resource: '*'
///             - Sid: Allow use of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:role/Developer
///               Action:
///                 - kms:GenerateMac
///                 - kms:VerifyMac
///                 - kms:DescribeKey
///               Resource: '*'
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ### Multi-Region Primary Key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: An example multi-Region primary key
///       multiRegion: true
///       enableKeyRotation: true
///       deletionWindowInDays: 10
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Id: key-default-1
///           Statement:
///             - Sid: Enable IAM User Permissions
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Action: kms:*
///               Resource: '*'
///             - Sid: Allow administration of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:user/Alice
///               Action:
///                 - kms:ReplicateKey
///                 - kms:Create*
///                 - kms:Describe*
///                 - kms:Enable*
///                 - kms:List*
///                 - kms:Put*
///                 - kms:Update*
///                 - kms:Revoke*
///                 - kms:Disable*
///                 - kms:Get*
///                 - kms:Delete*
///                 - kms:ScheduleKeyDeletion
///                 - kms:CancelKeyDeletion
///               Resource: '*'
///             - Sid: Allow use of the key
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:user/Bob
///               Action:
///                 - kms:DescribeKey
///                 - kms:Encrypt
///                 - kms:Decrypt
///                 - kms:ReEncrypt*
///                 - kms:GenerateDataKey
///                 - kms:GenerateDataKeyWithoutPlaintext
///               Resource: '*'
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS Keys using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/key:Key a 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
pub mod key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the KMS [Custom Key Store](https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html) where the key will be stored instead of KMS (eg CloudHSM).
        #[builder(into, default)]
        pub custom_key_store_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the key contains a symmetric key or an asymmetric key pair and the encryption algorithms or signing algorithms that the key supports.
        /// Valid values: `SYMMETRIC_DEFAULT`,  `RSA_2048`, `RSA_3072`, `RSA_4096`, `HMAC_256`, `ECC_NIST_P256`, `ECC_NIST_P384`, `ECC_NIST_P521`, or `ECC_SECG_P256K1`. Defaults to `SYMMETRIC_DEFAULT`. For help with choosing a key spec, see the [AWS KMS Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose.html).
        #[builder(into, default)]
        pub customer_master_key_spec: pulumi_wasm_rust::Output<Option<String>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        /// If the KMS key is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The description of the key as viewed in AWS console.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether [key rotation](http://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html) is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub enable_key_rotation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the key is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the intended use of the key. Valid values: `ENCRYPT_DECRYPT`, `SIGN_VERIFY`, or `GENERATE_VERIFY_MAC`.
        /// Defaults to `ENCRYPT_DECRYPT`.
        #[builder(into, default)]
        pub key_usage: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        #[builder(into, default)]
        pub multi_region: pulumi_wasm_rust::Output<Option<bool>>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom period of time between each rotation date. Must be a number between 90 and 2560 (inclusive).
        #[builder(into, default)]
        pub rotation_period_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifies the external key that serves as key material for the KMS key in an external key store.
        #[builder(into, default)]
        pub xks_key_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// The Amazon Resource Name (ARN) of the key.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the KMS [Custom Key Store](https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html) where the key will be stored instead of KMS (eg CloudHSM).
        pub custom_key_store_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the key contains a symmetric key or an asymmetric key pair and the encryption algorithms or signing algorithms that the key supports.
        /// Valid values: `SYMMETRIC_DEFAULT`,  `RSA_2048`, `RSA_3072`, `RSA_4096`, `HMAC_256`, `ECC_NIST_P256`, `ECC_NIST_P384`, `ECC_NIST_P521`, or `ECC_SECG_P256K1`. Defaults to `SYMMETRIC_DEFAULT`. For help with choosing a key spec, see the [AWS KMS Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose.html).
        pub customer_master_key_spec: pulumi_wasm_rust::Output<Option<String>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        /// If the KMS key is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The description of the key as viewed in AWS console.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies whether [key rotation](http://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html) is enabled. Defaults to `false`.
        pub enable_key_rotation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the key is enabled. Defaults to `true`.
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The globally unique identifier for the key.
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the intended use of the key. Valid values: `ENCRYPT_DECRYPT`, `SIGN_VERIFY`, or `GENERATE_VERIFY_MAC`.
        /// Defaults to `ENCRYPT_DECRYPT`.
        pub key_usage: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        pub multi_region: pulumi_wasm_rust::Output<bool>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Custom period of time between each rotation date. Must be a number between 90 and 2560 (inclusive).
        pub rotation_period_in_days: pulumi_wasm_rust::Output<i32>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifies the external key that serves as key material for the KMS key in an external key store.
        pub xks_key_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeyArgs) -> KeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_inner();
        let custom_key_store_id_binding = args.custom_key_store_id.get_inner();
        let customer_master_key_spec_binding = args.customer_master_key_spec.get_inner();
        let deletion_window_in_days_binding = args.deletion_window_in_days.get_inner();
        let description_binding = args.description.get_inner();
        let enable_key_rotation_binding = args.enable_key_rotation.get_inner();
        let is_enabled_binding = args.is_enabled.get_inner();
        let key_usage_binding = args.key_usage.get_inner();
        let multi_region_binding = args.multi_region.get_inner();
        let policy_binding = args.policy.get_inner();
        let rotation_period_in_days_binding = args.rotation_period_in_days.get_inner();
        let tags_binding = args.tags.get_inner();
        let xks_key_id_binding = args.xks_key_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/key:Key".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                    value: &bypass_policy_lockout_safety_check_binding,
                },
                register_interface::ObjectField {
                    name: "customKeyStoreId".into(),
                    value: &custom_key_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "customerMasterKeySpec".into(),
                    value: &customer_master_key_spec_binding,
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
                    name: "enableKeyRotation".into(),
                    value: &enable_key_rotation_binding,
                },
                register_interface::ObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "keyUsage".into(),
                    value: &key_usage_binding,
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
                    name: "rotationPeriodInDays".into(),
                    value: &rotation_period_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "xksKeyId".into(),
                    value: &xks_key_id_binding,
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
                    name: "customKeyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "customerMasterKeySpec".into(),
                },
                register_interface::ResultField {
                    name: "deletionWindowInDays".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enableKeyRotation".into(),
                },
                register_interface::ResultField {
                    name: "isEnabled".into(),
                },
                register_interface::ResultField {
                    name: "keyId".into(),
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
                    name: "rotationPeriodInDays".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "xksKeyId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bypass_policy_lockout_safety_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bypassPolicyLockoutSafetyCheck").unwrap(),
            ),
            custom_key_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customKeyStoreId").unwrap(),
            ),
            customer_master_key_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerMasterKeySpec").unwrap(),
            ),
            deletion_window_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionWindowInDays").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enable_key_rotation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableKeyRotation").unwrap(),
            ),
            is_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isEnabled").unwrap(),
            ),
            key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyId").unwrap(),
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
            rotation_period_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rotationPeriodInDays").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            xks_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xksKeyId").unwrap(),
            ),
        }
    }
}
