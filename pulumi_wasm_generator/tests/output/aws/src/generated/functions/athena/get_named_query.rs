pub mod get_named_query {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamedQueryArgs {
        /// The plain language name for the query. Maximum length of 128.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The workgroup to which the query belongs. Defaults to `primary`.
        #[builder(into, default)]
        pub workgroup: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNamedQueryResult {
        /// Database to which the query belongs.
        pub database: pulumi_wasm_rust::Output<String>,
        /// Brief explanation of the query.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub querystring: pulumi_wasm_rust::Output<String>,
        pub workgroup: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNamedQueryArgs) -> GetNamedQueryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let workgroup_binding = args.workgroup.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:athena/getNamedQuery:getNamedQuery".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workgroup".into(),
                    value: &workgroup_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "querystring".into(),
                },
                register_interface::ResultField {
                    name: "workgroup".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNamedQueryResult {
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            querystring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("querystring").unwrap(),
            ),
            workgroup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroup").unwrap(),
            ),
        }
    }
}
