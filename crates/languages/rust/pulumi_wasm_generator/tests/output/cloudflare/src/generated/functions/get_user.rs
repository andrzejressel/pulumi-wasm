pub mod get_user {
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// The user's email address.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The user's unique identifier.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The user's username.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getUser:getUser".into(),
            version: super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
