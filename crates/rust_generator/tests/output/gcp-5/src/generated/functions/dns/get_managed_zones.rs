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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedZonesArgs,
    ) -> GetManagedZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getManagedZones:getManagedZones".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedZonesResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedZones"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
