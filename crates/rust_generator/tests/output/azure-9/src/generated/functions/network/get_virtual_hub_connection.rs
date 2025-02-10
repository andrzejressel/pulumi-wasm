#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_hub_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubConnectionArgs {
        /// The name of the Connection which should be retrieved.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub Connection exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Virtual Hub where this Connection exists.
        #[builder(into)]
        pub virtual_hub_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubConnectionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether Internet Security is enabled to secure internet traffic on this connection
        pub internet_security_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name which is used for this Static Route.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Network which the Virtual Hub is connected
        pub remote_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `routing` block as defined below.
        pub routings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVirtualHubConnectionRouting>,
        >,
        /// The ID of the Virtual Hub within which this connection is created
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
        pub virtual_hub_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualHubConnectionArgs,
    ) -> GetVirtualHubConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let virtual_hub_name_binding = args.virtual_hub_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualHubConnection:getVirtualHubConnection"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubName".into(),
                    value: virtual_hub_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualHubConnectionResult {
            id: o.get_field("id"),
            internet_security_enabled: o.get_field("internetSecurityEnabled"),
            name: o.get_field("name"),
            remote_virtual_network_id: o.get_field("remoteVirtualNetworkId"),
            resource_group_name: o.get_field("resourceGroupName"),
            routings: o.get_field("routings"),
            virtual_hub_id: o.get_field("virtualHubId"),
            virtual_hub_name: o.get_field("virtualHubName"),
        }
    }
}
