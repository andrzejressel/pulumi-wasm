#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workload_identity_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkloadIdentityPoolArgs {
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the pool which is the
        /// final component of the resource name.
        ///
        /// - - -
        #[builder(into)]
        pub workload_identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkloadIdentityPoolResult {
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub workload_identity_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkloadIdentityPoolArgs,
    ) -> GetWorkloadIdentityPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let workload_identity_pool_id_binding = args
            .workload_identity_pool_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:iam/getWorkloadIdentityPool:getWorkloadIdentityPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityPoolId".into(),
                    value: workload_identity_pool_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkloadIdentityPoolResult {
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            workload_identity_pool_id: o.get_field("workloadIdentityPoolId"),
        }
    }
}
