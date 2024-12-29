pub mod get_user_pool_client {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolClientArgs {
        /// Client Id of the user pool.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetUserPoolClientArgs) -> GetUserPoolClientResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_id_binding = args.client_id.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPoolClient:getUserPoolClient".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessTokenValidity".into(),
                },
                register_interface::ResultField {
                    name: "allowedOauthFlows".into(),
                },
                register_interface::ResultField {
                    name: "allowedOauthFlowsUserPoolClient".into(),
                },
                register_interface::ResultField {
                    name: "allowedOauthScopes".into(),
                },
                register_interface::ResultField {
                    name: "analyticsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "callbackUrls".into(),
                },
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "defaultRedirectUri".into(),
                },
                register_interface::ResultField {
                    name: "enablePropagateAdditionalUserContextData".into(),
                },
                register_interface::ResultField {
                    name: "enableTokenRevocation".into(),
                },
                register_interface::ResultField {
                    name: "explicitAuthFlows".into(),
                },
                register_interface::ResultField {
                    name: "generateSecret".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idTokenValidity".into(),
                },
                register_interface::ResultField {
                    name: "logoutUrls".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "preventUserExistenceErrors".into(),
                },
                register_interface::ResultField {
                    name: "readAttributes".into(),
                },
                register_interface::ResultField {
                    name: "refreshTokenValidity".into(),
                },
                register_interface::ResultField {
                    name: "supportedIdentityProviders".into(),
                },
                register_interface::ResultField {
                    name: "tokenValidityUnits".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
                register_interface::ResultField {
                    name: "writeAttributes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserPoolClientResult {
            access_token_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessTokenValidity").unwrap(),
            ),
            allowed_oauth_flows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedOauthFlows").unwrap(),
            ),
            allowed_oauth_flows_user_pool_client: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedOauthFlowsUserPoolClient").unwrap(),
            ),
            allowed_oauth_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedOauthScopes").unwrap(),
            ),
            analytics_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticsConfigurations").unwrap(),
            ),
            callback_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callbackUrls").unwrap(),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            default_redirect_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRedirectUri").unwrap(),
            ),
            enable_propagate_additional_user_context_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enablePropagateAdditionalUserContextData").unwrap(),
            ),
            enable_token_revocation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTokenRevocation").unwrap(),
            ),
            explicit_auth_flows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("explicitAuthFlows").unwrap(),
            ),
            generate_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generateSecret").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            id_token_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idTokenValidity").unwrap(),
            ),
            logout_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logoutUrls").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            prevent_user_existence_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preventUserExistenceErrors").unwrap(),
            ),
            read_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readAttributes").unwrap(),
            ),
            refresh_token_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshTokenValidity").unwrap(),
            ),
            supported_identity_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedIdentityProviders").unwrap(),
            ),
            token_validity_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenValidityUnits").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
            write_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writeAttributes").unwrap(),
            ),
        }
    }
}
