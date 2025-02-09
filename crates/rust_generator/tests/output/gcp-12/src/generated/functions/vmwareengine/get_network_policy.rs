#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPolicyArgs {
        /// Location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPolicyResult {
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub edge_services_cidr: pulumi_gestalt_rust::Output<String>,
        pub external_ips: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetNetworkPolicyExternalIp>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub internet_accesses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetNetworkPolicyInternetAccess>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub vmware_engine_network: pulumi_gestalt_rust::Output<String>,
        pub vmware_engine_network_canonical: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkPolicyArgs,
    ) -> GetNetworkPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getNetworkPolicy:getNetworkPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
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
        GetNetworkPolicyResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            edge_services_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeServicesCidr"),
            ),
            external_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalIps"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            internet_accesses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internetAccesses"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vmware_engine_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareEngineNetwork"),
            ),
            vmware_engine_network_canonical: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareEngineNetworkCanonical"),
            ),
        }
    }
}
