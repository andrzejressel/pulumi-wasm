/// Manages a Glue Security Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_configuration::create(
///         "example",
///         SecurityConfigurationArgs::builder()
///             .encryption_configuration(
///                 SecurityConfigurationEncryptionConfiguration::builder()
///                     .cloudwatchEncryption(
///                         SecurityConfigurationEncryptionConfigurationCloudwatchEncryption::builder()
///                             .cloudwatchEncryptionMode("DISABLED")
///                             .build_struct(),
///                     )
///                     .jobBookmarksEncryption(
///                         SecurityConfigurationEncryptionConfigurationJobBookmarksEncryption::builder()
///                             .jobBookmarksEncryptionMode("DISABLED")
///                             .build_struct(),
///                     )
///                     .s3Encryption(
///                         SecurityConfigurationEncryptionConfigurationS3Encryption::builder()
///                             .kmsKeyArn("${exampleAwsKmsKey.arn}")
///                             .s3EncryptionMode("SSE-KMS")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Security Configurations using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/securityConfiguration:SecurityConfiguration example example
/// ```
pub mod security_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityConfigurationArgs {
        /// Configuration block containing encryption configuration. Detailed below.
        #[builder(into)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityConfigurationResult {
        /// Configuration block containing encryption configuration. Detailed below.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SecurityConfigurationArgs,
    ) -> SecurityConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/securityConfiguration:SecurityConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityConfigurationResult {
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
