#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_variables {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVariablesArgs {
        /// The resource ID of the automation account.
        #[builder(into)]
        pub automation_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVariablesResult {
        pub automation_account_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `variable` blocks as defined below for each boolean variable.
        pub bools: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesBool>,
        >,
        /// One or more `variable` blocks as defined below for each datetime variable.
        pub datetimes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesDatetime>,
        >,
        /// Specifies if the Automation Variable is encrypted.
        pub encrypteds: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesEncrypted>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `variable` blocks as defined below for each int variable.
        pub ints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesInt>,
        >,
        /// One or more `variable` blocks as defined below for each null variable.
        pub nulls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesNull>,
        >,
        pub objects: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesObject>,
        >,
        /// One or more `variable` blocks as defined below for each string variable.
        pub strings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetVariablesString>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVariablesArgs,
    ) -> GetVariablesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_id_binding = args
            .automation_account_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:automation/getVariables:getVariables".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountId".into(),
                    value: automation_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVariablesResult {
            automation_account_id: o.get_field("automationAccountId"),
            bools: o.get_field("bools"),
            datetimes: o.get_field("datetimes"),
            encrypteds: o.get_field("encrypteds"),
            id: o.get_field("id"),
            ints: o.get_field("ints"),
            nulls: o.get_field("nulls"),
            objects: o.get_field("objects"),
            strings: o.get_field("strings"),
        }
    }
}
