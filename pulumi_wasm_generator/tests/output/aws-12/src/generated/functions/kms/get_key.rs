pub mod get_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// List of grant tokens
        #[builder(into, default)]
        pub grant_tokens: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key identifier which can be one of the following format:
        /// * Key ID. E.g: `1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Key ARN. E.g.: `arn:aws:kms:us-east-1:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Alias name. E.g.: `alias/my-key`
        /// * Alias ARN: E.g.: `arn:aws:kms:us-east-1:111122223333:alias/my-key`
        #[builder(into)]
        pub key_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let grant_tokens_binding = args.grant_tokens.get_output(context).get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getKey:getKey".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            cloud_hsm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudHsmClusterId"),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            custom_key_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customKeyStoreId"),
            ),
            customer_master_key_spec: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerMasterKeySpec"),
            ),
            deletion_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            expiration_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationModel"),
            ),
            grant_tokens: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("grantTokens"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("keyId")),
            key_manager: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyManager"),
            ),
            key_spec: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySpec"),
            ),
            key_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyState"),
            ),
            key_usage: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyUsage"),
            ),
            multi_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiRegion"),
            ),
            multi_region_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiRegionConfigurations"),
            ),
            origin: pulumi_wasm_rust::__private::into_domain(o.extract_field("origin")),
            pending_deletion_window_in_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pendingDeletionWindowInDays"),
            ),
            valid_to: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validTo"),
            ),
            xks_key_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("xksKeyConfigurations"),
            ),
        }
    }
}
