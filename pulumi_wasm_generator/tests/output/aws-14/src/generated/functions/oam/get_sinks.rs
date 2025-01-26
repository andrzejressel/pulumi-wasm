pub mod get_sinks {
    #[allow(dead_code)]
    pub struct GetSinksResult {
        /// Set of ARN of the Sinks.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetSinksResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:oam/getSinks:getSinks".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSinksResult {
            arns: pulumi_wasm_rust::__private::into_domain(o.extract_field("arns")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
