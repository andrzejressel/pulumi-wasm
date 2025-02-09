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
///       function: aws:getCallerIdentity
///       arguments: {}
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
///       function: aws:getCallerIdentity
///       arguments: {}
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
///       function: aws:getCallerIdentity
///       arguments: {}
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
///       function: aws:getCallerIdentity
///       arguments: {}
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
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS Keys using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/key:Key a 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ID of the KMS [Custom Key Store](https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html) where the key will be stored instead of KMS (eg CloudHSM).
        #[builder(into, default)]
        pub custom_key_store_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the key contains a symmetric key or an asymmetric key pair and the encryption algorithms or signing algorithms that the key supports.
        /// Valid values: `SYMMETRIC_DEFAULT`,  `RSA_2048`, `RSA_3072`, `RSA_4096`, `HMAC_256`, `ECC_NIST_P256`, `ECC_NIST_P384`, `ECC_NIST_P521`, or `ECC_SECG_P256K1`. Defaults to `SYMMETRIC_DEFAULT`. For help with choosing a key spec, see the [AWS KMS Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose.html).
        #[builder(into, default)]
        pub customer_master_key_spec: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        /// If the KMS key is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The description of the key as viewed in AWS console.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether [key rotation](http://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html) is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub enable_key_rotation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the key is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub is_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the intended use of the key. Valid values: `ENCRYPT_DECRYPT`, `SIGN_VERIFY`, or `GENERATE_VERIFY_MAC`.
        /// Defaults to `ENCRYPT_DECRYPT`.
        #[builder(into, default)]
        pub key_usage: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        #[builder(into, default)]
        pub multi_region: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Custom period of time between each rotation date. Must be a number between 90 and 2560 (inclusive).
        #[builder(into, default)]
        pub rotation_period_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifies the external key that serves as key material for the KMS key in an external key store.
        #[builder(into, default)]
        pub xks_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// The Amazon Resource Name (ARN) of the key.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A flag to indicate whether to bypass the key policy lockout safety check.
        /// Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.
        /// For more information, refer to the scenario in the [Default Key Policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the _AWS Key Management Service Developer Guide_.
        /// The default value is `false`.
        pub bypass_policy_lockout_safety_check: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// ID of the KMS [Custom Key Store](https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html) where the key will be stored instead of KMS (eg CloudHSM).
        pub custom_key_store_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the key contains a symmetric key or an asymmetric key pair and the encryption algorithms or signing algorithms that the key supports.
        /// Valid values: `SYMMETRIC_DEFAULT`,  `RSA_2048`, `RSA_3072`, `RSA_4096`, `HMAC_256`, `ECC_NIST_P256`, `ECC_NIST_P384`, `ECC_NIST_P521`, or `ECC_SECG_P256K1`. Defaults to `SYMMETRIC_DEFAULT`. For help with choosing a key spec, see the [AWS KMS Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose.html).
        pub customer_master_key_spec: pulumi_gestalt_rust::Output<Option<String>>,
        /// The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the KMS key.
        /// If you specify a value, it must be between `7` and `30`, inclusive. If you do not specify a value, it defaults to `30`.
        /// If the KMS key is a multi-Region primary key with replicas, the waiting period begins when the last of its replica keys is deleted. Otherwise, the waiting period begins immediately.
        pub deletion_window_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The description of the key as viewed in AWS console.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether [key rotation](http://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html) is enabled. Defaults to `false`.
        pub enable_key_rotation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the key is enabled. Defaults to `true`.
        pub is_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The globally unique identifier for the key.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the intended use of the key. Valid values: `ENCRYPT_DECRYPT`, `SIGN_VERIFY`, or `GENERATE_VERIFY_MAC`.
        /// Defaults to `ENCRYPT_DECRYPT`.
        pub key_usage: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key. Defaults to `false`.
        pub multi_region: pulumi_gestalt_rust::Output<bool>,
        /// A valid policy JSON document. Although this is a key policy, not an IAM policy, an `aws.iam.getPolicyDocument`, in the form that designates a principal, can be used.
        ///
        /// > **NOTE:** Note: All KMS keys must have a key policy. If a key policy is not specified, AWS gives the KMS key a [default key policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) that gives all principals in the owning account unlimited access to all KMS operations for the key. This default key policy effectively delegates all access control to IAM policies and KMS grants.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Custom period of time between each rotation date. Must be a number between 90 and 2560 (inclusive).
        pub rotation_period_in_days: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifies the external key that serves as key material for the KMS key in an external key store.
        pub xks_key_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KeyArgs,
    ) -> KeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bypass_policy_lockout_safety_check_binding_1 = args
            .bypass_policy_lockout_safety_check
            .get_output(context);
        let bypass_policy_lockout_safety_check_binding = bypass_policy_lockout_safety_check_binding_1
            .get_inner();
        let custom_key_store_id_binding_1 = args.custom_key_store_id.get_output(context);
        let custom_key_store_id_binding = custom_key_store_id_binding_1.get_inner();
        let customer_master_key_spec_binding_1 = args
            .customer_master_key_spec
            .get_output(context);
        let customer_master_key_spec_binding = customer_master_key_spec_binding_1
            .get_inner();
        let deletion_window_in_days_binding_1 = args
            .deletion_window_in_days
            .get_output(context);
        let deletion_window_in_days_binding = deletion_window_in_days_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let enable_key_rotation_binding_1 = args.enable_key_rotation.get_output(context);
        let enable_key_rotation_binding = enable_key_rotation_binding_1.get_inner();
        let is_enabled_binding_1 = args.is_enabled.get_output(context);
        let is_enabled_binding = is_enabled_binding_1.get_inner();
        let key_usage_binding_1 = args.key_usage.get_output(context);
        let key_usage_binding = key_usage_binding_1.get_inner();
        let multi_region_binding_1 = args.multi_region.get_output(context);
        let multi_region_binding = multi_region_binding_1.get_inner();
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let rotation_period_in_days_binding_1 = args
            .rotation_period_in_days
            .get_output(context);
        let rotation_period_in_days_binding = rotation_period_in_days_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let xks_key_id_binding_1 = args.xks_key_id.get_output(context);
        let xks_key_id_binding = xks_key_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/key:Key".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            bypass_policy_lockout_safety_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bypassPolicyLockoutSafetyCheck"),
            ),
            custom_key_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customKeyStoreId"),
            ),
            customer_master_key_spec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerMasterKeySpec"),
            ),
            deletion_window_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionWindowInDays"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable_key_rotation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableKeyRotation"),
            ),
            is_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isEnabled"),
            ),
            key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyId"),
            ),
            key_usage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyUsage"),
            ),
            multi_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiRegion"),
            ),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            rotation_period_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rotationPeriodInDays"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            xks_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("xksKeyId"),
            ),
        }
    }
}
