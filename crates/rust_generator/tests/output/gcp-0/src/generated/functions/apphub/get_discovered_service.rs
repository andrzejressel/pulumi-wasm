#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_discovered_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiscoveredServiceArgs {
        /// The location of the discovered service.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The host project of the discovered service.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The uri of the service.
        #[builder(into)]
        pub service_uri: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiscoveredServiceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location that the underlying resource resides in.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource name of a Service. Format: "projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}".
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// Properties of an underlying compute resource that can comprise a Service. Structure is documented below
        pub service_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apphub::GetDiscoveredServiceServiceProperty>,
        >,
        /// Reference to an underlying networking resource that can comprise a Service. Structure is documented below
        pub service_references: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apphub::GetDiscoveredServiceServiceReference>,
        >,
        pub service_uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDiscoveredServiceArgs,
    ) -> GetDiscoveredServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_uri_binding = args.service_uri.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:apphub/getDiscoveredService:getDiscoveredService".into(),
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
                    name: "serviceUri".into(),
                    value: service_uri_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDiscoveredServiceResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_properties: o.get_field("serviceProperties"),
            service_references: o.get_field("serviceReferences"),
            service_uri: o.get_field("serviceUri"),
        }
    }
}
