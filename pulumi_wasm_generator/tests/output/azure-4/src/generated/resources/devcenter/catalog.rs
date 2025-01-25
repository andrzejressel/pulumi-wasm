pub mod catalog {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogArgs {
        #[builder(into, default)]
        pub catalog_adogit: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::devcenter::CatalogCatalogAdogit>,
        >,
        #[builder(into, default)]
        pub catalog_github: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::devcenter::CatalogCatalogGithub>,
        >,
        #[builder(into)]
        pub dev_center_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CatalogResult {
        pub catalog_adogit: pulumi_wasm_rust::Output<
            Option<super::super::types::devcenter::CatalogCatalogAdogit>,
        >,
        pub catalog_github: pulumi_wasm_rust::Output<
            Option<super::super::types::devcenter::CatalogCatalogGithub>,
        >,
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CatalogArgs,
    ) -> CatalogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_adogit_binding = args.catalog_adogit.get_output(context).get_inner();
        let catalog_github_binding = args.catalog_github.get_output(context).get_inner();
        let dev_center_id_binding = args.dev_center_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/catalog:Catalog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogAdogit".into(),
                    value: &catalog_adogit_binding,
                },
                register_interface::ObjectField {
                    name: "catalogGithub".into(),
                    value: &catalog_github_binding,
                },
                register_interface::ObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogAdogit".into(),
                },
                register_interface::ResultField {
                    name: "catalogGithub".into(),
                },
                register_interface::ResultField {
                    name: "devCenterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CatalogResult {
            catalog_adogit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogAdogit").unwrap(),
            ),
            catalog_github: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogGithub").unwrap(),
            ),
            dev_center_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devCenterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
