#[allow(clippy::doc_lazy_continuation)]
pub mod get_encrypted_value {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEncryptedValueArgs {
        /// The Algorithm which should be used to Decrypt/Encrypt this Value. Possible values are `RSA1_5`, `RSA-OAEP` and `RSA-OAEP-256`.
        #[builder(into)]
        pub algorithm: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Base64 URL Encoded Encrypted Data which should be decrypted into `plain_text_value`.
        #[builder(into, default)]
        pub encrypted_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Key Vault Key which should be used to Decrypt/Encrypt this Value.
        #[builder(into)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The plain-text value which should be Encrypted into `encrypted_data`.
        ///
        /// > **Note:** One of either `encrypted_data` or `plain_text_value` must be specified and is used to populate the encrypted/decrypted value for the other field.
        #[builder(into, default)]
        pub plain_text_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEncryptedValueResult {
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        /// The Base64URL decoded string of `plain_text_value`. Because the API would remove padding characters of `plain_text_value` when encrypting, this attribute is useful to get the original value.
        pub decoded_plain_text_value: pulumi_gestalt_rust::Output<String>,
        pub encrypted_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
        pub plain_text_value: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEncryptedValueArgs,
    ) -> GetEncryptedValueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let algorithm_binding = args.algorithm.get_output(context).get_inner();
        let encrypted_data_binding = args.encrypted_data.get_output(context).get_inner();
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let plain_text_value_binding = args
            .plain_text_value
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getEncryptedValue:getEncryptedValue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "algorithm".into(),
                    value: &algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "encryptedData".into(),
                    value: &encrypted_data_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "plainTextValue".into(),
                    value: &plain_text_value_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEncryptedValueResult {
            algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("algorithm"),
            ),
            decoded_plain_text_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("decodedPlainTextValue"),
            ),
            encrypted_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptedData"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            plain_text_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("plainTextValue"),
            ),
        }
    }
}
