/// Creates and manages an AWS XRay Encryption Config.
///
/// > **NOTE:** Removing this resource from the provider has no effect to the encryption configuration within X-Ray.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       description: Some Key
///       deletionWindowInDays: 7
///       policy: ${example.json}
///   exampleEncryptionConfig:
///     type: aws:xray:EncryptionConfig
///     name: example
///     properties:
///       type: KMS
///       keyId: ${exampleKey.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: Enable IAM User Permissions
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${current.accountId}:root
///             actions:
///               - kms:*
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import XRay Encryption Config using the region name. For example:
///
/// ```sh
/// $ pulumi import aws:xray/encryptionConfig:EncryptionConfig example us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod encryption_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EncryptionConfigArgs {
        /// An AWS KMS customer master key (CMK) ARN.
        #[builder(into, default)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of encryption. Set to `KMS` to use your own key for encryption. Set to `NONE` for default encryption.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EncryptionConfigResult {
        /// An AWS KMS customer master key (CMK) ARN.
        pub key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of encryption. Set to `KMS` to use your own key for encryption. Set to `NONE` for default encryption.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EncryptionConfigArgs,
    ) -> EncryptionConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:xray/encryptionConfig:EncryptionConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EncryptionConfigResult {
            key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyId"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
