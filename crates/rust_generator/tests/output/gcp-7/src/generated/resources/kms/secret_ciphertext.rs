/// Encrypts secret data with Google Cloud KMS and provides access to the ciphertext.
///
///
/// > **NOTE:** Using this resource will allow you to conceal secret data within your
/// resource definitions, but it does not take care of protecting that data in the
/// logging output, plan output, or state output.  Please take care to secure your secret
/// data outside of resource definitions.
///
///
/// To get more information about SecretCiphertext, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys/encrypt)
/// * How-to Guides
///     * [Encrypting and decrypting data with a symmetric key](https://cloud.google.com/kms/docs/encrypt-decrypt)
///
///
///
/// ## Example Usage
///
/// ### Kms Secret Ciphertext Basic
///
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   cryptokey:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
///   myPassword:
///     type: gcp:kms:SecretCiphertext
///     name: my_password
///     properties:
///       cryptoKey: ${cryptokey.id}
///       plaintext: my-secret-password
///   instance:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: default
///       name: my-instance
///       machineType: e2-medium
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: debian-cloud/debian-11
///       metadata:
///         password: ${myPassword.ciphertext}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret_ciphertext {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretCiphertextArgs {
        /// The additional authenticated data used for integrity checks during encryption and decryption.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into, default)]
        pub additional_authenticated_data: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The full name of the CryptoKey that will be used to encrypt the provided plaintext.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}'`
        ///
        ///
        /// - - -
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The plaintext to be encrypted.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub plaintext: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretCiphertextResult {
        /// The additional authenticated data used for integrity checks during encryption and decryption.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub additional_authenticated_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// Contains the result of encrypting the provided plaintext, encoded in base64.
        pub ciphertext: pulumi_gestalt_rust::Output<String>,
        /// The full name of the CryptoKey that will be used to encrypt the provided plaintext.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}'`
        ///
        ///
        /// - - -
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        /// The plaintext to be encrypted.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretCiphertextArgs,
    ) -> SecretCiphertextResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_authenticated_data_binding = args
            .additional_authenticated_data
            .get_output(context);
        let crypto_key_binding = args.crypto_key.get_output(context);
        let plaintext_binding = args.plaintext.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/secretCiphertext:SecretCiphertext".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalAuthenticatedData".into(),
                    value: additional_authenticated_data_binding.get_id(),
                },
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
        let o = context.register_resource(request);
        SecretCiphertextResult {
            additional_authenticated_data: o.get_field("additionalAuthenticatedData"),
            ciphertext: o.get_field("ciphertext"),
            crypto_key: o.get_field("cryptoKey"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
