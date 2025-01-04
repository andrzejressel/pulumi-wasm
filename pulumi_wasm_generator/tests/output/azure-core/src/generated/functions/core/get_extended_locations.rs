pub mod get_extended_locations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExtendedLocationsArgs {
        /// The Azure location to retrieve the Extended Locations for.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetExtendedLocationsResult {
        /// The available extended locations for the Azure Location.
        pub extended_locations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetExtendedLocationsArgs) -> GetExtendedLocationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:core/getExtendedLocations:getExtendedLocations".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "extendedLocations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetExtendedLocationsResult {
            extended_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedLocations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
        }
    }
}
