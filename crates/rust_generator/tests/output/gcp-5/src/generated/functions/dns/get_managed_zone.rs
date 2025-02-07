pub mod get_managed_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedZoneArgs {
        /// A unique name for the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project for the Google Cloud DNS zone.  If this is not provided the default project will be used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetManagedZoneResult {
        /// A textual description field.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified DNS name of this zone, e.g. `example.io.`.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_zone_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The list of nameservers that will be authoritative for this
        /// domain. Use NS records to redirect from your DNS provider to these names,
        /// thus making Google Cloud DNS authoritative for this zone.
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone's visibility: public zones are exposed to the Internet,
        /// while private zones are visible only to Virtual Private Cloud resources.
        pub visibility: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedZoneArgs,
    ) -> GetManagedZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getManagedZone:getManagedZone".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedZoneResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedZoneId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameServers"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            visibility: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("visibility"),
            ),
        }
    }
}
