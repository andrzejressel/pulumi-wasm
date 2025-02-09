#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_mpa_account_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMpaAccountScopeArgs {
        /// The Billing Account Name of the MPA account.
        #[builder(into)]
        pub billing_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Customer Name in the above Billing Account.
        #[builder(into)]
        pub customer_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMpaAccountScopeResult {
        pub billing_account_name: pulumi_gestalt_rust::Output<String>,
        pub customer_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMpaAccountScopeArgs,
    ) -> GetMpaAccountScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let billing_account_name_binding_1 = args
            .billing_account_name
            .get_output(context);
        let billing_account_name_binding = billing_account_name_binding_1.get_inner();
        let customer_name_binding_1 = args.customer_name.get_output(context);
        let customer_name_binding = customer_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:billing/getMpaAccountScope:getMpaAccountScope".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccountName".into(),
                    value: &billing_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "customerName".into(),
                    value: &customer_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMpaAccountScopeResult {
            billing_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccountName"),
            ),
            customer_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
