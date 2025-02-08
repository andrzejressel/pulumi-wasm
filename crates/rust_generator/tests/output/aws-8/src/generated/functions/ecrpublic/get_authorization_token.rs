#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_authorization_token {
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenResult {
        /// Temporary IAM authentication credentials to access the ECR repository encoded in base64 in the form of `user_name:password`.
        pub authorization_token: pulumi_gestalt_rust::Output<String>,
        /// Time in UTC RFC3339 format when the authorization token expires.
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Password decoded from the authorization token.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// User name decoded from the authorization token.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
    ) -> GetAuthorizationTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecrpublic/getAuthorizationToken:getAuthorizationToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthorizationTokenResult {
            authorization_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationToken"),
            ),
            expires_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresAt"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            user_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
