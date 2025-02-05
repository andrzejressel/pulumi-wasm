/// Resource for managing a Verified Access Logging Configuration.
///
/// ## Example Usage
///
/// ### With CloudWatch Logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .cloudwatchLogs(
///                         InstanceLoggingConfigurationAccessLogsCloudwatchLogs::builder()
///                             .enabled(true)
///                             .logGroup("${exampleAwsCloudwatchLogGroup.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Kinesis Data Firehose Logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .kinesisDataFirehose(
///                         InstanceLoggingConfigurationAccessLogsKinesisDataFirehose::builder()
///                             .deliveryStream(
///                                 "${exampleAwsKinesisFirehoseDeliveryStream.name}",
///                             )
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With S3 logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .s3(
///                         InstanceLoggingConfigurationAccessLogsS3::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .enabled(true)
///                             .prefix("example")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With all three logging options
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .cloudwatchLogs(
///                         InstanceLoggingConfigurationAccessLogsCloudwatchLogs::builder()
///                             .enabled(true)
///                             .logGroup("${exampleAwsCloudwatchLogGroup.id}")
///                             .build_struct(),
///                     )
///                     .kinesisDataFirehose(
///                         InstanceLoggingConfigurationAccessLogsKinesisDataFirehose::builder()
///                             .deliveryStream(
///                                 "${exampleAwsKinesisFirehoseDeliveryStream.name}",
///                             )
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .s3(
///                         InstanceLoggingConfigurationAccessLogsS3::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With `include_trust_context`
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .includeTrustContext(true)
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With `log_version`
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_logging_configuration::create(
///         "example",
///         InstanceLoggingConfigurationArgs::builder()
///             .access_logs(
///                 InstanceLoggingConfigurationAccessLogs::builder()
///                     .logVersion("ocsf-1.0.0-rc.2")
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Access Logging Configuration using the Verified Access Instance `id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedaccess/instanceLoggingConfiguration:InstanceLoggingConfiguration example vai-1234567890abcdef0
/// ```
pub mod instance_logging_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceLoggingConfigurationArgs {
        /// A block that specifies the configuration options for Verified Access instances. Detailed below.
        #[builder(into)]
        pub access_logs: pulumi_wasm_rust::InputOrOutput<
            super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogs,
        >,
        /// The ID of the Verified Access instance.
        #[builder(into)]
        pub verifiedaccess_instance_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceLoggingConfigurationResult {
        /// A block that specifies the configuration options for Verified Access instances. Detailed below.
        pub access_logs: pulumi_wasm_rust::Output<
            super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogs,
        >,
        /// The ID of the Verified Access instance.
        pub verifiedaccess_instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceLoggingConfigurationArgs,
    ) -> InstanceLoggingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_logs_binding = args.access_logs.get_output(context).get_inner();
        let verifiedaccess_instance_id_binding = args
            .verifiedaccess_instance_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/instanceLoggingConfiguration:InstanceLoggingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLogs".into(),
                    value: &access_logs_binding,
                },
                register_interface::ObjectField {
                    name: "verifiedaccessInstanceId".into(),
                    value: &verifiedaccess_instance_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceLoggingConfigurationResult {
            access_logs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessLogs"),
            ),
            verifiedaccess_instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verifiedaccessInstanceId"),
            ),
        }
    }
}
