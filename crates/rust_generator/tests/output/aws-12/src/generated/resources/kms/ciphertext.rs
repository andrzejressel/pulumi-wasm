/// The KMS ciphertext resource allows you to encrypt plaintext into ciphertext
/// by using an AWS KMS customer master key. The value returned by this resource
/// is stable across every apply. For a changing ciphertext value each apply, see
/// the `aws.kms.Ciphertext` data source.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let oauth = ciphertext::create(
///         "oauth",
///         CiphertextArgs::builder()
///             .key_id("${oauthConfig.keyId}")
///             .plaintext(
///                 "{\n  \"client_id\": \"e587dbae22222f55da22\",\n  \"client_secret\": \"8289575d00000ace55e1815ec13673955721b8a5\"\n}",
///             )
///             .build_struct(),
///     );
///     let oauthConfig = key::create(
///         "oauthConfig",
///         KeyArgs::builder().description("oauth config").is_enabled(true).build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod ciphertext {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CiphertextArgs {
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
    pub struct CiphertextResult {
        /// Base64 encoded ciphertext
        pub ciphertext_blob: pulumi_gestalt_rust::Output<String>,
        /// An optional mapping that makes up the encryption context.
        pub context: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Globally unique key ID for the customer master key.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Data to be encrypted. Note that this may show up in logs, and it will be stored in the state file.
        pub plaintext: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CiphertextArgs,
    ) -> CiphertextResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let context_binding = args.context.get_output(context).get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let plaintext_binding = args.plaintext.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/ciphertext:Ciphertext".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "context".into(),
                    value: &context_binding,
                },
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
                register_interface::ObjectField {
                    name: "plaintext".into(),
                    value: &plaintext_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CiphertextResult {
            ciphertext_blob: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ciphertextBlob"),
            ),
            context: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("context"),
            ),
            key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyId"),
            ),
            plaintext: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("plaintext"),
            ),
        }
    }
}
