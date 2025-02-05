pub mod get_policy_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyStoreArgs {
        /// The ID of the Policy Store.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyStoreResult {
        /// The ARN of the Policy Store.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date the Policy Store was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date the Policy Store was last updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Validation settings for the policy store.
        pub validation_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::verifiedpermissions::GetPolicyStoreValidationSetting,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicyStoreArgs,
    ) -> GetPolicyStoreResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:verifiedpermissions/getPolicyStore:getPolicyStore".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicyStoreResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            validation_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validationSettings"),
            ),
        }
    }
}
