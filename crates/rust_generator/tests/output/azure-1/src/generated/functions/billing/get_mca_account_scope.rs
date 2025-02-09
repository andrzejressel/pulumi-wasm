#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_mca_account_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMcaAccountScopeArgs {
        /// The Billing Account Name of the MCA account.
        #[builder(into)]
        pub billing_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Billing Profile Name in the above Billing Account.
        #[builder(into)]
        pub billing_profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Invoice Section Name in the above Billing Profile.
        #[builder(into)]
        pub invoice_section_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMcaAccountScopeResult {
        pub billing_account_name: pulumi_gestalt_rust::Output<String>,
        pub billing_profile_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub invoice_section_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMcaAccountScopeArgs,
    ) -> GetMcaAccountScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let billing_account_name_binding_1 = args
            .billing_account_name
            .get_output(context);
        let billing_account_name_binding = billing_account_name_binding_1.get_inner();
        let billing_profile_name_binding_1 = args
            .billing_profile_name
            .get_output(context);
        let billing_profile_name_binding = billing_profile_name_binding_1.get_inner();
        let invoice_section_name_binding_1 = args
            .invoice_section_name
            .get_output(context);
        let invoice_section_name_binding = invoice_section_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:billing/getMcaAccountScope:getMcaAccountScope".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMcaAccountScopeResult {
            billing_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccountName"),
            ),
            billing_profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingProfileName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            invoice_section_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invoiceSectionName"),
            ),
        }
    }
}
