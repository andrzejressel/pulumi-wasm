#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource_bridge_appliance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceBridgeApplianceArgs {
        /// The name of this Arc Resource Bridge Appliance.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Arc Resource Bridge Appliance exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceBridgeApplianceResult {
        /// Fabric/Infrastructure for this Arc Resource Bridge Appliance.
        pub distro: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arc::GetResourceBridgeApplianceIdentity>,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance.
        pub infrastructure_provider: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// RSA public key in PKCS1 format encoded in base64.
        pub public_key_base64: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Arc Resource Bridge Appliance.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourceBridgeApplianceArgs,
    ) -> GetResourceBridgeApplianceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:arc/getResourceBridgeAppliance:getResourceBridgeAppliance"
                .into(),
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
        GetResourceBridgeApplianceResult {
            distro: o.get_field("distro"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            infrastructure_provider: o.get_field("infrastructureProvider"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_key_base64: o.get_field("publicKeyBase64"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
