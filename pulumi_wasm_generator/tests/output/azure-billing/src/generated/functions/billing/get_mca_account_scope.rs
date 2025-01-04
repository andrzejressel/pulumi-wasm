pub mod get_mca_account_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMcaAccountScopeArgs {
        /// The Billing Account Name of the MCA account.
        #[builder(into)]
        pub billing_account_name: pulumi_wasm_rust::Output<String>,
        /// The Billing Profile Name in the above Billing Account.
        #[builder(into)]
        pub billing_profile_name: pulumi_wasm_rust::Output<String>,
        /// The Invoice Section Name in the above Billing Profile.
        #[builder(into)]
        pub invoice_section_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetMcaAccountScopeResult {
        pub billing_account_name: pulumi_wasm_rust::Output<String>,
        pub billing_profile_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub invoice_section_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetMcaAccountScopeArgs) -> GetMcaAccountScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_name_binding = args.billing_account_name.get_inner();
        let billing_profile_name_binding = args.billing_profile_name.get_inner();
        let invoice_section_name_binding = args.invoice_section_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:billing/getMcaAccountScope:getMcaAccountScope".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccountName".into(),
                    value: &billing_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "billingProfileName".into(),
                    value: &billing_profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "invoiceSectionName".into(),
                    value: &invoice_section_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "billingAccountName".into(),
                },
                register_interface::ResultField {
                    name: "billingProfileName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "invoiceSectionName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMcaAccountScopeResult {
            billing_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccountName").unwrap(),
            ),
            billing_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingProfileName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            invoice_section_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invoiceSectionName").unwrap(),
            ),
        }
    }
}
