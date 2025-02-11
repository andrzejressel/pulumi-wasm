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
        context: &pulumi_gestalt_rust::Context,
        args: GetAppEngineServiceArgs,
    ) -> GetAppEngineServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let module_id_binding = args.module_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:monitoring/getAppEngineService:getAppEngineService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "moduleId".into(),
                    value: &module_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAppEngineServiceResult {
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            module_id: o.get_field("moduleId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_id: o.get_field("serviceId"),
            telemetries: o.get_field("telemetries"),
            user_labels: o.get_field("userLabels"),
        }
    }
}
