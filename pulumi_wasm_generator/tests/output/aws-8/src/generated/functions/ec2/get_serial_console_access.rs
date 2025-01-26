pub mod get_serial_console_access {
    #[allow(dead_code)]
    pub struct GetSerialConsoleAccessResult {
        /// Whether or not serial console access is enabled. Returns as `true` or `false`.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetSerialConsoleAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getSerialConsoleAccess:getSerialConsoleAccess".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSerialConsoleAccessResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
