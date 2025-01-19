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
pub mod resource_bridge_appliance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceBridgeApplianceArgs {
        /// Specifies a supported Fabric/Infrastructure for this Arc Resource Bridge Appliance. The possible value is `AKSEdge`.
        #[builder(into)]
        pub distro: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::arc::ResourceBridgeApplianceIdentity,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance. Possible values are `HCI`,`SCVMM` and `VMWare`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub infrastructure_provider: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name which should be used for this Arc Resource Bridge Appliance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The `public_key_base64` is an RSA public key in PKCS1 format encoded in base64. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub public_key_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the resource group where the Arc Resource Bridge Appliance exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Resource Bridge Appliance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceBridgeApplianceResult {
        /// Specifies a supported Fabric/Infrastructure for this Arc Resource Bridge Appliance. The possible value is `AKSEdge`.
        pub distro: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::arc::ResourceBridgeApplianceIdentity,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance. Possible values are `HCI`,`SCVMM` and `VMWare`. Changing this forces a new resource to be created.
        pub infrastructure_provider: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Name which should be used for this Arc Resource Bridge Appliance. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The `public_key_base64` is an RSA public key in PKCS1 format encoded in base64. Changing this forces a new resource to be created.
        pub public_key_base64: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the resource group where the Arc Resource Bridge Appliance exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Arc Resource Bridge Appliance.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourceBridgeApplianceArgs,
    ) -> ResourceBridgeApplianceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let distro_binding = args.distro.get_inner();
        let identity_binding = args.identity.get_inner();
        let infrastructure_provider_binding = args.infrastructure_provider.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_key_base64_binding = args.public_key_base64.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "distro".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureProvider".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicKeyBase64".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceBridgeApplianceResult {
            distro: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distro").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            infrastructure_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureProvider").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_key_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeyBase64").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
