#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_id_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountIdTokenArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.   Used only when using impersonation mode.
        #[builder(into, default)]
        pub delegates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Include the verified email in the claim. Used only when using impersonation mode.
        #[builder(into, default)]
        pub include_email: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The audience claim for the `id_token`.
        #[builder(into)]
        pub target_audience: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The email of the service account being impersonated.  Used only when using impersonation mode.
        #[builder(into, default)]
        pub target_service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountIdTokenResult {
        pub delegates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The `id_token` representing the new generated identity.
        pub id_token: pulumi_gestalt_rust::Output<String>,
        pub include_email: pulumi_gestalt_rust::Output<Option<bool>>,
        pub target_audience: pulumi_gestalt_rust::Output<String>,
        pub target_service_account: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountIdTokenArgs,
    ) -> GetAccountIdTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delegates_binding = args.delegates.get_output(context);
        let include_email_binding = args.include_email.get_output(context);
        let target_audience_binding = args.target_audience.get_output(context);
        let target_service_account_binding = args
            .target_service_account
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:serviceaccount/getAccountIdToken:getAccountIdToken".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeEmail".into(),
                    value: &include_email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetAudience".into(),
                    value: &target_audience_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountIdTokenResult {
            delegates: o.get_field("delegates"),
            id: o.get_field("id"),
            id_token: o.get_field("idToken"),
            include_email: o.get_field("includeEmail"),
            target_audience: o.get_field("targetAudience"),
            target_service_account: o.get_field("targetServiceAccount"),
        }
    }
}
