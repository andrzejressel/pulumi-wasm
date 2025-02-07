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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDiscoveredServiceArgs,
    ) -> GetDiscoveredServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_uri_binding = args.service_uri.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:apphub/getDiscoveredService:getDiscoveredService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceUri".into(),
                    value: &service_uri_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDiscoveredServiceResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceProperties"),
            ),
            service_references: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceReferences"),
            ),
            service_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceUri"),
            ),
        }
    }
}
