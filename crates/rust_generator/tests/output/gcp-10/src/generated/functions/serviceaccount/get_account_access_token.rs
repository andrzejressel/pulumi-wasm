#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccountAccessTokenArgs,
    ) -> GetAccountAccessTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let delegates_binding = args.delegates.get_output(context).get_inner();
        let lifetime_binding = args.lifetime.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let target_service_account_binding = args
            .target_service_account
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:serviceaccount/getAccountAccessToken:getAccountAccessToken"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding,
                },
                register_interface::ObjectField {
                    name: "lifetime".into(),
                    value: &lifetime_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountAccessTokenResult {
            access_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessToken"),
            ),
            delegates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delegates"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            lifetime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifetime"),
            ),
            scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopes"),
            ),
            target_service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetServiceAccount"),
            ),
        }
    }
}
