/// Creates and manages an AWS XRay Encryption Config.
///
/// > **NOTE:** Removing this resource from the provider has no effect to the encryption configuration within X-Ray.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = encryption_config::create(
///         "example",
///         EncryptionConfigArgs::builder().type_("NONE").build_struct(),
///     );
/// }
/// ```
///
///
/// ### With KMS Key
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["kms:*",])
///                     .effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["arn:aws:iam::${current.accountId}:root",]). type
///                     ("AWS").build_struct(),]).resources(vec!["*",])
///                     .sid("Enable IAM User Permissions").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleEncryptionConfig = encryption_config::create(
///         "exampleEncryptionConfig",
///         EncryptionConfigArgs::builder()
///             .key_id("${exampleKey.arn}")
///             .type_("KMS")
///             .build_struct(),
///     );
///     let exampleKey = key::create(
///         "exampleKey",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Some Key")
///             .policy("${example.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import XRay Encryption Config using the region name. For example:
///
/// ```sh
/// $ pulumi import aws:xray/encryptionConfig:EncryptionConfig example us-west-2
/// ```
pub mod encryption_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EncryptionConfigArgs {
        /// An AWS KMS customer master key (CMK) ARN.
        #[builder(into, default)]
        pub key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of encryption. Set to `KMS` to use your own key for encryption. Set to `NONE` for default encryption.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EncryptionConfigResult {
        /// An AWS KMS customer master key (CMK) ARN.
        pub key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of encryption. Set to `KMS` to use your own key for encryption. Set to `NONE` for default encryption.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EncryptionConfigArgs) -> EncryptionConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_id_binding = args.key_id.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:xray/encryptionConfig:EncryptionConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keyId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EncryptionConfigResult {
            key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
