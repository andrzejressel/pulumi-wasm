#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_organization_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationServiceAccountArgs {
        /// The organization ID the service account was created for.
        #[builder(into)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationServiceAccountResult {
        /// The email address of the service account. This value is
        /// often used to refer to the service account in order to grant IAM permissions.
        pub account_email: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Access Approval service account resource name. Format is "organizations/{organization_id}/serviceAccount".
        pub name: pulumi_gestalt_rust::Output<String>,
        pub organization_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrganizationServiceAccountArgs,
    ) -> GetOrganizationServiceAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let organization_id_binding = args.organization_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:accessapproval/getOrganizationServiceAccount:getOrganizationServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationId".into(),
                    value: organization_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrganizationServiceAccountResult {
            account_email: o.get_field("accountEmail"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            organization_id: o.get_field("organizationId"),
        }
    }
}
