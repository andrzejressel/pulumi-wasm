#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app_engine_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppEngineServiceArgs {
        /// The ID of the App Engine module underlying this
        /// service. Corresponds to the moduleId resource label in the [gae_app](https://cloud.google.com/monitoring/api/resources#tag_gae_app) monitored resource, or the service/module name.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into)]
        pub module_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppEngineServiceResult {
        /// Name used for UI elements listing this (Monitoring) Service.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub module_id: pulumi_gestalt_rust::Output<String>,
        /// The full REST resource name for this channel. The syntax is:
        /// `projects/[PROJECT_ID]/services/[SERVICE_ID]`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration for how to query telemetry on the Service. Structure is documented below.
        pub telemetries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetAppEngineServiceTelemetry>,
        >,
        pub user_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAppEngineServiceArgs,
    ) -> GetAppEngineServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let module_id_binding_1 = args.module_id.get_output(context);
        let module_id_binding = module_id_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getAppEngineService:getAppEngineService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "moduleId".into(),
                    value: &module_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAppEngineServiceResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            module_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("moduleId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            telemetries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("telemetries"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
