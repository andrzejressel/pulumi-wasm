#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_jwt {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountJwtArgs {
        /// Delegate chain of approvals needed to perform full impersonation. Specify the fully qualified service account name.
        #[builder(into, default)]
        pub delegates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Number of seconds until the JWT expires. If set and non-zero an `exp` claim will be added to the payload derived from the current timestamp plus expires_in seconds.
        #[builder(into, default)]
        pub expires_in: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The JSON-encoded JWT claims set to include in the self-signed JWT.
        #[builder(into)]
        pub payload: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The email of the service account that will sign the JWT.
        #[builder(into)]
        pub target_service_account: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountJwtResult {
        pub delegates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub expires_in: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The signed JWT containing the JWT Claims Set from the `payload`.
        pub jwt: pulumi_gestalt_rust::Output<String>,
        pub payload: pulumi_gestalt_rust::Output<String>,
        pub target_service_account: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccountJwtArgs,
    ) -> GetAccountJwtResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let delegates_binding_1 = args.delegates.get_output(context);
        let delegates_binding = delegates_binding_1.get_inner();
        let expires_in_binding_1 = args.expires_in.get_output(context);
        let expires_in_binding = expires_in_binding_1.get_inner();
        let payload_binding_1 = args.payload.get_output(context);
        let payload_binding = payload_binding_1.get_inner();
        let target_service_account_binding_1 = args
            .target_service_account
            .get_output(context);
        let target_service_account_binding = target_service_account_binding_1
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:serviceaccount/getAccountJwt:getAccountJwt".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding,
                },
                register_interface::ObjectField {
                    name: "expiresIn".into(),
                    value: &expires_in_binding,
                },
                register_interface::ObjectField {
                    name: "payload".into(),
                    value: &payload_binding,
                },
                register_interface::ObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountJwtResult {
            delegates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delegates"),
            ),
            expires_in: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresIn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            jwt: pulumi_gestalt_rust::__private::into_domain(o.extract_field("jwt")),
            payload: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("payload"),
            ),
            target_service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetServiceAccount"),
            ),
        }
    }
}
