/// Provides an Amazon Connect Instance Storage Config resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Storage Config Kinesis Firehose Config
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_storage_config::create(
///         "example",
///         InstanceStorageConfigArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .resource_type("CONTACT_TRACE_RECORDS")
///             .storage_config(
///                 InstanceStorageConfigStorageConfig::builder()
///                     .kinesisFirehoseConfig(
///                         InstanceStorageConfigStorageConfigKinesisFirehoseConfig::builder()
///                             .firehoseArn(
///                                 "${exampleAwsKinesisFirehoseDeliveryStream.arn}",
///                             )
///                             .build_struct(),
///                     )
///                     .storageType("KINESIS_FIREHOSE")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Storage Config Kinesis Stream Config
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_storage_config::create(
///         "example",
///         InstanceStorageConfigArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .resource_type("CONTACT_TRACE_RECORDS")
///             .storage_config(
///                 InstanceStorageConfigStorageConfig::builder()
///                     .kinesisStreamConfig(
///                         InstanceStorageConfigStorageConfigKinesisStreamConfig::builder()
///                             .streamArn("${exampleAwsKinesisStream.arn}")
///                             .build_struct(),
///                     )
///                     .storageType("KINESIS_STREAM")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Storage Config Kinesis Video Stream Config
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_storage_config::create(
///         "example",
///         InstanceStorageConfigArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .resource_type("MEDIA_STREAMS")
///             .storage_config(
///                 InstanceStorageConfigStorageConfig::builder()
///                     .kinesisVideoStreamConfig(
///                         InstanceStorageConfigStorageConfigKinesisVideoStreamConfig::builder()
///                             .encryptionConfig(
///                                 InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig::builder()
///                                     .encryptionType("KMS")
///                                     .keyId("${exampleAwsKmsKey.arn}")
///                                     .build_struct(),
///                             )
///                             .prefix("example")
///                             .retentionPeriodHours(3)
///                             .build_struct(),
///                     )
///                     .storageType("KINESIS_VIDEO_STREAM")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Storage Config S3 Config
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_storage_config::create(
///         "example",
///         InstanceStorageConfigArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .resource_type("CHAT_TRANSCRIPTS")
///             .storage_config(
///                 InstanceStorageConfigStorageConfig::builder()
///                     .s3Config(
///                         InstanceStorageConfigStorageConfigS3Config::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .bucketPrefix("example")
///                             .build_struct(),
///                     )
///                     .storageType("S3")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Storage Config S3 Config with Encryption Config
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_storage_config::create(
///         "example",
///         InstanceStorageConfigArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .resource_type("CHAT_TRANSCRIPTS")
///             .storage_config(
///                 InstanceStorageConfigStorageConfig::builder()
///                     .s3Config(
///                         InstanceStorageConfigStorageConfigS3Config::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .bucketPrefix("example")
///                             .encryptionConfig(
///                                 InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig::builder()
///                                     .encryptionType("KMS")
///                                     .keyId("${exampleAwsKmsKey.arn}")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .storageType("S3")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Instance Storage Configs using the `instance_id`, `association_id`, and `resource_type` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/instanceStorageConfig:InstanceStorageConfig example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5:CHAT_TRANSCRIPTS
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_storage_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceStorageConfigArgs {
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` | `SCREEN_RECORDINGS`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the storage configuration options for the Connect Instance. Documented below.
        #[builder(into)]
        pub storage_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::connect::InstanceStorageConfigStorageConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceStorageConfigResult {
        /// The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// A valid resource type. Valid Values: `AGENT_EVENTS` | `ATTACHMENTS` | `CALL_RECORDINGS` | `CHAT_TRANSCRIPTS` | `CONTACT_EVALUATIONS` | `CONTACT_TRACE_RECORDS` | `MEDIA_STREAMS` | `REAL_TIME_CONTACT_ANALYSIS_SEGMENTS` | `SCHEDULED_REPORTS` | `SCREEN_RECORDINGS`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage configuration options for the Connect Instance. Documented below.
        pub storage_config: pulumi_gestalt_rust::Output<
            super::super::types::connect::InstanceStorageConfigStorageConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceStorageConfigArgs,
    ) -> InstanceStorageConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let storage_config_binding = args.storage_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/instanceStorageConfig:InstanceStorageConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfig".into(),
                    value: &storage_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceStorageConfigResult {
            association_id: o.get_field("associationId"),
            instance_id: o.get_field("instanceId"),
            resource_type: o.get_field("resourceType"),
            storage_config: o.get_field("storageConfig"),
        }
    }
}
