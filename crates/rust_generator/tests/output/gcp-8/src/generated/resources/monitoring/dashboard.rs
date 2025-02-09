/// A Google Stackdriver dashboard. Dashboards define the content and layout of pages in the Stackdriver web application.
///
/// To get more information about Dashboards, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v1/projects.dashboards)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/dashboards)
///
/// ## Example Usage
///
/// ### Monitoring Dashboard Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dashboard = dashboard::create(
///         "dashboard",
///         DashboardArgs::builder()
///             .dashboard_json(
///                 "{\n  \"displayName\": \"Demo Dashboard\",\n  \"gridLayout\": {\n    \"widgets\": [\n      {\n        \"blank\": {}\n      }\n    ]\n  }\n}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Monitoring Dashboard GridLayout
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dashboard = dashboard::create(
///         "dashboard",
///         DashboardArgs::builder()
///             .dashboard_json(
///                 "{\n  \"displayName\": \"Grid Layout Example\",\n  \"gridLayout\": {\n    \"columns\": \"2\",\n    \"widgets\": [\n      {\n        \"title\": \"Widget 1\",\n        \"xyChart\": {\n          \"dataSets\": [{\n            \"timeSeriesQuery\": {\n              \"timeSeriesFilter\": {\n                \"filter\": \"metric.type=\\\"agent.googleapis.com/nginx/connections/accepted_count\\\"\",\n                \"aggregation\": {\n                  \"perSeriesAligner\": \"ALIGN_RATE\"\n                }\n              },\n              \"unitOverride\": \"1\"\n            },\n            \"plotType\": \"LINE\"\n          }],\n          \"timeshiftDuration\": \"0s\",\n          \"yAxis\": {\n            \"label\": \"y1Axis\",\n            \"scale\": \"LINEAR\"\n          }\n        }\n      },\n      {\n        \"text\": {\n          \"content\": \"Widget 2\",\n          \"format\": \"MARKDOWN\"\n        }\n      },\n      {\n        \"title\": \"Widget 3\",\n        \"xyChart\": {\n          \"dataSets\": [{\n            \"timeSeriesQuery\": {\n              \"timeSeriesFilter\": {\n                \"filter\": \"metric.type=\\\"agent.googleapis.com/nginx/connections/accepted_count\\\"\",\n                \"aggregation\": {\n                  \"perSeriesAligner\": \"ALIGN_RATE\"\n                }\n              },\n              \"unitOverride\": \"1\"\n            },\n            \"plotType\": \"STACKED_BAR\"\n          }],\n          \"timeshiftDuration\": \"0s\",\n          \"yAxis\": {\n            \"label\": \"y1Axis\",\n            \"scale\": \"LINEAR\"\n          }\n        }\n      }\n    ]\n  }\n}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Dashboard can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/dashboards/{{dashboard_id}}`
///
/// * `{{dashboard_id}}`
///
/// When using the `pulumi import` command, Dashboard can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/dashboard:Dashboard default projects/{{project}}/dashboards/{{dashboard_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/dashboard:Dashboard default {{dashboard_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dashboard {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DashboardArgs {
        /// The JSON representation of a dashboard, following the format at
        /// https://cloud.google.com/monitoring/api/ref_v3/rest/v1/projects.dashboards.
        #[builder(into)]
        pub dashboard_json: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DashboardResult {
        /// The JSON representation of a dashboard, following the format at
        /// https://cloud.google.com/monitoring/api/ref_v3/rest/v1/projects.dashboards.
        pub dashboard_json: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DashboardArgs,
    ) -> DashboardResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dashboard_json_binding = args.dashboard_json.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:monitoring/dashboard:Dashboard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardJson".into(),
                    value: dashboard_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DashboardResult {
            dashboard_json: o.get_field("dashboardJson"),
            project: o.get_field("project"),
        }
    }
}
