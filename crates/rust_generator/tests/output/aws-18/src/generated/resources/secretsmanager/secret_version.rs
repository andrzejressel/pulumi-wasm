/// Provides a resource to manage AWS Secrets Manager secret version including its secret value. To manage secret metadata, see the `aws.secretsmanager.Secret` resource.
///
/// > **NOTE:** If the `AWSCURRENT` staging label is present on this version during resource deletion, that label cannot be removed and will be skipped to prevent errors when fully deleting the secret. That label will leave this secret version active even after the resource is deleted from this provider unless the secret itself is deleted. Move the `AWSCURRENT` staging label before or after deleting this resource from this provider to fully trigger version deprecation if necessary.
///
/// ## Example Usage
///
/// ### Simple String Value
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = secret_version::create(
///         "example",
///         SecretVersionArgs::builder()
///             .secret_id("${exampleAwsSecretsmanagerSecret.id}")
///             .secret_string("example-string-to-protect")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Key-Value Pairs
///
/// Secrets Manager also accepts key-value pairs in JSON.
///
/// ```yaml
/// configuration:
///   # The map here can come from other supported configurations
///   # like locals, resource attribute, map() built-in, etc.
///   example:
///     type: map(string)
///     default:
///       key1: value1
///       key2: value2
/// resources:
///   exampleSecretVersion:
///     type: aws:secretsmanager:SecretVersion
///     name: example
///     properties:
///       secretId: ${exampleAwsSecretsmanagerSecret.id}
///       secretString:
///         fn::toJSON: ${example}
/// ```
///
///
/// Reading key-value pairs from JSON back into a native map
///
/// ## Import
///
/// Using `pulumi import`, import `aws_secretsmanager_secret_version` using the secret ID and version ID. For example:
///
/// ```sh
/// $ pulumi import aws:secretsmanager/secretVersion:SecretVersion example 'arn:aws:secretsmanager:us-east-1:123456789012:secret:example-123456|xxxxx-xxxxxxx-xxxxxxx-xxxxx'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod secret_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretVersionArgs {
        /// Specifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
        #[builder(into, default)]
        pub secret_binary: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
        #[builder(into, default)]
        pub secret_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.
        ///
        /// > **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
        #[builder(into, default)]
        pub version_stages: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct SecretVersionResult {
        /// The ARN of the secret.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
        pub secret_binary: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
        pub secret_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier of the version of the secret.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.
        ///
        /// > **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
        pub version_stages: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecretVersionArgs,
    ) -> SecretVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let secret_binary_binding = args.secret_binary.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let secret_string_binding = args.secret_string.get_output(context);
        let version_stages_binding = args.version_stages.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:secretsmanager/secretVersion:SecretVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretBinary".into(),
                    value: secret_binary_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: secret_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretString".into(),
                    value: secret_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionStages".into(),
                    value: version_stages_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecretVersionResult {
            arn: o.get_field("arn"),
            secret_binary: o.get_field("secretBinary"),
            secret_id: o.get_field("secretId"),
            secret_string: o.get_field("secretString"),
            version_id: o.get_field("versionId"),
            version_stages: o.get_field("versionStages"),
        }
    }
}
