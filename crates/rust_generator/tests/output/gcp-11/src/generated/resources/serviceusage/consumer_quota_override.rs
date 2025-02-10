/// A consumer override is applied to the consumer on its own authority to limit its own quota usage.
/// Consumer overrides cannot be used to grant more quota than would be allowed by admin overrides,
/// producer overrides, or the default limit of the service.
///
/// To get more information about ConsumerQuotaOverride, see:
/// * How-to Guides
///     * [Managing Service Quota](https://cloud.google.com/service-usage/docs/manage-quota )
///     * [REST API documentation](https://cloud.google.com/service-usage/docs/reference/rest/v1beta1/services.consumerQuotaMetrics.limits.consumerOverrides)
///
/// ## Example Usage
///
/// ### Consumer Quota Override
///
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: tf-test-project
///       projectId: quota
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   override:
///     type: gcp:serviceusage:ConsumerQuotaOverride
///     properties:
///       project: ${myProject.projectId}
///       service: servicemanagement.googleapis.com
///       metric:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: servicemanagement.googleapis.com/default_requests
///           return: result
///       limit:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: /min/project
///           return: result
///       overrideValue: '95'
///       force: true
/// ```
/// ### Region Consumer Quota Override
///
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: tf-test-project
///       projectId: quota
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   override:
///     type: gcp:serviceusage:ConsumerQuotaOverride
///     properties:
///       dimensions:
///         region: us-central1
///       project: ${myProject.projectId}
///       service: compute.googleapis.com
///       metric:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: compute.googleapis.com/n2_cpus
///           return: result
///       limit:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: /project/region
///           return: result
///       overrideValue: '8'
///       force: true
/// ```
/// ### Consumer Quota Override Custom Dimension
///
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: tf-test-project
///       projectId: quota
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   override:
///     type: gcp:serviceusage:ConsumerQuotaOverride
///     properties:
///       project: ${myProject.projectId}
///       service: libraryagent.googleapis.com
///       metric:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: libraryagent.googleapis.com/borrows
///           return: result
///       limit:
///         fn::invoke:
///           function: std:urlencode
///           arguments:
///             input: /author/project
///           return: result
///       overrideValue: '1'
///       force: true
///       dimensions:
///         author: larry
/// ```
///
/// ## Import
///
/// ConsumerQuotaOverride can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/services/{{service}}/consumerQuotaMetrics/{{metric}}/limits/{{limit}}/consumerOverrides/{{name}}`
///
/// * `services/{{service}}/consumerQuotaMetrics/{{metric}}/limits/{{limit}}/consumerOverrides/{{name}}`
///
/// * `{{service}}/{{metric}}/{{limit}}/{{name}}`
///
/// When using the `pulumi import` command, ConsumerQuotaOverride can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:serviceusage/consumerQuotaOverride:ConsumerQuotaOverride default projects/{{project}}/services/{{service}}/consumerQuotaMetrics/{{metric}}/limits/{{limit}}/consumerOverrides/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:serviceusage/consumerQuotaOverride:ConsumerQuotaOverride default services/{{service}}/consumerQuotaMetrics/{{metric}}/limits/{{limit}}/consumerOverrides/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:serviceusage/consumerQuotaOverride:ConsumerQuotaOverride default {{service}}/{{metric}}/{{limit}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod consumer_quota_override {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumerQuotaOverrideArgs {
        /// If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit.
        #[builder(into, default)]
        pub dimensions: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If the new quota would decrease the existing quota by more than 10%, the request is rejected.
        /// If `force` is `true`, that safety check is ignored.
        #[builder(into, default)]
        pub force: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The limit on the metric, e.g. `/project/region`.
        /// > Make sure that `limit` is in a format that doesn't start with `1/` or contain curly braces.
        /// E.g. use `/project/user` instead of `1/{project}/{user}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub limit: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metric that should be limited, e.g. `compute.googleapis.com/cpus`.
        #[builder(into)]
        pub metric: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota).
        #[builder(into)]
        pub override_value: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service that the metrics belong to, e.g. `compute.googleapis.com`.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConsumerQuotaOverrideResult {
        /// If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit.
        pub dimensions: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If the new quota would decrease the existing quota by more than 10%, the request is rejected.
        /// If `force` is `true`, that safety check is ignored.
        pub force: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The limit on the metric, e.g. `/project/region`.
        /// > Make sure that `limit` is in a format that doesn't start with `1/` or contain curly braces.
        /// E.g. use `/project/user` instead of `1/{project}/{user}`.
        ///
        ///
        /// - - -
        pub limit: pulumi_gestalt_rust::Output<String>,
        /// The metric that should be limited, e.g. `compute.googleapis.com/cpus`.
        pub metric: pulumi_gestalt_rust::Output<String>,
        /// The server-generated name of the quota override.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota).
        pub override_value: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The service that the metrics belong to, e.g. `compute.googleapis.com`.
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConsumerQuotaOverrideArgs,
    ) -> ConsumerQuotaOverrideResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dimensions_binding = args.dimensions.get_output(context);
        let force_binding = args.force.get_output(context);
        let limit_binding = args.limit.get_output(context);
        let metric_binding = args.metric.get_output(context);
        let override_value_binding = args.override_value.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:serviceusage/consumerQuotaOverride:ConsumerQuotaOverride".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dimensions".into(),
                    value: dimensions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "force".into(),
                    value: force_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "limit".into(),
                    value: limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metric".into(),
                    value: metric_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideValue".into(),
                    value: override_value_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConsumerQuotaOverrideResult {
            dimensions: o.get_field("dimensions"),
            force: o.get_field("force"),
            limit: o.get_field("limit"),
            metric: o.get_field("metric"),
            name: o.get_field("name"),
            override_value: o.get_field("overrideValue"),
            project: o.get_field("project"),
            service: o.get_field("service"),
        }
    }
}
