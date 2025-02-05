pub mod get_registration_code {
    #[allow(dead_code)]
    pub struct GetRegistrationCodeResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The CA certificate registration code.
        pub registration_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetRegistrationCodeResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iot/getRegistrationCode:getRegistrationCode".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegistrationCodeResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            registration_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registrationCode"),
            ),
        }
    }
}
