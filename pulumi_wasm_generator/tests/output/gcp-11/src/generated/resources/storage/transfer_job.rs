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
pub mod transfer_job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransferJobArgs {
        /// Unique description to identify the Transfer Job.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies the Event-driven transfer options. Event-driven transfers listen to an event stream to transfer updated files. Structure documented below Either `event_stream` or `schedule` must be set.
        #[builder(into, default)]
        pub event_stream: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobEventStream>,
        >,
        /// The name of the Transfer Job. This name must start with "transferJobs/" prefix and end with a letter or a number, and should be no more than 128 characters ( `transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For transfers involving PosixFilesystem, this name must start with transferJobs/OPI specifically ( `transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For all other transfer types, this name must not start with transferJobs/OPI. Default the provider will assign a random unique name with `transferJobs/{{name}}` format, where `name` is a numeric value.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Notification configuration. This is not supported for transfers involving PosixFilesystem. Structure documented below.
        #[builder(into, default)]
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobNotificationConfig>,
        >,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Schedule specification defining when the Transfer Job should be scheduled to start, end and what time to run. Structure documented below. Either `schedule` or `event_stream` must be set.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobSchedule>,
        >,
        /// Status of the job. Default: `ENABLED`. **NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.**
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Transfer specification. Structure documented below.
        ///
        /// - - -
        #[builder(into)]
        pub transfer_spec: pulumi_wasm_rust::Output<
            super::super::types::storage::TransferJobTransferSpec,
        >,
    }
    #[allow(dead_code)]
    pub struct TransferJobResult {
        /// When the Transfer Job was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// When the Transfer Job was deleted.
        pub deletion_time: pulumi_wasm_rust::Output<String>,
        /// Unique description to identify the Transfer Job.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies the Event-driven transfer options. Event-driven transfers listen to an event stream to transfer updated files. Structure documented below Either `event_stream` or `schedule` must be set.
        pub event_stream: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobEventStream>,
        >,
        /// When the Transfer Job was last modified.
        pub last_modification_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Transfer Job. This name must start with "transferJobs/" prefix and end with a letter or a number, and should be no more than 128 characters ( `transferJobs/^(?!OPI)[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For transfers involving PosixFilesystem, this name must start with transferJobs/OPI specifically ( `transferJobs/OPI^[A-Za-z0-9-._~]*[A-Za-z0-9]$` ). For all other transfer types, this name must not start with transferJobs/OPI. Default the provider will assign a random unique name with `transferJobs/{{name}}` format, where `name` is a numeric value.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Notification configuration. This is not supported for transfers involving PosixFilesystem. Structure documented below.
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobNotificationConfig>,
        >,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Schedule specification defining when the Transfer Job should be scheduled to start, end and what time to run. Structure documented below. Either `schedule` or `event_stream` must be set.
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::TransferJobSchedule>,
        >,
        /// Status of the job. Default: `ENABLED`. **NOTE: The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.**
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Transfer specification. Structure documented below.
        ///
        /// - - -
        pub transfer_spec: pulumi_wasm_rust::Output<
            super::super::types::storage::TransferJobTransferSpec,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TransferJobArgs) -> TransferJobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let event_stream_binding = args.event_stream.get_inner();
        let name_binding = args.name.get_inner();
        let notification_config_binding = args.notification_config.get_inner();
        let project_binding = args.project.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let status_binding = args.status.get_inner();
        let transfer_spec_binding = args.transfer_spec.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/transferJob:TransferJob".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventStream".into(),
                    value: &event_stream_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationConfig".into(),
                    value: &notification_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "transferSpec".into(),
                    value: &transfer_spec_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventStream".into(),
                },
                register_interface::ResultField {
                    name: "lastModificationTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "transferSpec".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TransferJobResult {
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            deletion_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            event_stream: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventStream").unwrap(),
            ),
            last_modification_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModificationTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            transfer_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transferSpec").unwrap(),
            ),
        }
    }
}
