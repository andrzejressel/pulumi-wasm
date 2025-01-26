pub mod get_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocationArgs {
        /// Code for the location to retrieve.
        #[builder(into)]
        pub location_code: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocationResult {
        /// The available MAC Security (MACsec) port speeds for the location.
        pub available_macsec_port_speeds: pulumi_wasm_rust::Output<Vec<String>>,
        /// The available port speeds for the location.
        pub available_port_speeds: pulumi_wasm_rust::Output<Vec<String>>,
        /// Names of the service providers for the location.
        pub available_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location_code: pulumi_wasm_rust::Output<String>,
        /// Name of the location. This includes the name of the colocation partner and the physical site of the building.
        pub location_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLocationArgs,
    ) -> GetLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_code_binding = args.location_code.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:directconnect/getLocation:getLocation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "locationCode".into(),
                    value: &location_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availableMacsecPortSpeeds".into(),
                },
                register_interface::ResultField {
                    name: "availablePortSpeeds".into(),
                },
                register_interface::ResultField {
                    name: "availableProviders".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "locationCode".into(),
                },
                register_interface::ResultField {
                    name: "locationName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLocationResult {
            available_macsec_port_speeds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableMacsecPortSpeeds").unwrap(),
            ),
            available_port_speeds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availablePortSpeeds").unwrap(),
            ),
            available_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableProviders").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationCode").unwrap(),
            ),
            location_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationName").unwrap(),
            ),
        }
    }
}
