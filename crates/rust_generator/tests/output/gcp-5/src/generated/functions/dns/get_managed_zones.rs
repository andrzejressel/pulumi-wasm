#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_zones {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedZonesArgs {
        /// The ID of the project containing Google Cloud DNS zones. If this is not provided the default project will be used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetManagedZonesResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of managed zones.
        pub managed_zones: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dns::GetManagedZonesManagedZone>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedZonesArgs,
    ) -> GetManagedZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:dns/getManagedZones:getManagedZones".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetManagedZonesResult {
            id: o.get_field("id"),
            managed_zones: o.get_field("managedZones"),
            project: o.get_field("project"),
        }
    }
}
