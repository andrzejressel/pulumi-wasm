/// A [project being monitored](https://cloud.google.com/monitoring/settings/multiple-projects#create-multi) by a Metrics Scope.
///
///
/// To get more information about MonitoredProject, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v1/locations.global.metricsScopes.projects)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/settings/manage-api)
///
/// ## Example Usage
///
/// ### Monitoring Monitored Project Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = project::create(
///         "basic",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("m-id-display")
///             .org_id("123456789")
///             .project_id("m-id")
///             .build_struct(),
///     );
///     let primary = monitored_project::create(
///         "primary",
///         MonitoredProjectArgs::builder()
///             .metrics_scope("my-project-name")
///             .name("${basic.projectId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MonitoredProject can be imported using any of these accepted formats:
///
/// * `v1/locations/global/metricsScopes/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, MonitoredProject can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/monitoredProject:MonitoredProject default v1/locations/global/metricsScopes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/monitoredProject:MonitoredProject default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitored_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoredProjectArgs {
        /// Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub metrics_scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The resource name of the `MonitoredProject`. On input, the resource name includes the scoping project ID and monitored project ID. On output, it contains the equivalent project numbers. Example: `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitoredProjectResult {
        /// Output only. The time when this `MonitoredProject` was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}
        ///
        ///
        /// - - -
        pub metrics_scope: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The resource name of the `MonitoredProject`. On input, the resource name includes the scoping project ID and monitored project ID. On output, it contains the equivalent project numbers. Example: `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MonitoredProjectArgs,
    ) -> MonitoredProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let metrics_scope_binding = args.metrics_scope.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:monitoring/monitoredProject:MonitoredProject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricsScope".into(),
                    value: metrics_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitoredProjectResult {
            create_time: o.get_field("createTime"),
            metrics_scope: o.get_field("metricsScope"),
            name: o.get_field("name"),
        }
    }
}
