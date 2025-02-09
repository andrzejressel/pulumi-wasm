#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kms_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsSecretArgs {
        /// The [additional authenticated data](https://cloud.google.com/kms/docs/additional-authenticated-data) used for integrity checks during encryption and decryption.
        #[builder(into, default)]
        pub additional_authenticated_data: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ciphertext to be decrypted, encoded in base64
        #[builder(into)]
        pub ciphertext: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the CryptoKey that will be used to
        /// decrypt the provided ciphertext. This is represented by the format
        /// `{projectId}/{location}/{keyRingName}/{cryptoKeyName}`.
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsSecretResult {
        pub additional_authenticated_data: pulumi_gestalt_rust::Output<Option<String>>,
        pub ciphertext: pulumi_gestalt_rust::Output<String>,
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Contains the result of decrypting the provided ciphertext.
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKmsSecretArgs,
    ) -> GetKmsSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_authenticated_data_binding = args
            .additional_authenticated_data
            .get_output(context);
        let ciphertext_binding = args.ciphertext.get_output(context);
        let crypto_key_binding = args.crypto_key.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getKMSSecret:getKMSSecret".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalAuthenticatedData".into(),
                    value: additional_authenticated_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ciphertext".into(),
                    value: ciphertext_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: crypto_key_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKmsSecretResult {
            additional_authenticated_data: o.get_field("additionalAuthenticatedData"),
            ciphertext: o.get_field("ciphertext"),
            crypto_key: o.get_field("cryptoKey"),
            id: o.get_field("id"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
