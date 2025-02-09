#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kms_secret_asymmetric {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsSecretAsymmetricArgs {
        /// The ciphertext to be decrypted, encoded in base64
        #[builder(into)]
        pub ciphertext: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The crc32 checksum of the `ciphertext` in hexadecimal notation. If not specified, it will be computed.
        #[builder(into, default)]
        pub crc32: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the CryptoKey version that will be used to
        /// decrypt the provided ciphertext. This is represented by the format
        /// `projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version}`.
        #[builder(into)]
        pub crypto_key_version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsSecretAsymmetricResult {
        pub ciphertext: pulumi_gestalt_rust::Output<String>,
        /// Contains the crc32 checksum of the provided ciphertext.
        pub crc32: pulumi_gestalt_rust::Output<Option<String>>,
        pub crypto_key_version: pulumi_gestalt_rust::Output<String>,
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
        args: GetKmsSecretAsymmetricArgs,
    ) -> GetKmsSecretAsymmetricResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ciphertext_binding = args.ciphertext.get_output(context);
        let crc32_binding = args.crc32.get_output(context);
        let crypto_key_version_binding = args.crypto_key_version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getKMSSecretAsymmetric:getKMSSecretAsymmetric".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ciphertext".into(),
                    value: ciphertext_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crc32".into(),
                    value: crc32_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyVersion".into(),
                    value: crypto_key_version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKmsSecretAsymmetricResult {
            ciphertext: o.get_field("ciphertext"),
            crc32: o.get_field("crc32"),
            crypto_key_version: o.get_field("cryptoKeyVersion"),
            id: o.get_field("id"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
