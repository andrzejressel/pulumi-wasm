pub mod get_resource_bridge_appliance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceBridgeApplianceArgs {
        /// The name of this Arc Resource Bridge Appliance.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Arc Resource Bridge Appliance exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceBridgeApplianceResult {
        /// Fabric/Infrastructure for this Arc Resource Bridge Appliance.
        pub distro: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arc::GetResourceBridgeApplianceIdentity>,
        >,
        /// The infrastructure provider about the connected Arc Resource Bridge Appliance.
        pub infrastructure_provider: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Arc Resource Bridge Appliance exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// RSA public key in PKCS1 format encoded in base64.
        pub public_key_base64: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Arc Resource Bridge Appliance.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResourceBridgeApplianceArgs,
    ) -> GetResourceBridgeApplianceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:arc/getResourceBridgeAppliance:getResourceBridgeAppliance"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourceBridgeApplianceResult {
            distro: pulumi_wasm_rust::__private::into_domain(o.extract_field("distro")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            infrastructure_provider: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("infrastructureProvider"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            public_key_base64: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicKeyBase64"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
