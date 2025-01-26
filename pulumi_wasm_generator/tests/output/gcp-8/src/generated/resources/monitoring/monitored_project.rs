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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod monitored_project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoredProjectArgs {
        /// Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub metrics_scope: pulumi_wasm_rust::InputOrOutput<String>,
        /// Immutable. The resource name of the `MonitoredProject`. On input, the resource name includes the scoping project ID and monitored project ID. On output, it contains the equivalent project numbers. Example: `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitoredProjectResult {
        /// Output only. The time when this `MonitoredProject` was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}
        ///
        ///
        /// - - -
        pub metrics_scope: pulumi_wasm_rust::Output<String>,
        /// Immutable. The resource name of the `MonitoredProject`. On input, the resource name includes the scoping project ID and monitored project ID. On output, it contains the equivalent project numbers. Example: `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MonitoredProjectArgs,
    ) -> MonitoredProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metrics_scope_binding = args.metrics_scope.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/monitoredProject:MonitoredProject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metricsScope".into(),
                    value: &metrics_scope_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitoredProjectResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            metrics_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metricsScope"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
