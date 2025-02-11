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
        context: &pulumi_gestalt_rust::Context,
        args: GetMcaAccountScopeArgs,
    ) -> GetMcaAccountScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let billing_account_name_binding = args.billing_account_name.get_output(context);
        let billing_profile_name_binding = args.billing_profile_name.get_output(context);
        let invoice_section_name_binding = args.invoice_section_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:billing/getMcaAccountScope:getMcaAccountScope".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccountName".into(),
                    value: &billing_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingProfileName".into(),
                    value: &billing_profile_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invoiceSectionName".into(),
                    value: &invoice_section_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMcaAccountScopeResult {
            billing_account_name: o.get_field("billingAccountName"),
            billing_profile_name: o.get_field("billingProfileName"),
            id: o.get_field("id"),
            invoice_section_name: o.get_field("invoiceSectionName"),
        }
    }
}
