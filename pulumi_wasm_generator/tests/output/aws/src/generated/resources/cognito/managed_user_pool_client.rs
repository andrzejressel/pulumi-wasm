/// Use the `aws.cognito.UserPoolClient` resource to manage a Cognito User Pool Client.
///
/// **This resource is advanced** and has special caveats to consider before use. Please read this document completely before using the resource.
///
/// Use the `aws.cognito.ManagedUserPoolClient` resource to manage a Cognito User Pool Client that is automatically created by an AWS service. For instance, when [configuring an OpenSearch Domain to use Cognito authentication](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html), the OpenSearch service creates the User Pool Client during setup and removes it when it is no longer required. As a result, the `aws.cognito.ManagedUserPoolClient` resource does not create or delete this resource, but instead assumes management of it.
///
/// Use the `aws.cognito.UserPoolClient` resource to manage Cognito User Pool Clients for normal use cases.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleManagedUserPoolClient:
///     type: aws:cognito:ManagedUserPoolClient
///     name: example
///     properties:
///       namePrefix: AmazonOpenSearchService-example
///       userPoolId: ${exampleUserPool.id}
///     options:
///       dependsOn:
///         - ${exampleDomain}
///   exampleUserPool:
///     type: aws:cognito:UserPool
///     name: example
///     properties:
///       name: example
///   exampleIdentityPool:
///     type: aws:cognito:IdentityPool
///     name: example
///     properties:
///       identityPoolName: example
///   exampleDomain:
///     type: aws:opensearch:Domain
///     name: example
///     properties:
///       domainName: example
///       cognitoOptions:
///         enabled: true
///         userPoolId: ${exampleUserPool.id}
///         identityPoolId: ${exampleIdentityPool.id}
///         roleArn: ${exampleRole.arn}
///       ebsOptions:
///         ebsEnabled: true
///         volumeSize: 10
///     options:
///       dependsOn:
///         - ${exampleAwsCognitoUserPoolDomain}
///         - ${exampleRolePolicyAttachment}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       path: /service-role/
///       assumeRolePolicy: ${example.json}
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policyArn: arn:${current.partition}:iam::aws:policy/AmazonESCognitoAccess
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: ""
///             actions:
///               - sts:AssumeRole
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - es.${current.dnsSuffix}
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Pool Clients using the `id` of the Cognito User Pool and the `id` of the Cognito User Pool Client. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/managedUserPoolClient:ManagedUserPoolClient client us-west-2_abc123/3ho4ek12345678909nh3fmhpko
/// ```
pub mod managed_user_pool_client {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedUserPoolClientArgs {
        /// Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
        #[builder(into, default)]
        pub access_token_validity: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub allowed_oauth_flows: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
        #[builder(into, default)]
        pub allowed_oauth_flows_user_pool_client: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub allowed_oauth_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
        #[builder(into, default)]
        pub analytics_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cognito::ManagedUserPoolClientAnalyticsConfiguration,
            >,
        >,
        /// Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
        #[builder(into, default)]
        pub auth_session_validity: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub callback_urls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Default redirect URI and must be included in the list of callback URLs.
        #[builder(into, default)]
        pub default_redirect_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables the propagation of additional user context data.
        #[builder(into, default)]
        pub enable_propagate_additional_user_context_data: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Enables or disables token revocation.
        #[builder(into, default)]
        pub enable_token_revocation: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
        #[builder(into, default)]
        pub explicit_auth_flows: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
        #[builder(into, default)]
        pub id_token_validity: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        #[builder(into, default)]
        pub logout_urls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Regular expression that matches the name of the desired User Pool Client. It must only match one User Pool Client.
        #[builder(into, default)]
        pub name_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// String that matches the beginning of the name of the desired User Pool Client. It must match only one User Pool Client.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
        #[builder(into, default)]
        pub prevent_user_existence_errors: pulumi_wasm_rust::Output<Option<String>>,
        /// List of user pool attributes that the application client can read from.
        #[builder(into, default)]
        pub read_attributes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
        #[builder(into, default)]
        pub refresh_token_validity: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
        #[builder(into, default)]
        pub supported_identity_providers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block for representing the validity times in units. See details below. Detailed below.
        #[builder(into, default)]
        pub token_validity_units: pulumi_wasm_rust::Output<
            Option<super::super::types::cognito::ManagedUserPoolClientTokenValidityUnits>,
        >,
        /// User pool that the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// List of user pool attributes that the application client can write to.
        #[builder(into, default)]
        pub write_attributes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ManagedUserPoolClientResult {
        /// Time limit, between 5 minutes and 1 day, after which the access token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.access_token`.
        pub access_token_validity: pulumi_wasm_rust::Output<i32>,
        /// List of allowed OAuth flows, including `code`, `implicit`, and `client_credentials`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub allowed_oauth_flows: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether the client is allowed to use OAuth 2.0 features. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure the following arguments: `callback_urls`, `logout_urls`, `allowed_oauth_scopes` and `allowed_oauth_flows`.
        pub allowed_oauth_flows_user_pool_client: pulumi_wasm_rust::Output<bool>,
        /// List of allowed OAuth scopes, including `phone`, `email`, `openid`, `profile`, and `aws.cognito.signin.user.admin`. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub allowed_oauth_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Configuration block for Amazon Pinpoint analytics that collects metrics for this user pool. See details below.
        pub analytics_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cognito::ManagedUserPoolClientAnalyticsConfiguration,
            >,
        >,
        /// Duration, in minutes, of the session token created by Amazon Cognito for each API request in an authentication flow. The session token must be responded to by the native user of the user pool before it expires. Valid values for `auth_session_validity` are between `3` and `15`, with a default value of `3`.
        pub auth_session_validity: pulumi_wasm_rust::Output<i32>,
        /// List of allowed callback URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub callback_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// Client secret of the user pool client.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// Default redirect URI and must be included in the list of callback URLs.
        pub default_redirect_uri: pulumi_wasm_rust::Output<String>,
        /// Enables the propagation of additional user context data.
        pub enable_propagate_additional_user_context_data: pulumi_wasm_rust::Output<
            bool,
        >,
        /// Enables or disables token revocation.
        pub enable_token_revocation: pulumi_wasm_rust::Output<bool>,
        /// List of authentication flows. The available options include ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, USER_PASSWORD_AUTH, ALLOW_ADMIN_USER_PASSWORD_AUTH, ALLOW_CUSTOM_AUTH, ALLOW_USER_PASSWORD_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_REFRESH_TOKEN_AUTH.
        pub explicit_auth_flows: pulumi_wasm_rust::Output<Vec<String>>,
        /// Time limit, between 5 minutes and 1 day, after which the ID token is no longer valid and cannot be used. By default, the unit is hours. The unit can be overridden by a value in `token_validity_units.id_token`.
        pub id_token_validity: pulumi_wasm_rust::Output<i32>,
        /// List of allowed logout URLs for the identity providers. `allowed_oauth_flows_user_pool_client` must be set to `true` before you can configure this option.
        pub logout_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the user pool client.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Regular expression that matches the name of the desired User Pool Client. It must only match one User Pool Client.
        pub name_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// String that matches the beginning of the name of the desired User Pool Client. It must match only one User Pool Client.
        ///
        /// The following arguments are optional:
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Setting determines the errors and responses returned by Cognito APIs when a user does not exist in the user pool during authentication, account confirmation, and password recovery.
        pub prevent_user_existence_errors: pulumi_wasm_rust::Output<String>,
        /// List of user pool attributes that the application client can read from.
        pub read_attributes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Time limit, between 60 minutes and 10 years, after which the refresh token is no longer valid and cannot be used. By default, the unit is days. The unit can be overridden by a value in `token_validity_units.refresh_token`.
        pub refresh_token_validity: pulumi_wasm_rust::Output<i32>,
        /// List of provider names for the identity providers that are supported on this client. It uses the `provider_name` attribute of the `aws.cognito.IdentityProvider` resource(s), or the equivalent string(s).
        pub supported_identity_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Configuration block for representing the validity times in units. See details below. Detailed below.
        pub token_validity_units: pulumi_wasm_rust::Output<
            Option<super::super::types::cognito::ManagedUserPoolClientTokenValidityUnits>,
        >,
        /// User pool that the client belongs to.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// List of user pool attributes that the application client can write to.
        pub write_attributes: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedUserPoolClientArgs,
    ) -> ManagedUserPoolClientResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_token_validity_binding = args.access_token_validity.get_inner();
        let allowed_oauth_flows_binding = args.allowed_oauth_flows.get_inner();
        let allowed_oauth_flows_user_pool_client_binding = args
            .allowed_oauth_flows_user_pool_client
            .get_inner();
        let allowed_oauth_scopes_binding = args.allowed_oauth_scopes.get_inner();
        let analytics_configuration_binding = args.analytics_configuration.get_inner();
        let auth_session_validity_binding = args.auth_session_validity.get_inner();
        let callback_urls_binding = args.callback_urls.get_inner();
        let default_redirect_uri_binding = args.default_redirect_uri.get_inner();
        let enable_propagate_additional_user_context_data_binding = args
            .enable_propagate_additional_user_context_data
            .get_inner();
        let enable_token_revocation_binding = args.enable_token_revocation.get_inner();
        let explicit_auth_flows_binding = args.explicit_auth_flows.get_inner();
        let id_token_validity_binding = args.id_token_validity.get_inner();
        let logout_urls_binding = args.logout_urls.get_inner();
        let name_pattern_binding = args.name_pattern.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let prevent_user_existence_errors_binding = args
            .prevent_user_existence_errors
            .get_inner();
        let read_attributes_binding = args.read_attributes.get_inner();
        let refresh_token_validity_binding = args.refresh_token_validity.get_inner();
        let supported_identity_providers_binding = args
            .supported_identity_providers
            .get_inner();
        let token_validity_units_binding = args.token_validity_units.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let write_attributes_binding = args.write_attributes.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/managedUserPoolClient:ManagedUserPoolClient".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessTokenValidity".into(),
                    value: &access_token_validity_binding,
                },
                register_interface::ObjectField {
                    name: "allowedOauthFlows".into(),
                    value: &allowed_oauth_flows_binding,
                },
                register_interface::ObjectField {
                    name: "allowedOauthFlowsUserPoolClient".into(),
                    value: &allowed_oauth_flows_user_pool_client_binding,
                },
                register_interface::ObjectField {
                    name: "allowedOauthScopes".into(),
                    value: &allowed_oauth_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "analyticsConfiguration".into(),
                    value: &analytics_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "authSessionValidity".into(),
                    value: &auth_session_validity_binding,
                },
                register_interface::ObjectField {
                    name: "callbackUrls".into(),
                    value: &callback_urls_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRedirectUri".into(),
                    value: &default_redirect_uri_binding,
                },
                register_interface::ObjectField {
                    name: "enablePropagateAdditionalUserContextData".into(),
                    value: &enable_propagate_additional_user_context_data_binding,
                },
                register_interface::ObjectField {
                    name: "enableTokenRevocation".into(),
                    value: &enable_token_revocation_binding,
                },
                register_interface::ObjectField {
                    name: "explicitAuthFlows".into(),
                    value: &explicit_auth_flows_binding,
                },
                register_interface::ObjectField {
                    name: "idTokenValidity".into(),
                    value: &id_token_validity_binding,
                },
                register_interface::ObjectField {
                    name: "logoutUrls".into(),
                    value: &logout_urls_binding,
                },
                register_interface::ObjectField {
                    name: "namePattern".into(),
                    value: &name_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "preventUserExistenceErrors".into(),
                    value: &prevent_user_existence_errors_binding,
                },
                register_interface::ObjectField {
                    name: "readAttributes".into(),
                    value: &read_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "refreshTokenValidity".into(),
                    value: &refresh_token_validity_binding,
                },
                register_interface::ObjectField {
                    name: "supportedIdentityProviders".into(),
                    value: &supported_identity_providers_binding,
                },
                register_interface::ObjectField {
                    name: "tokenValidityUnits".into(),
                    value: &token_validity_units_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "writeAttributes".into(),
                    value: &write_attributes_binding,
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
                    name: "analyticsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "authSessionValidity".into(),
                },
                register_interface::ResultField {
                    name: "callbackUrls".into(),
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
                    name: "idTokenValidity".into(),
                },
                register_interface::ResultField {
                    name: "logoutUrls".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePattern".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedUserPoolClientResult {
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
            analytics_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticsConfiguration").unwrap(),
            ),
            auth_session_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authSessionValidity").unwrap(),
            ),
            callback_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callbackUrls").unwrap(),
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
            id_token_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idTokenValidity").unwrap(),
            ),
            logout_urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logoutUrls").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePattern").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
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
