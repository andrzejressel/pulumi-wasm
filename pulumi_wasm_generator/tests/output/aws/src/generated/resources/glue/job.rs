/// Provides a Glue Job resource.
///
/// > Glue functionality, such as monitoring and logging of jobs, is typically managed with the `default_arguments` argument. See the [Special Parameters Used by AWS Glue](https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html) topic in the Glue developer guide for additional information.
///
/// ## Example Usage
///
/// ### Python Job
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = job::create(
///         "example",
///         JobArgs::builder()
///             .command(
///                 JobCommand::builder()
///                     .scriptLocation("s3://${exampleAwsS3Bucket.bucket}/example.py")
///                     .build_struct(),
///             )
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Ray Job
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = job::create(
///         "example",
///         JobArgs::builder()
///             .command(
///                 JobCommand::builder()
///                     .name("glueray")
///                     .pythonVersion("3.9")
///                     .runtime("Ray2.4")
///                     .scriptLocation("s3://${exampleAwsS3Bucket.bucket}/example.py")
///                     .build_struct(),
///             )
///             .glue_version("4.0")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .worker_type("Z.2X")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Scala Job
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:Job
///     properties:
///       name: example
///       roleArn: ${exampleAwsIamRole.arn}
///       command:
///         scriptLocation: s3://${exampleAwsS3Bucket.bucket}/example.scala
///       defaultArguments:
///         --job-language: scala
/// ```
///
/// ### Streaming Job
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = job::create(
///         "example",
///         JobArgs::builder()
///             .command(
///                 JobCommand::builder()
///                     .name("gluestreaming")
///                     .scriptLocation("s3://${exampleAwsS3Bucket.bucket}/example.script")
///                     .build_struct(),
///             )
///             .name("example streaming job")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enabling CloudWatch Logs and Metrics
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: example
///       retentionInDays: 14
///   exampleJob:
///     type: aws:glue:Job
///     name: example
///     properties:
///       defaultArguments:
///         --continuous-log-logGroup: ${example.name}
///         --enable-continuous-cloudwatch-log: 'true'
///         --enable-continuous-log-filter: 'true'
///         --enable-metrics:
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Jobs using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/job:Job MyJob MyJob
/// ```
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// The command of the job. Defined below.
        #[builder(into)]
        pub command: pulumi_wasm_rust::Output<super::super::types::glue::JobCommand>,
        /// The list of connections used for this job.
        #[builder(into, default)]
        pub connections: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The map of default arguments for this job. You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes. For information about how to specify and consume your own Job arguments, see the [Calling AWS Glue APIs in Python](http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html) topic in the developer guide. For information about the key-value pairs that AWS Glue consumes to set up your job, see the [Special Parameters Used by AWS Glue](http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-glue-arguments.html) topic in the developer guide.
        #[builder(into, default)]
        pub default_arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the job.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the job is run with a standard or flexible execution class. The standard execution class is ideal for time-sensitive workloads that require fast job startup and dedicated resources. Valid value: `FLEX`, `STANDARD`.
        #[builder(into, default)]
        pub execution_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Execution property of the job. Defined below.
        #[builder(into, default)]
        pub execution_property: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::JobExecutionProperty>,
        >,
        /// The version of glue to use, for example "1.0". Ray jobs should set this to 4.0 or greater. For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        #[builder(into, default)]
        pub glue_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether job run queuing is enabled for the job runs for this job. A value of true means job run queuing is enabled for the job runs. If false or not populated, the job runs will not be considered for queueing.
        #[builder(into, default)]
        pub job_run_queuing_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the day of the week and hour for the maintenance window for streaming jobs.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. `Required` when `pythonshell` is set, accept either `0.0625` or `1.0`. Use `number_of_workers` and `worker_type` arguments instead with `glue_version` `2.0` and above.
        #[builder(into, default)]
        pub max_capacity: pulumi_wasm_rust::Output<Option<f64>>,
        /// The maximum number of times to retry this job if it fails.
        #[builder(into, default)]
        pub max_retries: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this job. It must be unique in your account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Non-overridable arguments for this job, specified as name-value pairs.
        #[builder(into, default)]
        pub non_overridable_arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Notification property of the job. Defined below.
        #[builder(into, default)]
        pub notification_property: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::JobNotificationProperty>,
        >,
        /// The number of workers of a defined workerType that are allocated when a job runs.
        #[builder(into, default)]
        pub number_of_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ARN of the IAM role associated with this job.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Configuration to be associated with the job.
        #[builder(into, default)]
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The job timeout in minutes. The default is 2880 minutes (48 hours) for `glueetl` and `pythonshell` jobs, and null (unlimited) for `gluestreaming` jobs.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, G.2X, or G.025X for Spark jobs. Accepts the value Z.2X for Ray jobs.
        /// * For the Standard worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.
        /// * For the G.1X worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. Recommended for memory-intensive jobs.
        /// * For the G.2X worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. Recommended for memory-intensive jobs.
        /// * For the G.4X worker type, each worker maps to 4 DPU (16 vCPUs, 64 GB of memory) with 256GB disk (approximately 235GB free), and provides 1 executor per worker. Recommended for memory-intensive jobs. Only available for Glue version 3.0. Available AWS Regions: US East (Ohio), US East (N. Virginia), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), and Europe (Stockholm).
        /// * For the G.8X worker type, each worker maps to 8 DPU (32 vCPUs, 128 GB of memory) with 512GB disk (approximately 487GB free), and provides 1 executor per worker. Recommended for memory-intensive jobs. Only available for Glue version 3.0. Available AWS Regions: US East (Ohio), US East (N. Virginia), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), and Europe (Stockholm).
        /// * For the G.025X worker type, each worker maps to 0.25 DPU (2 vCPU, 4GB of memory, 64 GB disk), and provides 1 executor per worker. Recommended for low volume streaming jobs. Only available for Glue version 3.0.
        /// * For the Z.2X worker type, each worker maps to 2 M-DPU (8vCPU, 64 GB of m emory, 128 GB disk), and provides up to 8 Ray workers based on the autoscaler.
        #[builder(into, default)]
        pub worker_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// Amazon Resource Name (ARN) of Glue Job
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The command of the job. Defined below.
        pub command: pulumi_wasm_rust::Output<super::super::types::glue::JobCommand>,
        /// The list of connections used for this job.
        pub connections: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The map of default arguments for this job. You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes. For information about how to specify and consume your own Job arguments, see the [Calling AWS Glue APIs in Python](http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html) topic in the developer guide. For information about the key-value pairs that AWS Glue consumes to set up your job, see the [Special Parameters Used by AWS Glue](http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-glue-arguments.html) topic in the developer guide.
        pub default_arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Description of the job.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the job is run with a standard or flexible execution class. The standard execution class is ideal for time-sensitive workloads that require fast job startup and dedicated resources. Valid value: `FLEX`, `STANDARD`.
        pub execution_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Execution property of the job. Defined below.
        pub execution_property: pulumi_wasm_rust::Output<
            super::super::types::glue::JobExecutionProperty,
        >,
        /// The version of glue to use, for example "1.0". Ray jobs should set this to 4.0 or greater. For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        pub glue_version: pulumi_wasm_rust::Output<String>,
        /// Specifies whether job run queuing is enabled for the job runs for this job. A value of true means job run queuing is enabled for the job runs. If false or not populated, the job runs will not be considered for queueing.
        pub job_run_queuing_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the day of the week and hour for the maintenance window for streaming jobs.
        pub maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. `Required` when `pythonshell` is set, accept either `0.0625` or `1.0`. Use `number_of_workers` and `worker_type` arguments instead with `glue_version` `2.0` and above.
        pub max_capacity: pulumi_wasm_rust::Output<f64>,
        /// The maximum number of times to retry this job if it fails.
        pub max_retries: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this job. It must be unique in your account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Non-overridable arguments for this job, specified as name-value pairs.
        pub non_overridable_arguments: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Notification property of the job. Defined below.
        pub notification_property: pulumi_wasm_rust::Output<
            super::super::types::glue::JobNotificationProperty,
        >,
        /// The number of workers of a defined workerType that are allocated when a job runs.
        pub number_of_workers: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the IAM role associated with this job.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Security Configuration to be associated with the job.
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The job timeout in minutes. The default is 2880 minutes (48 hours) for `glueetl` and `pythonshell` jobs, and null (unlimited) for `gluestreaming` jobs.
        pub timeout: pulumi_wasm_rust::Output<i32>,
        /// The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, G.2X, or G.025X for Spark jobs. Accepts the value Z.2X for Ray jobs.
        /// * For the Standard worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.
        /// * For the G.1X worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. Recommended for memory-intensive jobs.
        /// * For the G.2X worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. Recommended for memory-intensive jobs.
        /// * For the G.4X worker type, each worker maps to 4 DPU (16 vCPUs, 64 GB of memory) with 256GB disk (approximately 235GB free), and provides 1 executor per worker. Recommended for memory-intensive jobs. Only available for Glue version 3.0. Available AWS Regions: US East (Ohio), US East (N. Virginia), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), and Europe (Stockholm).
        /// * For the G.8X worker type, each worker maps to 8 DPU (32 vCPUs, 128 GB of memory) with 512GB disk (approximately 487GB free), and provides 1 executor per worker. Recommended for memory-intensive jobs. Only available for Glue version 3.0. Available AWS Regions: US East (Ohio), US East (N. Virginia), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), and Europe (Stockholm).
        /// * For the G.025X worker type, each worker maps to 0.25 DPU (2 vCPU, 4GB of memory, 64 GB disk), and provides 1 executor per worker. Recommended for low volume streaming jobs. Only available for Glue version 3.0.
        /// * For the Z.2X worker type, each worker maps to 2 M-DPU (8vCPU, 64 GB of m emory, 128 GB disk), and provides up to 8 Ray workers based on the autoscaler.
        pub worker_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobArgs) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let command_binding = args.command.get_inner();
        let connections_binding = args.connections.get_inner();
        let default_arguments_binding = args.default_arguments.get_inner();
        let description_binding = args.description.get_inner();
        let execution_class_binding = args.execution_class.get_inner();
        let execution_property_binding = args.execution_property.get_inner();
        let glue_version_binding = args.glue_version.get_inner();
        let job_run_queuing_enabled_binding = args.job_run_queuing_enabled.get_inner();
        let maintenance_window_binding = args.maintenance_window.get_inner();
        let max_capacity_binding = args.max_capacity.get_inner();
        let max_retries_binding = args.max_retries.get_inner();
        let name_binding = args.name.get_inner();
        let non_overridable_arguments_binding = args
            .non_overridable_arguments
            .get_inner();
        let notification_property_binding = args.notification_property.get_inner();
        let number_of_workers_binding = args.number_of_workers.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let security_configuration_binding = args.security_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let worker_type_binding = args.worker_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/job:Job".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "command".into(),
                    value: &command_binding,
                },
                register_interface::ObjectField {
                    name: "connections".into(),
                    value: &connections_binding,
                },
                register_interface::ObjectField {
                    name: "defaultArguments".into(),
                    value: &default_arguments_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "executionClass".into(),
                    value: &execution_class_binding,
                },
                register_interface::ObjectField {
                    name: "executionProperty".into(),
                    value: &execution_property_binding,
                },
                register_interface::ObjectField {
                    name: "glueVersion".into(),
                    value: &glue_version_binding,
                },
                register_interface::ObjectField {
                    name: "jobRunQueuingEnabled".into(),
                    value: &job_run_queuing_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "maxCapacity".into(),
                    value: &max_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "maxRetries".into(),
                    value: &max_retries_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nonOverridableArguments".into(),
                    value: &non_overridable_arguments_binding,
                },
                register_interface::ObjectField {
                    name: "notificationProperty".into(),
                    value: &notification_property_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfWorkers".into(),
                    value: &number_of_workers_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "workerType".into(),
                    value: &worker_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "command".into(),
                },
                register_interface::ResultField {
                    name: "connections".into(),
                },
                register_interface::ResultField {
                    name: "defaultArguments".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "executionClass".into(),
                },
                register_interface::ResultField {
                    name: "executionProperty".into(),
                },
                register_interface::ResultField {
                    name: "glueVersion".into(),
                },
                register_interface::ResultField {
                    name: "jobRunQueuingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "maxCapacity".into(),
                },
                register_interface::ResultField {
                    name: "maxRetries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nonOverridableArguments".into(),
                },
                register_interface::ResultField {
                    name: "notificationProperty".into(),
                },
                register_interface::ResultField {
                    name: "numberOfWorkers".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "securityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "workerType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            command: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("command").unwrap(),
            ),
            connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connections").unwrap(),
            ),
            default_arguments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultArguments").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            execution_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionClass").unwrap(),
            ),
            execution_property: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionProperty").unwrap(),
            ),
            glue_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("glueVersion").unwrap(),
            ),
            job_run_queuing_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobRunQueuingEnabled").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            max_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxCapacity").unwrap(),
            ),
            max_retries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRetries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            non_overridable_arguments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonOverridableArguments").unwrap(),
            ),
            notification_property: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationProperty").unwrap(),
            ),
            number_of_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfWorkers").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            security_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            worker_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerType").unwrap(),
            ),
        }
    }
}