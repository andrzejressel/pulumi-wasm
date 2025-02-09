#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_access_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountAccessTokenArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.  (e.g. `["projects/-/serviceAccounts/delegate-svc-account@project-id.iam.gserviceaccount.com"]`)
        #[builder(into, default)]
        pub delegates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Lifetime of the impersonated token (defaults to its max: `3600s`).
        #[builder(into, default)]
        pub lifetime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scopes the new credential should have (e.g. `["cloud-platform"]`)
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The service account _to_ impersonate (e.g. `service_B@your-project-id.iam.gserviceaccount.com`)
        #[builder(into)]
        pub target_service_account: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountAccessTokenResult {
        /// The `access_token` representing the new generated identity.
        pub access_token: pulumi_gestalt_rust::Output<String>,
        pub delegates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub lifetime: pulumi_gestalt_rust::Output<Option<String>>,
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        pub target_service_account: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountAccessTokenArgs,
    ) -> GetAccountAccessTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delegates_binding = args.delegates.get_output(context);
        let lifetime_binding = args.lifetime.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let target_service_account_binding = args
            .target_service_account
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:serviceaccount/getAccountAccessToken:getAccountAccessToken"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegates".into(),
                    value: delegates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifetime".into(),
                    value: lifetime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccount".into(),
                    value: target_service_account_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountAccessTokenResult {
            access_token: o.get_field("accessToken"),
            delegates: o.get_field("delegates"),
            id: o.get_field("id"),
            lifetime: o.get_field("lifetime"),
            scopes: o.get_field("scopes"),
            target_service_account: o.get_field("targetServiceAccount"),
        }
    }
}
