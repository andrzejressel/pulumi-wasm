pub mod get_templates {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTemplatesArgs {
        /// AWS Region to which the quota increases apply.
        #[builder(into)]
        pub region: pulumi_wasm_rust::Output<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        #[builder(into, default)]
        pub templates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTemplatesResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// AWS Region to which the template applies.
        pub region: pulumi_wasm_rust::Output<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        pub templates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTemplatesArgs) -> GetTemplatesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let region_binding = args.region.get_inner();
        let templates_binding = args.templates.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicequotas/getTemplates:getTemplates".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "templates".into(),
                    value: &templates_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "templates".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTemplatesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templates").unwrap(),
            ),
        }
    }
}
