pub mod get_authorizers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizersArgs {
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizersResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of Authorizer identifiers.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAuthorizersArgs) -> GetAuthorizersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rest_api_id_binding = args.rest_api_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getAuthorizers:getAuthorizers".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "restApiId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAuthorizersResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApiId").unwrap(),
            ),
        }
    }
}
