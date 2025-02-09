#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_discovered_workload {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiscoveredWorkloadArgs {
        /// The location of the discovered workload.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The host project of the discovered workload.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The uri of the workload (instance group managed by the Instance Group Manager). Example: "//compute.googleapis.com/projects/1/regions/us-east1/instanceGroups/id1"
        #[builder(into)]
        pub workload_uri: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiscoveredWorkloadResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location that the underlying resource resides in.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource name of a Workload. Format: "projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}".
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// Properties of an underlying compute resource that can comprise a Workload. Structure is documented below
        pub workload_properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::apphub::GetDiscoveredWorkloadWorkloadProperty,
            >,
        >,
        /// Reference to an underlying networking resource that can comprise a Workload. Structure is documented below
        pub workload_references: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::apphub::GetDiscoveredWorkloadWorkloadReference,
            >,
        >,
        pub workload_uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDiscoveredWorkloadArgs,
    ) -> GetDiscoveredWorkloadResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let workload_uri_binding = args.workload_uri.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:apphub/getDiscoveredWorkload:getDiscoveredWorkload".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadUri".into(),
                    value: workload_uri_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDiscoveredWorkloadResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            workload_properties: o.get_field("workloadProperties"),
            workload_references: o.get_field("workloadReferences"),
            workload_uri: o.get_field("workloadUri"),
        }
    }
}
