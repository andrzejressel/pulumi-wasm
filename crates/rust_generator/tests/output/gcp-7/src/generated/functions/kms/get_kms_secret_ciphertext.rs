#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kms_secret_ciphertext {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsSecretCiphertextArgs {
        /// The id of the CryptoKey that will be used to
        /// encrypt the provided plaintext. This is represented by the format
        /// `{projectId}/{location}/{keyRingName}/{cryptoKeyName}`.
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The plaintext to be encrypted
        #[builder(into)]
        pub plaintext: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsSecretCiphertextResult {
        /// Contains the result of encrypting the provided plaintext, encoded in base64.
        pub ciphertext: pulumi_gestalt_rust::Output<String>,
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKmsSecretCiphertextArgs,
    ) -> GetKmsSecretCiphertextResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let plaintext_binding = args.plaintext.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getKMSSecretCiphertext:getKMSSecretCiphertext".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: crypto_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plaintext".into(),
                    value: plaintext_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKmsSecretCiphertextResult {
            ciphertext: o.get_field("ciphertext"),
            crypto_key: o.get_field("cryptoKey"),
            id: o.get_field("id"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
