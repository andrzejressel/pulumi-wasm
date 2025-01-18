pub mod get_adbs_national_character_sets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAdbsNationalCharacterSetsArgs {
        /// The Azure Region to query for the national character sets in.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAdbsNationalCharacterSetsResult {
        /// A `character_sets` block as defined below.
        pub character_sets: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::oracle::GetAdbsNationalCharacterSetsCharacterSet,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetAdbsNationalCharacterSetsArgs,
    ) -> GetAdbsNationalCharacterSetsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getAdbsNationalCharacterSets:getAdbsNationalCharacterSets"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "characterSets".into(),
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
        GetAdbsNationalCharacterSetsResult {
            character_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("characterSets").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
        }
    }
}
