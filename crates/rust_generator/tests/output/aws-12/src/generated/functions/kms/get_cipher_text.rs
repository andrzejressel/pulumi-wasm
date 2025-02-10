#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cipher_text {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCipherTextArgs {
        /// An optional mapping that makes up the encryption context.
        #[builder(into, default)]
        pub context: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Globally unique key ID for the customer master key.
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Data to be encrypted. Note that this may show up in logs, and it will be stored in the state file.
        #[builder(into)]
        pub plaintext: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCipherTextResult {
        /// Base64 encoded ciphertext
        pub ciphertext_blob: pulumi_gestalt_rust::Output<String>,
        pub context: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_id: pulumi_gestalt_rust::Output<String>,
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCipherTextArgs,
    ) -> GetCipherTextResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let context_binding = args.context.get_output(context);
        let key_id_binding = args.key_id.get_output(context);
        let plaintext_binding = args.plaintext.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kms/getCipherText:getCipherText".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "context".into(),
                    value: context_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plaintext".into(),
                    value: plaintext_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCipherTextResult {
            ciphertext_blob: o.get_field("ciphertextBlob"),
            context: o.get_field("context"),
            id: o.get_field("id"),
            key_id: o.get_field("keyId"),
            plaintext: o.get_field("plaintext"),
        }
    }
}
