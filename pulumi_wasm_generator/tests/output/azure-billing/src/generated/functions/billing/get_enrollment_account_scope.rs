pub mod get_enrollment_account_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnrollmentAccountScopeArgs {
        /// The Billing Account Name of the Enterprise Account.
        #[builder(into)]
        pub billing_account_name: pulumi_wasm_rust::Output<String>,
        /// The Enrollment Account Name in the above Enterprise Account.
        #[builder(into)]
        pub enrollment_account_name: pulumi_wasm_rust::Output<String>,
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
        args: GetEnrollmentAccountScopeArgs,
    ) -> GetEnrollmentAccountScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_name_binding = args.billing_account_name.get_inner();
        let enrollment_account_name_binding = args.enrollment_account_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:billing/getEnrollmentAccountScope:getEnrollmentAccountScope"
                .into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "billingAccountName".into(),
                },
                register_interface::ResultField {
                    name: "enrollmentAccountName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEnrollmentAccountScopeResult {
            billing_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccountName").unwrap(),
            ),
            enrollment_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enrollmentAccountName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
