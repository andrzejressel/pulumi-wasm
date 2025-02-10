#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_project_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectServiceArgs {
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Google Platform project service.
        ///
        /// - - -
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProjectServiceResult {
        pub check_if_service_has_usage_on_destroy: pulumi_gestalt_rust::Output<bool>,
        pub disable_dependent_services: pulumi_gestalt_rust::Output<bool>,
        pub disable_on_destroy: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProjectServiceArgs,
    ) -> GetProjectServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:projects/getProjectService:getProjectService".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetProjectServiceResult {
            check_if_service_has_usage_on_destroy: o
                .get_field("checkIfServiceHasUsageOnDestroy"),
            disable_dependent_services: o.get_field("disableDependentServices"),
            disable_on_destroy: o.get_field("disableOnDestroy"),
            id: o.get_field("id"),
            project: o.get_field("project"),
            service: o.get_field("service"),
        }
    }
}
