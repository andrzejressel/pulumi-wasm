pub mod get_lifecycle_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentArgs {
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The above arguments serialized as a standard JSON policy document.
        pub json: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLifecyclePolicyDocumentArgs,
    ) -> GetLifecyclePolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getLifecyclePolicyDocument:getLifecyclePolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLifecyclePolicyDocumentResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            json: pulumi_wasm_rust::__private::into_domain(o.extract_field("json")),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
        }
    }
}
