/// A scheduled job that can publish a PubSub message or an HTTP request
/// every X interval of time, using a crontab format string.
///
///
/// To get more information about Job, see:
///
/// * [API documentation](https://cloud.google.com/scheduler/docs/reference/rest/)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/scheduler/)
///
/// ## Example Usage
///
/// ### Scheduler Job Pubsub
///
///
/// ```yaml
/// resources:
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: job-topic
///   job:
///     type: gcp:cloudscheduler:Job
///     properties:
///       name: test-job
///       description: test job
///       schedule: '*/2 * * * *'
///       pubsubTarget:
///         topicName: ${topic.id}
///         data:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: test
///             return: result
/// ```
/// ### Scheduler Job Http
///
///
/// ```yaml
/// resources:
///   job:
///     type: gcp:cloudscheduler:Job
///     properties:
///       name: test-job
///       description: test http job
///       schedule: '*/8 * * * *'
///       timeZone: America/New_York
///       attemptDeadline: 320s
///       retryConfig:
///         retryCount: 1
///       httpTarget:
///         httpMethod: POST
///         uri: https://example.com/
///         body:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: '{"foo":"bar"}'
///             return: result
///         headers:
///           Content-Type: application/json
/// ```
/// ### Scheduler Job Paused
///
///
/// ```yaml
/// resources:
///   job:
///     type: gcp:cloudscheduler:Job
///     properties:
///       paused: true
///       name: test-job
///       description: test http job with updated fields
///       schedule: '*/8 * * * *'
///       timeZone: America/New_York
///       attemptDeadline: 320s
///       retryConfig:
///         retryCount: 1
///       httpTarget:
///         httpMethod: POST
///         uri: https://example.com/ping
///         body:
///           fn::invoke:
///             function: std:base64encode
///             arguments:
///               input: '{"foo":"bar"}'
///             return: result
///         headers:
///           Content-Type: application/json
/// ```
/// ### Scheduler Job App Engine
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let job = job::create(
///         "job",
///         JobArgs::builder()
///             .app_engine_http_target(
///                 JobAppEngineHttpTarget::builder()
///                     .appEngineRouting(
///                         JobAppEngineHttpTargetAppEngineRouting::builder()
///                             .instance("my-instance-001")
///                             .service("web")
///                             .version("prod")
///                             .build_struct(),
///                     )
///                     .httpMethod("POST")
///                     .relativeUri("/ping")
///                     .build_struct(),
///             )
///             .attempt_deadline("320s")
///             .description("test app engine job")
///             .name("test-job")
///             .retry_config(
///                 JobRetryConfig::builder()
///                     .maxDoublings(2)
///                     .maxRetryDuration("10s")
///                     .minBackoffDuration("1s")
///                     .retryCount(3)
///                     .build_struct(),
///             )
///             .schedule("*/4 * * * *")
///             .time_zone("Europe/London")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Scheduler Job Oauth
///
///
/// ```yaml
/// resources:
///   job:
///     type: gcp:cloudscheduler:Job
///     properties:
///       name: test-job
///       description: test http job
///       schedule: '*/8 * * * *'
///       timeZone: America/New_York
///       attemptDeadline: 320s
///       httpTarget:
///         httpMethod: GET
///         uri: https://cloudscheduler.googleapis.com/v1/projects/my-project-name/locations/us-west1/jobs
///         oauthToken:
///           serviceAccountEmail: ${default.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
/// ### Scheduler Job Oidc
///
///
/// ```yaml
/// resources:
///   job:
///     type: gcp:cloudscheduler:Job
///     properties:
///       name: test-job
///       description: test http job
///       schedule: '*/8 * * * *'
///       timeZone: America/New_York
///       attemptDeadline: 320s
///       httpTarget:
///         httpMethod: GET
///         uri: https://example.com/ping
///         oidcToken:
///           serviceAccountEmail: ${default.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ## Import
///
/// Job can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/jobs/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Job can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudscheduler/job:Job default projects/{{project}}/locations/{{region}}/jobs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudscheduler/job:Job default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudscheduler/job:Job default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudscheduler/job:Job default {{name}}
/// ```
///
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// App Engine HTTP target.
        /// If the job providers a App Engine HTTP target the cron will
        /// send a request to the service instance
        /// Structure is documented below.
        #[builder(into, default)]
        pub app_engine_http_target: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudscheduler::JobAppEngineHttpTarget>,
        >,
        /// The deadline for job attempts. If the request handler does not respond by this deadline then the request is
        /// cancelled and the attempt is marked as a DEADLINE_EXCEEDED failure. The failed attempt can be viewed in
        /// execution logs. Cloud Scheduler will retry the job according to the RetryConfig.
        /// The allowed duration for this deadline is:
        /// * For HTTP targets, between 15 seconds and 30 minutes.
        /// * For App Engine HTTP targets, between 15 seconds and 24 hours.
        /// * **Note**: For PubSub targets, this field is ignored - setting it will introduce an unresolvable diff.
        /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s"
        #[builder(into, default)]
        pub attempt_deadline: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A human-readable description for the job.
        /// This string must not contain more than 500 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// HTTP target.
        /// If the job providers a http_target the cron will
        /// send a request to the targeted url
        /// Structure is documented below.
        #[builder(into, default)]
        pub http_target: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudscheduler::JobHttpTarget>,
        >,
        /// The name of the job.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets the job to a paused state. Jobs default to being enabled when this property is not set.
        #[builder(into, default)]
        pub paused: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Pub/Sub target
        /// If the job providers a Pub/Sub target the cron will publish
        /// a message to the provided topic
        /// Structure is documented below.
        #[builder(into, default)]
        pub pubsub_target: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudscheduler::JobPubsubTarget>,
        >,
        /// Region where the scheduler job resides. If it is not provided, this provider will use the provider default.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// By default, if a job does not complete successfully,
        /// meaning that an acknowledgement is not received from the handler,
        /// then it will be retried with exponential backoff according to the settings
        /// Structure is documented below.
        #[builder(into, default)]
        pub retry_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudscheduler::JobRetryConfig>,
        >,
        /// Describes the schedule on which the job will be executed.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the time zone to be used in interpreting schedule.
        /// The value of this field must be a time zone name from the tz database.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// App Engine HTTP target.
        /// If the job providers a App Engine HTTP target the cron will
        /// send a request to the service instance
        /// Structure is documented below.
        pub app_engine_http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudscheduler::JobAppEngineHttpTarget>,
        >,
        /// The deadline for job attempts. If the request handler does not respond by this deadline then the request is
        /// cancelled and the attempt is marked as a DEADLINE_EXCEEDED failure. The failed attempt can be viewed in
        /// execution logs. Cloud Scheduler will retry the job according to the RetryConfig.
        /// The allowed duration for this deadline is:
        /// * For HTTP targets, between 15 seconds and 30 minutes.
        /// * For App Engine HTTP targets, between 15 seconds and 24 hours.
        /// * **Note**: For PubSub targets, this field is ignored - setting it will introduce an unresolvable diff.
        /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s"
        pub attempt_deadline: pulumi_wasm_rust::Output<Option<String>>,
        /// A human-readable description for the job.
        /// This string must not contain more than 500 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP target.
        /// If the job providers a http_target the cron will
        /// send a request to the targeted url
        /// Structure is documented below.
        pub http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudscheduler::JobHttpTarget>,
        >,
        /// The name of the job.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Sets the job to a paused state. Jobs default to being enabled when this property is not set.
        pub paused: pulumi_wasm_rust::Output<bool>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Pub/Sub target
        /// If the job providers a Pub/Sub target the cron will publish
        /// a message to the provided topic
        /// Structure is documented below.
        pub pubsub_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudscheduler::JobPubsubTarget>,
        >,
        /// Region where the scheduler job resides. If it is not provided, this provider will use the provider default.
        pub region: pulumi_wasm_rust::Output<String>,
        /// By default, if a job does not complete successfully,
        /// meaning that an acknowledgement is not received from the handler,
        /// then it will be retried with exponential backoff according to the settings
        /// Structure is documented below.
        pub retry_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudscheduler::JobRetryConfig>,
        >,
        /// Describes the schedule on which the job will be executed.
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the job.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Specifies the time zone to be used in interpreting schedule.
        /// The value of this field must be a time zone name from the tz database.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_engine_http_target_binding = args
            .app_engine_http_target
            .get_output(context)
            .get_inner();
        let attempt_deadline_binding = args
            .attempt_deadline
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let http_target_binding = args.http_target.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let paused_binding = args.paused.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let pubsub_target_binding = args.pubsub_target.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let retry_config_binding = args.retry_config.get_output(context).get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudscheduler/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appEngineHttpTarget".into(),
                    value: &app_engine_http_target_binding,
                },
                register_interface::ObjectField {
                    name: "attemptDeadline".into(),
                    value: &attempt_deadline_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "httpTarget".into(),
                    value: &http_target_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "paused".into(),
                    value: &paused_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pubsubTarget".into(),
                    value: &pubsub_target_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "retryConfig".into(),
                    value: &retry_config_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobResult {
            app_engine_http_target: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appEngineHttpTarget"),
            ),
            attempt_deadline: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attemptDeadline"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            http_target: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpTarget"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            paused: pulumi_wasm_rust::__private::into_domain(o.extract_field("paused")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pubsub_target: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pubsubTarget"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            retry_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retryConfig"),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
        }
    }
}
