pub mod get_variables {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVariablesArgs {
        /// The resource ID of the automation account.
        #[builder(into)]
        pub automation_account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVariablesResult {
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// One or more `variable` blocks as defined below for each boolean variable.
        pub bools: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesBool>,
        >,
        /// One or more `variable` blocks as defined below for each datetime variable.
        pub datetimes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesDatetime>,
        >,
        /// Specifies if the Automation Variable is encrypted.
        pub encrypteds: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesEncrypted>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `variable` blocks as defined below for each int variable.
        pub ints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesInt>,
        >,
        /// One or more `variable` blocks as defined below for each null variable.
        pub nulls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesNull>,
        >,
        pub objects: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesObject>,
        >,
        /// One or more `variable` blocks as defined below for each string variable.
        pub strings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesString>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVariablesArgs,
    ) -> GetVariablesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_id_binding = args
            .automation_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:automation/getVariables:getVariables".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountId".into(),
                    value: &automation_account_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVariablesResult {
            automation_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automationAccountId"),
            ),
            bools: pulumi_wasm_rust::__private::into_domain(o.extract_field("bools")),
            datetimes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("datetimes"),
            ),
            encrypteds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encrypteds"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ints: pulumi_wasm_rust::__private::into_domain(o.extract_field("ints")),
            nulls: pulumi_wasm_rust::__private::into_domain(o.extract_field("nulls")),
            objects: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objects"),
            ),
            strings: pulumi_wasm_rust::__private::into_domain(o.extract_field("strings")),
        }
    }
}
