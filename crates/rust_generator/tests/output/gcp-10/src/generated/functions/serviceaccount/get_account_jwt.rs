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
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountJwtArgs,
    ) -> GetAccountJwtResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delegates_binding = args.delegates.get_output(context);
        let expires_in_binding = args.expires_in.get_output(context);
        let payload_binding = args.payload.get_output(context);
        let target_service_account_binding = args
            .target_service_account
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:serviceaccount/getAccountJwt:getAccountJwt".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegates".into(),
                    value: &delegates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiresIn".into(),
                    value: &expires_in_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "payload".into(),
                    value: &payload_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetServiceAccount".into(),
                    value: &target_service_account_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountJwtResult {
            delegates: o.get_field("delegates"),
            expires_in: o.get_field("expiresIn"),
            id: o.get_field("id"),
            jwt: o.get_field("jwt"),
            payload: o.get_field("payload"),
            target_service_account: o.get_field("targetServiceAccount"),
        }
    }
}
