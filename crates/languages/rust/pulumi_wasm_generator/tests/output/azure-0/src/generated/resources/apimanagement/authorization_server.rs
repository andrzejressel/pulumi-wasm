/// Manages an Authorization Server within an API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleAuthorizationServer:
///     type: azure:apimanagement:AuthorizationServer
///     name: example
///     properties:
///       name: test-server
///       apiManagementName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       displayName: Test Server
///       authorizationEndpoint: https://example.mydomain.com/client/authorize
///       clientId: 42424242-4242-4242-4242-424242424242
///       clientRegistrationEndpoint: https://example.mydomain.com/client/register
///       grantTypes:
///         - authorizationCode
///       authorizationMethods:
///         - GET
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: search-api
///         resourceGroupName: search-service
/// ```
///
/// ## Import
///
/// API Management Authorization Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/authorizationServer:AuthorizationServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/authorizationServers/server1
/// ```
///
pub mod authorization_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationServerArgs {
        /// The name of the API Management Service in which this Authorization Server should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The OAUTH Authorization Endpoint.
        #[builder(into)]
        pub authorization_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// The HTTP Verbs supported by the Authorization Endpoint. Possible values are `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT` and `TRACE`.
        ///
        /// > **NOTE:** `GET` must always be present.
        #[builder(into)]
        pub authorization_methods: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The mechanism by which Access Tokens are passed to the API. Possible values are `authorizationHeader` and `query`.
        #[builder(into, default)]
        pub bearer_token_sending_methods: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Authentication Methods supported by the Token endpoint of this Authorization Server.. Possible values are `Basic` and `Body`.
        #[builder(into, default)]
        pub client_authentication_methods: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Client/App ID registered with this Authorization Server.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The URI of page where Client/App Registration is performed for this Authorization Server.
        #[builder(into)]
        pub client_registration_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Client/App Secret registered with this Authorization Server.
        #[builder(into, default)]
        pub client_secret: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Default Scope used when requesting an Access Token, specified as a string containing space-delimited values.
        #[builder(into, default)]
        pub default_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A description of the Authorization Server, which may contain HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user-friendly name of this Authorization Server.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Form of Authorization Grants required when requesting an Access Token. Possible values are `authorizationCode`, `clientCredentials`, `implicit` and `resourceOwnerPassword`.
        #[builder(into)]
        pub grant_types: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The name of this Authorization Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The password associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        #[builder(into, default)]
        pub resource_owner_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The username associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        #[builder(into, default)]
        pub resource_owner_username: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Does this Authorization Server support State? If this is set to `true` the client may use the state parameter to raise protocol security.
        #[builder(into, default)]
        pub support_state: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `token_body_parameter` block as defined below.
        #[builder(into, default)]
        pub token_body_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::apimanagement::AuthorizationServerTokenBodyParameter,
                >,
            >,
        >,
        /// The OAUTH Token Endpoint.
        #[builder(into, default)]
        pub token_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizationServerResult {
        /// The name of the API Management Service in which this Authorization Server should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The OAUTH Authorization Endpoint.
        pub authorization_endpoint: pulumi_wasm_rust::Output<String>,
        /// The HTTP Verbs supported by the Authorization Endpoint. Possible values are `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT` and `TRACE`.
        ///
        /// > **NOTE:** `GET` must always be present.
        pub authorization_methods: pulumi_wasm_rust::Output<Vec<String>>,
        /// The mechanism by which Access Tokens are passed to the API. Possible values are `authorizationHeader` and `query`.
        pub bearer_token_sending_methods: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Authentication Methods supported by the Token endpoint of this Authorization Server.. Possible values are `Basic` and `Body`.
        pub client_authentication_methods: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Client/App ID registered with this Authorization Server.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The URI of page where Client/App Registration is performed for this Authorization Server.
        pub client_registration_endpoint: pulumi_wasm_rust::Output<String>,
        /// The Client/App Secret registered with this Authorization Server.
        pub client_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The Default Scope used when requesting an Access Token, specified as a string containing space-delimited values.
        pub default_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the Authorization Server, which may contain HTML formatting tags.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The user-friendly name of this Authorization Server.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Form of Authorization Grants required when requesting an Access Token. Possible values are `authorizationCode`, `clientCredentials`, `implicit` and `resourceOwnerPassword`.
        pub grant_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of this Authorization Server. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The password associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        pub resource_owner_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The username associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        pub resource_owner_username: pulumi_wasm_rust::Output<Option<String>>,
        /// Does this Authorization Server support State? If this is set to `true` the client may use the state parameter to raise protocol security.
        pub support_state: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `token_body_parameter` block as defined below.
        pub token_body_parameters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::apimanagement::AuthorizationServerTokenBodyParameter,
                >,
            >,
        >,
        /// The OAUTH Token Endpoint.
        pub token_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthorizationServerArgs,
    ) -> AuthorizationServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let authorization_endpoint_binding = args
            .authorization_endpoint
            .get_output(context)
            .get_inner();
        let authorization_methods_binding = args
            .authorization_methods
            .get_output(context)
            .get_inner();
        let bearer_token_sending_methods_binding = args
            .bearer_token_sending_methods
            .get_output(context)
            .get_inner();
        let client_authentication_methods_binding = args
            .client_authentication_methods
            .get_output(context)
            .get_inner();
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let client_registration_endpoint_binding = args
            .client_registration_endpoint
            .get_output(context)
            .get_inner();
        let client_secret_binding = args.client_secret.get_output(context).get_inner();
        let default_scope_binding = args.default_scope.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let grant_types_binding = args.grant_types.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let resource_owner_password_binding = args
            .resource_owner_password
            .get_output(context)
            .get_inner();
        let resource_owner_username_binding = args
            .resource_owner_username
            .get_output(context)
            .get_inner();
        let support_state_binding = args.support_state.get_output(context).get_inner();
        let token_body_parameters_binding = args
            .token_body_parameters
            .get_output(context)
            .get_inner();
        let token_endpoint_binding = args.token_endpoint.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/authorizationServer:AuthorizationServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationEndpoint".into(),
                    value: &authorization_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationMethods".into(),
                    value: &authorization_methods_binding,
                },
                register_interface::ObjectField {
                    name: "bearerTokenSendingMethods".into(),
                    value: &bearer_token_sending_methods_binding,
                },
                register_interface::ObjectField {
                    name: "clientAuthenticationMethods".into(),
                    value: &client_authentication_methods_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientRegistrationEndpoint".into(),
                    value: &client_registration_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "defaultScope".into(),
                    value: &default_scope_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "grantTypes".into(),
                    value: &grant_types_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceOwnerPassword".into(),
                    value: &resource_owner_password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceOwnerUsername".into(),
                    value: &resource_owner_username_binding,
                },
                register_interface::ObjectField {
                    name: "supportState".into(),
                    value: &support_state_binding,
                },
                register_interface::ObjectField {
                    name: "tokenBodyParameters".into(),
                    value: &token_body_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "tokenEndpoint".into(),
                    value: &token_endpoint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AuthorizationServerResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            authorization_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationEndpoint"),
            ),
            authorization_methods: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationMethods"),
            ),
            bearer_token_sending_methods: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bearerTokenSendingMethods"),
            ),
            client_authentication_methods: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientAuthenticationMethods"),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            client_registration_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientRegistrationEndpoint"),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientSecret"),
            ),
            default_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultScope"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            grant_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("grantTypes"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            resource_owner_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceOwnerPassword"),
            ),
            resource_owner_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceOwnerUsername"),
            ),
            support_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportState"),
            ),
            token_body_parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tokenBodyParameters"),
            ),
            token_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tokenEndpoint"),
            ),
        }
    }
}
