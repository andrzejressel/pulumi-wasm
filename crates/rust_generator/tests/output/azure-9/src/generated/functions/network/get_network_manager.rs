#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkManagerArgs {
        /// The name of the Network Manager.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Network Manager exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkManagerResult {
        /// One or more `cross_tenant_scopes` blocks as defined below.
        pub cross_tenant_scopes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetNetworkManagerCrossTenantScope>,
        >,
        /// A description of the Network Manager.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Network Manager exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of configuration deployment type configured on the Network Manager.
        pub scope_accesses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `scope` block as defined below.
        pub scopes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetNetworkManagerScope>,
        >,
        /// A mapping of tags assigned to the Network Manager.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkManagerArgs,
    ) -> GetNetworkManagerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getNetworkManager:getNetworkManager".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkManagerResult {
            cross_tenant_scopes: o.get_field("crossTenantScopes"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope_accesses: o.get_field("scopeAccesses"),
            scopes: o.get_field("scopes"),
            tags: o.get_field("tags"),
        }
    }
}
