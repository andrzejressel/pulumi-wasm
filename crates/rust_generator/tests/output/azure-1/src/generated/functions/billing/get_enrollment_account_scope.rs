#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_enrollment_account_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnrollmentAccountScopeArgs {
        /// The Billing Account Name of the Enterprise Account.
        #[builder(into)]
        pub billing_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Enrollment Account Name in the above Enterprise Account.
        #[builder(into)]
        pub enrollment_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnrollmentAccountScopeResult {
        pub billing_account_name: pulumi_gestalt_rust::Output<String>,
        pub enrollment_account_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEnrollmentAccountScopeArgs,
    ) -> GetEnrollmentAccountScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let billing_account_name_binding = args.billing_account_name.get_output(context);
        let enrollment_account_name_binding = args
            .enrollment_account_name
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:billing/getEnrollmentAccountScope:getEnrollmentAccountScope"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccountName".into(),
                    value: &billing_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enrollmentAccountName".into(),
                    value: &enrollment_account_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEnrollmentAccountScopeResult {
            billing_account_name: o.get_field("billingAccountName"),
            enrollment_account_name: o.get_field("enrollmentAccountName"),
            id: o.get_field("id"),
        }
    }
}
