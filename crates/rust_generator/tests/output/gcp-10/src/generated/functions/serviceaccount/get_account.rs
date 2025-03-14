#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The Google service account ID. This be one of:
        ///
        /// * The name of the service account within the project (e.g. `my-service`)
        ///
        /// * The fully-qualified path to a service account resource (e.g.
        /// `projects/my-project/serviceAccounts/...`)
        ///
        /// * The email address of the service account (e.g.
        /// `my-service@my-project.iam.gserviceaccount.com`)
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project that the service account is present in.
        /// Defaults to the provider project configuration.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether a service account is disabled or not.
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        /// The display name for the service account.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The e-mail address of the service account. This value
        /// should be referenced from any `gcp.organizations.getIAMPolicy` data sources
        /// that would grant the service account privileges.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Identity of the service account in the form `serviceAccount:{email}`. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The fully-qualified name of the service account.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique id of the service account.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:serviceaccount/getAccount:getAccount".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountResult {
            account_id: o.get_field("accountId"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            email: o.get_field("email"),
            id: o.get_field("id"),
            member: o.get_field("member"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
