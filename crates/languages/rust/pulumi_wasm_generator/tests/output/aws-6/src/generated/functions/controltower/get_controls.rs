pub mod get_controls {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetControlsArgs {
        /// The ARN of the organizational unit.
        #[builder(into)]
        pub target_identifier: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetControlsResult {
        /// List of all the ARNs for the controls applied to the `target_identifier`.
        pub enabled_controls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub target_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetControlsArgs,
    ) -> GetControlsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let target_identifier_binding = args
            .target_identifier
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:controltower/getControls:getControls".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "targetIdentifier".into(),
                    value: &target_identifier_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetControlsResult {
            enabled_controls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabledControls"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            target_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetIdentifier"),
            ),
        }
    }
}
