pub mod get_user_pool_client {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolClientArgs {
        /// Client Id of the user pool.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserPoolClientResult {
        /// (Optional) Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. This value will be overridden if you have entered a value in `token_validity_units`.
        pub access_token_validity: pulumi_wasm_rust::Output<i32>,
        /// (Optional) List of allowed OAuth flows (code, implicit, client_credentials).
        pub allowed_oauth_flows: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) Whether the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.
        pub allowed_oauth_flows_user_pool_client: pulumi_wasm_rust::Output<bool>,
        /// (Optional) List of allowed OAuth scopes (phone, email, openid, profile, and aws.cognito.signin.user.admin).
        pub allowed_oauth_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) Configuration block for Amazon Pinpoint analytics for collecting metrics for this user pool. Detailed below.
        pub analytics_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cognito::GetUserPoolClientAnalyticsConfiguration,
            >,
        >,
        /// (Optional) List of allowed callback URLs for the identity providers.
        pub callback_urls: pulumi_wasm_rust::Output<Vec<String>>,
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// Client secret of the user pool client.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// (Optional) Default redirect URI. Must be in the list of callback URLs.
        pub default_redirect_uri: pulumi_wasm_rust::Output<String>,
        pub enable_propagate_additional_user_context_data: pulumi_wasm_rust::Output<
            bool,
        >,
        /// (Optional) Enables or disables token revocation.
        pub enable_token_revocation: pulumi_wasm_rust::Output<bool>,
        /// (Optional) List of authentication flows (ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, ALLOW_REFRESH_TOKEN_AUTH).
        pub explicit_auth_flows: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) Should an application secret be generated.
        pub generate_secret: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// (Optional) Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. This value will be overridden if you have entered a value in `token_validity_units`.
        pub id_token_validity: pulumi_wasm_rust::Output<i32>,
        /// (Optional) List of allowed logout URLs for the identity providers.
        pub logout_urls: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// (Optional) Choose which errors and responses are returned by Cognito APIs during authentication, account confirmation, and password recovery when the user does not exist in the user pool. When set to `ENABLED` and the user does not exist, authentication returns an error indicating either the username or password was incorrect, and account confirmation and password recovery return a response indicating a code was sent to a simulated destination. When set to `LEGACY`, those APIs will return a `UserNotFoundException` exception if the user does not exist in the user pool.
        pub prevent_user_existence_errors: pulumi_wasm_rust::Output<String>,
        /// (Optional) List of user pool attributes the application client can read from.
        pub read_attributes: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) Time limit in days refresh tokens are valid for.
        pub refresh_token_validity: pulumi_wasm_rust::Output<i32>,
        /// (Optional) List of provider names for the identity providers that are supported on this client. Uses the `provider_name` attribute of `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
        pub supported_identity_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) Configuration block for units in which the validity times are represented in. Detailed below.
        pub token_validity_units: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolClientTokenValidityUnit>,
        >,
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// (Optional) List of user pool attributes the application client can write to.
        pub write_attributes: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserPoolClientArgs,
    ) -> GetUserPoolClientResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPoolClient:getUserPoolClient".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserPoolClientResult {
            access_token_validity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessTokenValidity"),
            ),
            allowed_oauth_flows: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedOauthFlows"),
            ),
            allowed_oauth_flows_user_pool_client: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedOauthFlowsUserPoolClient"),
            ),
            allowed_oauth_scopes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedOauthScopes"),
            ),
            analytics_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("analyticsConfigurations"),
            ),
            callback_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("callbackUrls"),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientSecret"),
            ),
            default_redirect_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultRedirectUri"),
            ),
            enable_propagate_additional_user_context_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enablePropagateAdditionalUserContextData"),
            ),
            enable_token_revocation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableTokenRevocation"),
            ),
            explicit_auth_flows: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("explicitAuthFlows"),
            ),
            generate_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generateSecret"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            id_token_validity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("idTokenValidity"),
            ),
            logout_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logoutUrls"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            prevent_user_existence_errors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preventUserExistenceErrors"),
            ),
            read_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readAttributes"),
            ),
            refresh_token_validity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("refreshTokenValidity"),
            ),
            supported_identity_providers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportedIdentityProviders"),
            ),
            token_validity_units: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tokenValidityUnits"),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
            write_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("writeAttributes"),
            ),
        }
    }
}
