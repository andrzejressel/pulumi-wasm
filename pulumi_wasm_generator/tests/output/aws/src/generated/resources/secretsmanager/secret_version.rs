/// Provides a resource to manage AWS Secrets Manager secret version including its secret value. To manage secret metadata, see the `aws.secretsmanager.Secret` resource.
///
/// > **NOTE:** If the `AWSCURRENT` staging label is present on this version during resource deletion, that label cannot be removed and will be skipped to prevent errors when fully deleting the secret. That label will leave this secret version active even after the resource is deleted from this provider unless the secret itself is deleted. Move the `AWSCURRENT` staging label before or after deleting this resource from this provider to fully trigger version deprecation if necessary.
///
/// ## Example Usage
///
/// ### Simple String Value
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod secret_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretVersionArgs {
        /// Specifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
        #[builder(into, default)]
        pub secret_binary: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// Specifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
        #[builder(into, default)]
        pub secret_string: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.
        ///
        /// > **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
        #[builder(into, default)]
        pub version_stages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct SecretVersionResult {
        /// The ARN of the secret.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies binary data that you want to encrypt and store in this version of the secret. This is required if `secret_string` is not set. Needs to be encoded to base64.
        pub secret_binary: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// Specifies text data that you want to encrypt and store in this version of the secret. This is required if `secret_binary` is not set.
        pub secret_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the version of the secret.
        pub version_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of staging labels that are attached to this version of the secret. A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version. If you do not specify a value, then AWS Secrets Manager automatically moves the staging label `AWSCURRENT` to this new version on creation.
        ///
        /// > **NOTE:** If `version_stages` is configured, you must include the `AWSCURRENT` staging label if this secret version is the only version or if the label is currently present on this secret version, otherwise this provider will show a perpetual difference.
        pub version_stages: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SecretVersionArgs) -> SecretVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let secret_binary_binding = args.secret_binary.get_inner();
        let secret_id_binding = args.secret_id.get_inner();
        let secret_string_binding = args.secret_string.get_inner();
        let version_stages_binding = args.version_stages.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:secretsmanager/secretVersion:SecretVersion".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secretBinary".into(),
                    value: &secret_binary_binding,
                },
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "secretString".into(),
                    value: &secret_string_binding,
                },
                register_interface::ObjectField {
                    name: "versionStages".into(),
                    value: &version_stages_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "secretBinary".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
                register_interface::ResultField {
                    name: "secretString".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "versionStages".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecretVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            secret_binary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretBinary").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
            secret_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretString").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            version_stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionStages").unwrap(),
            ),
        }
    }
}