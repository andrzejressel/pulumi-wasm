pub mod get_account_id_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountIdTokenArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.   Used only when using impersonation mode.
        #[builder(into, default)]
        pub delegates: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Include the verified email in the claim. Used only when using impersonation mode.
        #[builder(into, default)]
        pub include_email: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The audience claim for the `id_token`.
        #[builder(into)]
        pub target_audience: pulumi_wasm_rust::InputOrOutput<String>,
        /// The email of the service account being impersonated.  Used only when using impersonation mode.
        #[builder(into, default)]
        pub target_service_account: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountIdTokenResult {
        pub delegates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The `id_token` representing the new generated identity.
        pub id_token: pulumi_wasm_rust::Output<String>,
        pub include_email: pulumi_wasm_rust::Output<Option<bool>>,
        pub target_audience: pulumi_wasm_rust::Output<String>,
        pub target_service_account: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAccountIdTokenArgs,
    ) -> GetAccountIdTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delegates_binding = args.delegates.get_output(context).get_inner();
        let include_email_binding = args.include_email.get_output(context).get_inner();
        let target_audience_binding = args
            .target_audience
            .get_output(context)
            .get_inner();
        let target_service_account_binding = args
            .target_service_account
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:serviceaccount/getAccountIdToken:getAccountIdToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding,
                },
                register_interface::ObjectField {
                    name: "includeEmail".into(),
                    value: &include_email_binding,
                },
                register_interface::ObjectField {
                    name: "targetAudience".into(),
                    value: &target_audience_binding,
                },
                register_interface::ObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountIdTokenResult {
            delegates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("delegates"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            id_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("idToken"),
            ),
            include_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeEmail"),
            ),
            target_audience: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetAudience"),
            ),
            target_service_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetServiceAccount"),
            ),
        }
    }
}
