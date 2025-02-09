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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceBridgeApplianceArgs,
    ) -> ResourceBridgeApplianceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let distro_binding_1 = args.distro.get_output(context);
        let distro_binding = distro_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let infrastructure_provider_binding_1 = args
            .infrastructure_provider
            .get_output(context);
        let infrastructure_provider_binding = infrastructure_provider_binding_1
            .get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let public_key_base64_binding_1 = args.public_key_base64.get_output(context);
        let public_key_base64_binding = public_key_base64_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arc/resourceBridgeAppliance:ResourceBridgeAppliance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "distro".into(),
                    value: &distro_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureProvider".into(),
                    value: &infrastructure_provider_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicKeyBase64".into(),
                    value: &public_key_base64_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceBridgeApplianceResult {
            distro: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distro"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            infrastructure_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureProvider"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_key_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKeyBase64"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
