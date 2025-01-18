/// A named resource to which messages are sent by publishers.
///
///
///
/// ## Example Usage
///
/// ### Queue Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = queue::create(
///         "default",
///         QueueArgs::builder()
///             .location("us-central1")
///             .name("cloud-tasks-queue-test")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloud Tasks Queue Advanced
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let advancedConfiguration = queue::create(
///         "advancedConfiguration",
///         QueueArgs::builder()
///             .app_engine_routing_override(
///                 QueueAppEngineRoutingOverride::builder()
///                     .instance("test")
///                     .service("worker")
///                     .version("1.0")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .name("instance-name")
///             .rate_limits(
///                 QueueRateLimits::builder()
///                     .maxConcurrentDispatches(3)
///                     .maxDispatchesPerSecond(2)
///                     .build_struct(),
///             )
///             .retry_config(
///                 QueueRetryConfig::builder()
///                     .maxAttempts(5)
///                     .maxBackoff("3s")
///                     .maxDoublings(1)
///                     .maxRetryDuration("4s")
///                     .minBackoff("2s")
///                     .build_struct(),
///             )
///             .stackdriver_logging_config(
///                 QueueStackdriverLoggingConfig::builder()
///                     .samplingRatio(0.9)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloud Tasks Queue Http Target Oidc
///
///
/// ```yaml
/// resources:
///   httpTargetOidc:
///     type: gcp:cloudtasks:Queue
///     name: http_target_oidc
///     properties:
///       name: cloud-tasks-queue-http-target-oidc
///       location: us-central1
///       httpTarget:
///         httpMethod: POST
///         uriOverride:
///           scheme: HTTPS
///           host: oidc.example.com
///           port: 8443
///           pathOverride:
///             path: /users/1234
///           queryOverride:
///             queryParams: qparam1=123&qparam2=456
///           uriOverrideEnforceMode: IF_NOT_EXISTS
///         headerOverrides:
///           - header:
///               key: AddSomethingElse
///               value: MyOtherValue
///           - header:
///               key: AddMe
///               value: MyValue
///         oidcToken:
///           serviceAccountEmail: ${oidcServiceAccount.email}
///           audience: https://oidc.example.com
///   oidcServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: oidc_service_account
///     properties:
///       accountId: example-oidc
///       displayName: Tasks Queue OIDC Service Account
/// ```
/// ### Cloud Tasks Queue Http Target Oauth
///
///
/// ```yaml
/// resources:
///   httpTargetOauth:
///     type: gcp:cloudtasks:Queue
///     name: http_target_oauth
///     properties:
///       name: cloud-tasks-queue-http-target-oauth
///       location: us-central1
///       httpTarget:
///         httpMethod: POST
///         uriOverride:
///           scheme: HTTPS
///           host: oauth.example.com
///           port: 8443
///           pathOverride:
///             path: /users/1234
///           queryOverride:
///             queryParams: qparam1=123&qparam2=456
///           uriOverrideEnforceMode: IF_NOT_EXISTS
///         headerOverrides:
///           - header:
///               key: AddSomethingElse
///               value: MyOtherValue
///           - header:
///               key: AddMe
///               value: MyValue
///         oauthToken:
///           serviceAccountEmail: ${oauthServiceAccount.email}
///           scope: openid https://www.googleapis.com/auth/userinfo.email
///   oauthServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: oauth_service_account
///     properties:
///       accountId: example-oauth
///       displayName: Tasks Queue OAuth Service Account
/// ```
///
/// ## Import
///
/// Queue can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/queues/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Queue can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudtasks/queue:Queue default projects/{{project}}/locations/{{location}}/queues/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudtasks/queue:Queue default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudtasks/queue:Queue default {{location}}/{{name}}
/// ```
///
pub mod queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// Overrides for task-level appEngineRouting. These settings apply only
        /// to App Engine tasks in this queue
        /// Structure is documented below.
        #[builder(into, default)]
        pub app_engine_routing_override: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueAppEngineRoutingOverride>,
        >,
        /// Modifies HTTP target for HTTP tasks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueHttpTarget>,
        >,
        /// The location of the queue
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The queue name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Rate limits for task dispatches.
        /// The queue's actual dispatch rate is the result of:
        /// * Number of tasks in the queue
        /// * User-specified throttling: rateLimits, retryConfig, and the queue's state.
        /// * System throttling due to 429 (Too Many Requests) or 503 (Service
        /// Unavailable) responses from the worker, high error rates, or to
        /// smooth sudden large traffic spikes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rate_limits: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueRateLimits>,
        >,
        /// Settings that determine the retry behavior.
        /// Structure is documented below.
        #[builder(into, default)]
        pub retry_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueRetryConfig>,
        >,
        /// Configuration options for writing logs to Stackdriver Logging.
        /// Structure is documented below.
        #[builder(into, default)]
        pub stackdriver_logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueStackdriverLoggingConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// Overrides for task-level appEngineRouting. These settings apply only
        /// to App Engine tasks in this queue
        /// Structure is documented below.
        pub app_engine_routing_override: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueAppEngineRoutingOverride>,
        >,
        /// Modifies HTTP target for HTTP tasks.
        /// Structure is documented below.
        pub http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueHttpTarget>,
        >,
        /// The location of the queue
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The queue name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Rate limits for task dispatches.
        /// The queue's actual dispatch rate is the result of:
        /// * Number of tasks in the queue
        /// * User-specified throttling: rateLimits, retryConfig, and the queue's state.
        /// * System throttling due to 429 (Too Many Requests) or 503 (Service
        /// Unavailable) responses from the worker, high error rates, or to
        /// smooth sudden large traffic spikes.
        /// Structure is documented below.
        pub rate_limits: pulumi_wasm_rust::Output<
            super::super::types::cloudtasks::QueueRateLimits,
        >,
        /// Settings that determine the retry behavior.
        /// Structure is documented below.
        pub retry_config: pulumi_wasm_rust::Output<
            super::super::types::cloudtasks::QueueRetryConfig,
        >,
        /// Configuration options for writing logs to Stackdriver Logging.
        /// Structure is documented below.
        pub stackdriver_logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudtasks::QueueStackdriverLoggingConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueueArgs) -> QueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_engine_routing_override_binding = args
            .app_engine_routing_override
            .get_inner();
        let http_target_binding = args.http_target.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let rate_limits_binding = args.rate_limits.get_inner();
        let retry_config_binding = args.retry_config.get_inner();
        let stackdriver_logging_config_binding = args
            .stackdriver_logging_config
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudtasks/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appEngineRoutingOverride".into(),
                    value: &app_engine_routing_override_binding,
                },
                register_interface::ObjectField {
                    name: "httpTarget".into(),
                    value: &http_target_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rateLimits".into(),
                    value: &rate_limits_binding,
                },
                register_interface::ObjectField {
                    name: "retryConfig".into(),
                    value: &retry_config_binding,
                },
                register_interface::ObjectField {
                    name: "stackdriverLoggingConfig".into(),
                    value: &stackdriver_logging_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appEngineRoutingOverride".into(),
                },
                register_interface::ResultField {
                    name: "httpTarget".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "rateLimits".into(),
                },
                register_interface::ResultField {
                    name: "retryConfig".into(),
                },
                register_interface::ResultField {
                    name: "stackdriverLoggingConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueueResult {
            app_engine_routing_override: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appEngineRoutingOverride").unwrap(),
            ),
            http_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpTarget").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            rate_limits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rateLimits").unwrap(),
            ),
            retry_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryConfig").unwrap(),
            ),
            stackdriver_logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackdriverLoggingConfig").unwrap(),
            ),
        }
    }
}
