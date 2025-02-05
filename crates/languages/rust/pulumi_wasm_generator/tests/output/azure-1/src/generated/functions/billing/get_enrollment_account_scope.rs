pub mod get_enrollment_account_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnrollmentAccountScopeArgs {
        /// The Billing Account Name of the Enterprise Account.
        #[builder(into)]
        pub billing_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Enrollment Account Name in the above Enterprise Account.
        #[builder(into)]
        pub enrollment_account_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnrollmentAccountScopeResult {
        pub billing_account_name: pulumi_wasm_rust::Output<String>,
        pub enrollment_account_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEnrollmentAccountScopeArgs,
    ) -> GetEnrollmentAccountScopeResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_name_binding = args
            .billing_account_name
            .get_output(context)
            .get_inner();
        let enrollment_account_name_binding = args
            .enrollment_account_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:billing/getEnrollmentAccountScope:getEnrollmentAccountScope"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccountName".into(),
                    value: &billing_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "enrollmentAccountName".into(),
                    value: &enrollment_account_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEnrollmentAccountScopeResult {
            billing_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("billingAccountName"),
            ),
            enrollment_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enrollmentAccountName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
