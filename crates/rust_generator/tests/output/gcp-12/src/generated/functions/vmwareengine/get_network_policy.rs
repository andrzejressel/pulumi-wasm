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
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkPolicyArgs,
    ) -> GetNetworkPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getNetworkPolicy:getNetworkPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkPolicyResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            edge_services_cidr: o.get_field("edgeServicesCidr"),
            external_ips: o.get_field("externalIps"),
            id: o.get_field("id"),
            internet_accesses: o.get_field("internetAccesses"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            vmware_engine_network: o.get_field("vmwareEngineNetwork"),
            vmware_engine_network_canonical: o.get_field("vmwareEngineNetworkCanonical"),
        }
    }
}
