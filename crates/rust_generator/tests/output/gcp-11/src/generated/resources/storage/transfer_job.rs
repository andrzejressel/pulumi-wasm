/// Creates a new Transfer Job in Google Cloud Storage Transfer.
///
/// To get more information about Google Cloud Storage Transfer, see:
///
/// * [Overview](https://cloud.google.com/storage-transfer/docs/overview)
/// * [API documentation](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/transferJobs)
/// * How-to Guides
///     * [Configuring Access to Data Sources and Sinks](https://cloud.google.com/storage-transfer/docs/configure-access)
///
/// ## Example Usage
///
/// Example creating a nightly Transfer Job from an AWS S3 Bucket to a GCS bucket.
///
/// ```yaml
/// resources:
///   s3-backup-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${awsS3Bucket}-backup
///       storageClass: NEARLINE
///       project: ${project}
///       location: US
///   s3-backup-bucketBucketIAMMember:
///     type: gcp:storage:BucketIAMMember
///     name: s3-backup-bucket
///     properties:
///       bucket: ${["s3-backup-bucket"].name}
///       role: roles/storage.admin
///       member: serviceAccount:${default.email}
///     options:
///       dependsOn:
///         - ${["s3-backup-bucket"]}
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: ${pubsubTopicName}
///   notificationConfig:
///     type: gcp:pubsub:TopicIAMMember
///     name: notification_config
///     properties:
///       topic: ${topic.id}
///       role: roles/pubsub.publisher
///       member: serviceAccount:${default.email}
///   s3-bucket-nightly-backup:
///     type: gcp:storage:TransferJob
///     properties:
///       description: Nightly backup of S3 bucket
///       project: ${project}
///       transferSpec:
///         objectConditions:
///           maxTimeElapsedSinceLastModification: 600s
///           excludePrefixes:
///             - requests.gz
///         transferOptions:
///           deleteObjectsUniqueInSink: false
///         awsS3DataSource:
///           bucketName: ${awsS3Bucket}
///           awsAccessKey:
///             accessKeyId: ${awsAccessKey}
///             secretAccessKey: ${awsSecretKey}
///         gcsDataSink:
///           bucketName: ${["s3-backup-bucket"].name}
///           path: foo/bar/
///       schedule:
///         scheduleStartDate:
///           year: 2018
///           month: 10
///           day: 1
///         scheduleEndDate:
///           year: 2019
///           month: 1
///           day: 15
///         startTimeOfDay:
///           hours: 23
///           minutes: 30
///           seconds: 0
///           nanos: 0
///         repeatInterval: 604800s
///       notificationConfig:
///         pubsubTopic: ${topic.id}
///         eventTypes:
///           - TRANSFER_OPERATION_SUCCESS
///           - TRANSFER_OPERATION_FAILED
///         payloadFormat: JSON
///     options:
///       dependsOn:
///         - ${["s3-backup-bucketBucketIAMMember"]}
///         - ${notificationConfig}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:storage:getTransferProjectServiceAccount
///       arguments:
///         project: ${project}
/// ```
///
/// ## Import
///
/// Storage Transfer Jobs can be imported using the Transfer Job's `project` and `name` (without the `transferJob/` prefix), e.g.
///
/// * `{{project_id}}/{{name}}`, where `name` is a numeric value.
///
/// When using the `pulumi import` command, Storage Transfer Jobs can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/transferJob:TransferJob default {{project_id}}/123456789
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod transfer_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransferJobArgs {
        /// Unique description to identify the Transfer Job.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Event-driven transfer options. Event-driven transfers listen to an event stream to transfer updated files. Structure documented below Either `event_stream` or `schedule` must be set.
        #[builder(into, default)]
        pub event_stream: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::TransferJobEventStream>,
        >,
        /// The name of the Transfer Job. This name must start with "transferJobs/" prefix and end with a letter or a number, and should be no more than 128 characters ( `transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For transfers involving PosixFilesystem, this name must start with transferJobs/OPI specifically ( `transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For all other transfer types, this name must not start with transferJobs/OPI. Default the provider will assign a random unique name with `transferJobs/{{name}}` format, where `name` is a numeric value.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Notification configuration. This is not supported for transfers involving PosixFilesystem. Structure documented below.
        #[builder(into, default)]
        pub notification_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::TransferJobNotificationConfig>,
        >,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Schedule specification defining when the Transfer Job should be scheduled to start, end and what time to run. Structure documented below. Either `schedule` or `event_stream` must be set.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::TransferJobSchedule>,
        >,
        /// Status of the job. Default: `ENABLED`. **NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.**
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Transfer specification. Structure documented below.
        ///
        /// - - -
        #[builder(into)]
        pub transfer_spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::storage::TransferJobTransferSpec,
        >,
    }
    #[allow(dead_code)]
    pub struct TransferJobResult {
        /// When the Transfer Job was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// When the Transfer Job was deleted.
        pub deletion_time: pulumi_gestalt_rust::Output<String>,
        /// Unique description to identify the Transfer Job.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Event-driven transfer options. Event-driven transfers listen to an event stream to transfer updated files. Structure documented below Either `event_stream` or `schedule` must be set.
        pub event_stream: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::TransferJobEventStream>,
        >,
        /// When the Transfer Job was last modified.
        pub last_modification_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Transfer Job. This name must start with "transferJobs/" prefix and end with a letter or a number, and should be no more than 128 characters ( `transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For transfers involving PosixFilesystem, this name must start with transferJobs/OPI specifically ( `transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For all other transfer types, this name must not start with transferJobs/OPI. Default the provider will assign a random unique name with `transferJobs/{{name}}` format, where `name` is a numeric value.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Notification configuration. This is not supported for transfers involving PosixFilesystem. Structure documented below.
        pub notification_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::TransferJobNotificationConfig>,
        >,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Schedule specification defining when the Transfer Job should be scheduled to start, end and what time to run. Structure documented below. Either `schedule` or `event_stream` must be set.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::TransferJobSchedule>,
        >,
        /// Status of the job. Default: `ENABLED`. **NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.**
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Transfer specification. Structure documented below.
        ///
        /// - - -
        pub transfer_spec: pulumi_gestalt_rust::Output<
            super::super::types::storage::TransferJobTransferSpec,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TransferJobArgs,
    ) -> TransferJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_stream_binding = args.event_stream.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_config_binding = args.notification_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let status_binding = args.status.get_output(context);
        let transfer_spec_binding = args.transfer_spec.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/transferJob:TransferJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventStream".into(),
                    value: event_stream_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationConfig".into(),
                    value: notification_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transferSpec".into(),
                    value: transfer_spec_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TransferJobResult {
            creation_time: o.get_field("creationTime"),
            deletion_time: o.get_field("deletionTime"),
            description: o.get_field("description"),
            event_stream: o.get_field("eventStream"),
            last_modification_time: o.get_field("lastModificationTime"),
            name: o.get_field("name"),
            notification_config: o.get_field("notificationConfig"),
            project: o.get_field("project"),
            schedule: o.get_field("schedule"),
            status: o.get_field("status"),
            transfer_spec: o.get_field("transferSpec"),
        }
    }
}
