pub mod get_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// List of grant tokens
        #[builder(into, default)]
        pub grant_tokens: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key identifier which can be one of the following format:
        /// * Key ID. E.g: `1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Key ARN. E.g.: `arn:aws:kms:us-east-1:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Alias name. E.g.: `alias/my-key`
        /// * Alias ARN: E.g.: `arn:aws:kms:us-east-1:111122223333:alias/my-key`
        #[builder(into)]
        pub key_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKeyResult {
        /// The key ARN of a primary or replica key of a multi-Region key.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The twelve-digit account ID of the AWS account that owns the key
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The cluster ID of the AWS CloudHSM cluster that contains the key material for the KMS key.
        pub cloud_hsm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The date and time when the key was created
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the custom key store that contains the KMS key.
        pub custom_key_store_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the key contains a symmetric key or an asymmetric key pair and the encryption algorithms or signing algorithms that the key supports
        pub customer_master_key_spec: pulumi_wasm_rust::Output<String>,
        /// The date and time after which AWS KMS deletes the key. This value is present only when `key_state` is `PendingDeletion`, otherwise this value is 0
        pub deletion_date: pulumi_wasm_rust::Output<String>,
        /// The description of the key.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the key is enabled. When `key_state` is `Enabled` this value is true, otherwise it is false
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether the Key's key material expires. This value is present only when `origin` is `EXTERNAL`, otherwise this value is empty
        pub expiration_model: pulumi_wasm_rust::Output<String>,
        pub grant_tokens: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// The key's manager
        pub key_manager: pulumi_wasm_rust::Output<String>,
        /// Describes the type of key material in the KMS key.
        pub key_spec: pulumi_wasm_rust::Output<String>,
        /// The state of the key
        pub key_state: pulumi_wasm_rust::Output<String>,
        /// Specifies the intended use of the key
        pub key_usage: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the KMS key is a multi-Region (`true`) or regional (`false`) key.
        pub multi_region: pulumi_wasm_rust::Output<bool>,
        /// Lists the primary and replica keys in same multi-Region key. Present only when the value of `multi_region` is `true`.
        pub multi_region_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetKeyMultiRegionConfiguration>,
        >,
        /// When this value is `AWS_KMS`, AWS KMS created the key material. When this value is `EXTERNAL`, the key material was imported from your existing key management infrastructure or the CMK lacks key material
        pub origin: pulumi_wasm_rust::Output<String>,
        /// The waiting period before the primary key in a multi-Region key is deleted.
        pub pending_deletion_window_in_days: pulumi_wasm_rust::Output<i32>,
        /// The time at which the imported key material expires. This value is present only when `origin` is `EXTERNAL` and whose `expiration_model` is `KEY_MATERIAL_EXPIRES`, otherwise this value is 0
        pub valid_to: pulumi_wasm_rust::Output<String>,
        /// Information about the external key that is associated with a KMS key in an external key store.
        pub xks_key_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetKeyXksKeyConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKeyArgs) -> GetKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let grant_tokens_binding = args.grant_tokens.get_inner();
        let key_id_binding = args.key_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getKey:getKey".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "grantTokens".into(),
                    value: &grant_tokens_binding,
                },
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "cloudHsmClusterId".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "customKeyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "customerMasterKeySpec".into(),
                },
                register_interface::ResultField {
                    name: "deletionDate".into(),
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
                    name: "grantTokens".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyId".into(),
                },
                register_interface::ResultField {
                    name: "keyManager".into(),
                },
                register_interface::ResultField {
                    name: "keySpec".into(),
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
                    name: "multiRegionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "origin".into(),
                },
                register_interface::ResultField {
                    name: "pendingDeletionWindowInDays".into(),
                },
                register_interface::ResultField {
                    name: "validTo".into(),
                },
                register_interface::ResultField {
                    name: "xksKeyConfigurations".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            cloud_hsm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudHsmClusterId").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            custom_key_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customKeyStoreId").unwrap(),
            ),
            customer_master_key_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerMasterKeySpec").unwrap(),
            ),
            deletion_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionDate").unwrap(),
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
            grant_tokens: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantTokens").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyId").unwrap(),
            ),
            key_manager: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyManager").unwrap(),
            ),
            key_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySpec").unwrap(),
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
            multi_region_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiRegionConfigurations").unwrap(),
            ),
            origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origin").unwrap(),
            ),
            pending_deletion_window_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pendingDeletionWindowInDays").unwrap(),
            ),
            valid_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validTo").unwrap(),
            ),
            xks_key_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xksKeyConfigurations").unwrap(),
            ),
        }
    }
}
