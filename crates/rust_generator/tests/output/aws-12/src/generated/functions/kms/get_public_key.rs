#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicKeyArgs {
        /// List of grant tokens
        #[builder(into, default)]
        pub grant_tokens: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key identifier which can be one of the following format:
        /// * Key ID. E.g - `1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Key ARN. E.g. - `arn:aws:kms:us-east-1:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab`
        /// * Alias name. E.g. - `alias/my-key`
        /// * Alias ARN - E.g. - `arn:aws:kms:us-east-1:111122223333:alias/my-key`
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicKeyResult {
        /// Key ARN of the asymmetric CMK from which the public key was downloaded.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of the public key that was downloaded.
        pub customer_master_key_spec: pulumi_gestalt_rust::Output<String>,
        /// Encryption algorithms that AWS KMS supports for this key. Only set when the `key_usage` of the public key is `ENCRYPT_DECRYPT`.
        pub encryption_algorithms: pulumi_gestalt_rust::Output<Vec<String>>,
        pub grant_tokens: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Permitted use of the public key. Valid values are `ENCRYPT_DECRYPT` or `SIGN_VERIFY`
        pub key_usage: pulumi_gestalt_rust::Output<String>,
        /// Exported public key. The value is a DER-encoded X.509 public key, also known as SubjectPublicKeyInfo (SPKI), as defined in [RFC 5280](https://tools.ietf.org/html/rfc5280). The value is Base64-encoded.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// Exported public key. The value is Privacy Enhanced Mail (PEM) encoded.
        pub public_key_pem: pulumi_gestalt_rust::Output<String>,
        /// Signing algorithms that AWS KMS supports for this key. Only set when the `key_usage` of the public key is `SIGN_VERIFY`.
        pub signing_algorithms: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicKeyArgs,
    ) -> GetPublicKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let grant_tokens_binding = args.grant_tokens.get_output(context);
        let key_id_binding = args.key_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kms/getPublicKey:getPublicKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantTokens".into(),
                    value: &grant_tokens_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublicKeyResult {
            arn: o.get_field("arn"),
            customer_master_key_spec: o.get_field("customerMasterKeySpec"),
            encryption_algorithms: o.get_field("encryptionAlgorithms"),
            grant_tokens: o.get_field("grantTokens"),
            id: o.get_field("id"),
            key_id: o.get_field("keyId"),
            key_usage: o.get_field("keyUsage"),
            public_key: o.get_field("publicKey"),
            public_key_pem: o.get_field("publicKeyPem"),
            signing_algorithms: o.get_field("signingAlgorithms"),
        }
    }
}
