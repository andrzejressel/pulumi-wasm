pub mod get_account_access_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountAccessTokenArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.  (e.g. `["projects/-/serviceAccounts/delegate-svc-account@project-id.iam.gserviceaccount.com"]`)
        #[builder(into, default)]
        pub delegates: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Lifetime of the impersonated token (defaults to its max: `3600s`).
        #[builder(into, default)]
        pub lifetime: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The scopes the new credential should have (e.g. `["cloud-platform"]`)
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The service account _to_ impersonate (e.g. `service_B@your-project-id.iam.gserviceaccount.com`)
        #[builder(into)]
        pub target_service_account: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountAccessTokenResult {
        /// The `access_token` representing the new generated identity.
        pub access_token: pulumi_wasm_rust::Output<String>,
        pub delegates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub lifetime: pulumi_wasm_rust::Output<Option<String>>,
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        pub target_service_account: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAccountAccessTokenArgs,
    ) -> GetAccountAccessTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessToken".into(),
                },
                register_interface::ResultField {
                    name: "delegates".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lifetime".into(),
                },
                register_interface::ResultField {
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "targetServiceAccount".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountAccessTokenResult {
            access_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessToken").unwrap(),
            ),
            delegates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegates").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            lifetime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifetime").unwrap(),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            target_service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetServiceAccount").unwrap(),
            ),
        }
    }
}
