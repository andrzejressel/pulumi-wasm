#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_delegated_services {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDelegatedServicesArgs {
        /// Account ID number of a delegated administrator account in the organization.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDelegatedServicesResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Services for which the account is a delegated administrator, which have the following attributes:
        pub delegated_services: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::organizations::GetDelegatedServicesDelegatedService,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDelegatedServicesArgs,
    ) -> GetDelegatedServicesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getDelegatedServices:getDelegatedServices".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDelegatedServicesResult {
            account_id: o.get_field("accountId"),
            delegated_services: o.get_field("delegatedServices"),
            id: o.get_field("id"),
        }
    }
}
