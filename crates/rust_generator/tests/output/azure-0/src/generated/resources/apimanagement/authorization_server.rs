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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorization_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationServerArgs {
        /// The name of the API Management Service in which this Authorization Server should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The OAUTH Authorization Endpoint.
        #[builder(into)]
        pub authorization_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The HTTP Verbs supported by the Authorization Endpoint. Possible values are `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT` and `TRACE`.
        ///
        /// > **NOTE:** `GET` must always be present.
        #[builder(into)]
        pub authorization_methods: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The mechanism by which Access Tokens are passed to the API. Possible values are `authorizationHeader` and `query`.
        #[builder(into, default)]
        pub bearer_token_sending_methods: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Authentication Methods supported by the Token endpoint of this Authorization Server.. Possible values are `Basic` and `Body`.
        #[builder(into, default)]
        pub client_authentication_methods: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Client/App ID registered with this Authorization Server.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of page where Client/App Registration is performed for this Authorization Server.
        #[builder(into)]
        pub client_registration_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Client/App Secret registered with this Authorization Server.
        #[builder(into, default)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Default Scope used when requesting an Access Token, specified as a string containing space-delimited values.
        #[builder(into, default)]
        pub default_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of the Authorization Server, which may contain HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-friendly name of this Authorization Server.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Form of Authorization Grants required when requesting an Access Token. Possible values are `authorizationCode`, `clientCredentials`, `implicit` and `resourceOwnerPassword`.
        #[builder(into)]
        pub grant_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of this Authorization Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        #[builder(into, default)]
        pub resource_owner_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The username associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        #[builder(into, default)]
        pub resource_owner_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Does this Authorization Server support State? If this is set to `true` the client may use the state parameter to raise protocol security.
        #[builder(into, default)]
        pub support_state: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `token_body_parameter` block as defined below.
        #[builder(into, default)]
        pub token_body_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::apimanagement::AuthorizationServerTokenBodyParameter,
                >,
            >,
        >,
        /// The OAUTH Token Endpoint.
        #[builder(into, default)]
        pub token_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizationServerResult {
        /// The name of the API Management Service in which this Authorization Server should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The OAUTH Authorization Endpoint.
        pub authorization_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The HTTP Verbs supported by the Authorization Endpoint. Possible values are `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT` and `TRACE`.
        ///
        /// > **NOTE:** `GET` must always be present.
        pub authorization_methods: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The mechanism by which Access Tokens are passed to the API. Possible values are `authorizationHeader` and `query`.
        pub bearer_token_sending_methods: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Authentication Methods supported by the Token endpoint of this Authorization Server.. Possible values are `Basic` and `Body`.
        pub client_authentication_methods: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Client/App ID registered with this Authorization Server.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The URI of page where Client/App Registration is performed for this Authorization Server.
        pub client_registration_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The Client/App Secret registered with this Authorization Server.
        pub client_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Default Scope used when requesting an Access Token, specified as a string containing space-delimited values.
        pub default_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of the Authorization Server, which may contain HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user-friendly name of this Authorization Server.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Form of Authorization Grants required when requesting an Access Token. Possible values are `authorizationCode`, `clientCredentials`, `implicit` and `resourceOwnerPassword`.
        pub grant_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of this Authorization Server. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The password associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        pub resource_owner_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The username associated with the Resource Owner.
        ///
        /// > **NOTE:** This can only be specified when `grant_type` includes `resourceOwnerPassword`.
        pub resource_owner_username: pulumi_gestalt_rust::Output<Option<String>>,
        /// Does this Authorization Server support State? If this is set to `true` the client may use the state parameter to raise protocol security.
        pub support_state: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `token_body_parameter` block as defined below.
        pub token_body_parameters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::apimanagement::AuthorizationServerTokenBodyParameter,
                >,
            >,
        >,
        /// The OAUTH Token Endpoint.
        pub token_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizationServerArgs,
    ) -> AuthorizationServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let authorization_endpoint_binding = args
            .authorization_endpoint
            .get_output(context);
        let authorization_methods_binding = args
            .authorization_methods
            .get_output(context);
        let bearer_token_sending_methods_binding = args
            .bearer_token_sending_methods
            .get_output(context);
        let client_authentication_methods_binding = args
            .client_authentication_methods
            .get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_registration_endpoint_binding = args
            .client_registration_endpoint
            .get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let default_scope_binding = args.default_scope.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let grant_types_binding = args.grant_types.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let resource_owner_password_binding = args
            .resource_owner_password
            .get_output(context);
        let resource_owner_username_binding = args
            .resource_owner_username
            .get_output(context);
        let support_state_binding = args.support_state.get_output(context);
        let token_body_parameters_binding = args
            .token_body_parameters
            .get_output(context);
        let token_endpoint_binding = args.token_endpoint.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/authorizationServer:AuthorizationServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationEndpoint".into(),
                    value: authorization_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationMethods".into(),
                    value: authorization_methods_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bearerTokenSendingMethods".into(),
                    value: bearer_token_sending_methods_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientAuthenticationMethods".into(),
                    value: client_authentication_methods_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientRegistrationEndpoint".into(),
                    value: client_registration_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: client_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultScope".into(),
                    value: default_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantTypes".into(),
                    value: grant_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceOwnerPassword".into(),
                    value: resource_owner_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceOwnerUsername".into(),
                    value: resource_owner_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportState".into(),
                    value: support_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenBodyParameters".into(),
                    value: token_body_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenEndpoint".into(),
                    value: token_endpoint_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizationServerResult {
            api_management_name: o.get_field("apiManagementName"),
            authorization_endpoint: o.get_field("authorizationEndpoint"),
            authorization_methods: o.get_field("authorizationMethods"),
            bearer_token_sending_methods: o.get_field("bearerTokenSendingMethods"),
            client_authentication_methods: o.get_field("clientAuthenticationMethods"),
            client_id: o.get_field("clientId"),
            client_registration_endpoint: o.get_field("clientRegistrationEndpoint"),
            client_secret: o.get_field("clientSecret"),
            default_scope: o.get_field("defaultScope"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            grant_types: o.get_field("grantTypes"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_owner_password: o.get_field("resourceOwnerPassword"),
            resource_owner_username: o.get_field("resourceOwnerUsername"),
            support_state: o.get_field("supportState"),
            token_body_parameters: o.get_field("tokenBodyParameters"),
            token_endpoint: o.get_field("tokenEndpoint"),
        }
    }
}
