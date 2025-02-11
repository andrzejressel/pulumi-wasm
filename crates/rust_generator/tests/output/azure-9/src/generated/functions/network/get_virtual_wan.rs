#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_wan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualWanArgs {
        /// The name of this Virtual Wan.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Virtual Wan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualWanResult {
        /// Is branch to branch traffic is allowed?
        pub allow_branch_to_branch_traffic: pulumi_gestalt_rust::Output<bool>,
        /// Is VPN Encryption disabled?
        pub disable_vpn_encryption: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Virtual Wan exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Office365 Local Breakout Category.
        pub office365_local_breakout_category: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Type of Virtual Wan (Basic or Standard).
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Virtual Wan.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Virtual Hubs IDs attached to this Virtual WAN.
        pub virtual_hub_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of VPN Site IDs attached to this Virtual WAN.
        pub vpn_site_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualWanArgs,
    ) -> GetVirtualWanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualWan:getVirtualWan".into(),
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
        GetVirtualWanResult {
            allow_branch_to_branch_traffic: o.get_field("allowBranchToBranchTraffic"),
            disable_vpn_encryption: o.get_field("disableVpnEncryption"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            office365_local_breakout_category: o
                .get_field("office365LocalBreakoutCategory"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            virtual_hub_ids: o.get_field("virtualHubIds"),
            vpn_site_ids: o.get_field("vpnSiteIds"),
        }
    }
}
