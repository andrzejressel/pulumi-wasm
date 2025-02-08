#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPublicKeyArgs,
    ) -> GetPublicKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let grant_tokens_binding = args.grant_tokens.get_output(context).get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getPublicKey:getPublicKey".into(),
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
        GetPublicKeyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            customer_master_key_spec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerMasterKeySpec"),
            ),
            encryption_algorithms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionAlgorithms"),
            ),
            grant_tokens: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantTokens"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyId"),
            ),
            key_usage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyUsage"),
            ),
            public_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKey"),
            ),
            public_key_pem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKeyPem"),
            ),
            signing_algorithms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signingAlgorithms"),
            ),
        }
    }
}
