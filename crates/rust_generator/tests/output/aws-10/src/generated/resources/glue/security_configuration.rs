/// Manages a Glue Security Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityConfigurationArgs {
        /// Configuration block containing encryption configuration. Detailed below.
        #[builder(into)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityConfigurationResult {
        /// Configuration block containing encryption configuration. Detailed below.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityConfigurationArgs,
    ) -> SecurityConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/securityConfiguration:SecurityConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: encryption_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityConfigurationResult {
            encryption_configuration: o.get_field("encryptionConfiguration"),
            name: o.get_field("name"),
        }
    }
}
