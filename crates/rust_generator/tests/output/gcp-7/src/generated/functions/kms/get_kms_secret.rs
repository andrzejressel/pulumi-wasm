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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetKmsSecretArgs,
    ) -> GetKmsSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_authenticated_data_binding_1 = args
            .additional_authenticated_data
            .get_output(context);
        let additional_authenticated_data_binding = additional_authenticated_data_binding_1
            .get_inner();
        let ciphertext_binding_1 = args.ciphertext.get_output(context);
        let ciphertext_binding = ciphertext_binding_1.get_inner();
        let crypto_key_binding_1 = args.crypto_key.get_output(context);
        let crypto_key_binding = crypto_key_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKMSSecret:getKMSSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalAuthenticatedData".into(),
                    value: &additional_authenticated_data_binding,
                },
                register_interface::ObjectField {
                    name: "ciphertext".into(),
                    value: &ciphertext_binding,
                },
                register_interface::ObjectField {
                    name: "cryptoKey".into(),
                    value: &crypto_key_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKmsSecretResult {
            additional_authenticated_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalAuthenticatedData"),
            ),
            ciphertext: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ciphertext"),
            ),
            crypto_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cryptoKey"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            plaintext: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("plaintext"),
            ),
        }
    }
}
