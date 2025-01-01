pub mod get_appregistry_attribute_group_associations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppregistryAttributeGroupAssociationsArgs {
        /// ID of the application to which attribute groups are associated.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the application to which attribute groups are associated.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppregistryAttributeGroupAssociationsResult {
        /// Set of attribute group IDs this application is associated with.
        pub attribute_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetAppregistryAttributeGroupAssociationsArgs,
    ) -> GetAppregistryAttributeGroupAssociationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getAppregistryAttributeGroupAssociations:getAppregistryAttributeGroupAssociations"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributeGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAppregistryAttributeGroupAssociationsResult {
            attribute_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributeGroupIds").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
