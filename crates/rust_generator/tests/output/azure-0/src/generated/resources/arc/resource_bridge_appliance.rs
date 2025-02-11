/// Manages an Arc Resource Bridge Appliance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleResourceBridgeAppliance:
///     type: azure:arc:ResourceBridgeAppliance
///     name: example
///     properties:
///       name: example-appliance
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       distro: AKSEdge
///       infrastructureProvider: VMWare
///       identity:
///         type: SystemAssigned
///       tags:
///         hello: world
/// ```
///
/// ## Import
///
/// Arc Resource Bridge Appliance can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arc/resourceBridgeAppliance:ResourceBridgeAppliance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ResourceConnector/appliances/appliancesExample
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_bridge_appliance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceBridgeApplianceArgs {
        /// Specifies a supported Fabric/Infrastructure for this Arc Resource Bridge Appliance. The possible value is `AKSEdge`.
        #[builder(into)]
        pub distro: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::arc::ResourceBridgeApplianceIdentity,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance. Possible values are `HCI`,`SCVMM` and `VMWare`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub infrastructure_provider: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name which should be used for this Arc Resource Bridge Appliance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `public_key_base64` is an RSA public key in PKCS1 format encoded in base64. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub public_key_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group where the Arc Resource Bridge Appliance exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Arc Resource Bridge Appliance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceBridgeApplianceResult {
        /// Specifies a supported Fabric/Infrastructure for this Arc Resource Bridge Appliance. The possible value is `AKSEdge`.
        pub distro: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::arc::ResourceBridgeApplianceIdentity,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance. Possible values are `HCI`,`SCVMM` and `VMWare`. Changing this forces a new resource to be created.
        pub infrastructure_provider: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this Arc Resource Bridge Appliance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The `public_key_base64` is an RSA public key in PKCS1 format encoded in base64. Changing this forces a new resource to be created.
        pub public_key_base64: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the resource group where the Arc Resource Bridge Appliance exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Resource Bridge Appliance.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceBridgeApplianceArgs,
    ) -> ResourceBridgeApplianceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let distro_binding = args.distro.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let infrastructure_provider_binding = args
            .infrastructure_provider
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_key_base64_binding = args.public_key_base64.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:arc/resourceBridgeAppliance:ResourceBridgeAppliance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distro".into(),
                    value: &distro_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "infrastructureProvider".into(),
                    value: &infrastructure_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKeyBase64".into(),
                    value: &public_key_base64_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceBridgeApplianceResult {
            distro: o.get_field("distro"),
            identity: o.get_field("identity"),
            infrastructure_provider: o.get_field("infrastructureProvider"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_key_base64: o.get_field("publicKeyBase64"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
