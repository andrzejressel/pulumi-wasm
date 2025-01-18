pub mod get_organizational_unit_descendant_organizational_units {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitDescendantOrganizationalUnitsArgs {
        /// Parent ID of the organizational unit.
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitDescendantOrganizationalUnitsResult {
        /// List of child organizational units, which have the following attributes:
        pub childrens: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationalUnitDescendantOrganizationalUnitsChildren,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parent_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetOrganizationalUnitDescendantOrganizationalUnitsArgs,
    ) -> GetOrganizationalUnitDescendantOrganizationalUnitsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getOrganizationalUnitDescendantOrganizationalUnits:getOrganizationalUnitDescendantOrganizationalUnits"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "childrens".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parentId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrganizationalUnitDescendantOrganizationalUnitsResult {
            childrens: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("childrens").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
        }
    }
}
