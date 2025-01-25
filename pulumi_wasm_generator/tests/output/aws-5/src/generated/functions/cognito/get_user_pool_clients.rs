pub mod get_user_pool_clients {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolClientsArgs {
        /// Cognito user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserPoolClientsResult {
        /// List of Cognito user pool client IDs.
        pub client_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of Cognito user pool client names.
        pub client_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserPoolClientsArgs,
    ) -> GetUserPoolClientsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPoolClients:getUserPoolClients".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientIds".into(),
                },
                register_interface::ResultField {
                    name: "clientNames".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserPoolClientsResult {
            client_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientIds").unwrap(),
            ),
            client_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientNames").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}
