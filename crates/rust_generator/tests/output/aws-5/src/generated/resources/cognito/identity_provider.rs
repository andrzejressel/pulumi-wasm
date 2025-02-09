/// Provides a Cognito User Identity Provider resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cognito:UserPool
///     properties:
///       name: example-pool
///       autoVerifiedAttributes:
///         - email
///   exampleProvider:
///     type: aws:cognito:IdentityProvider
///     name: example_provider
///     properties:
///       userPoolId: ${example.id}
///       providerName: Google
///       providerType: Google
///       providerDetails:
///         authorize_scopes: email
///         client_id: your client_id
///         client_secret: your client_secret
///       attributeMapping:
///         email: email
///         username: sub
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cognito_identity_provider` resources using their User Pool ID and Provider Name. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityProvider:IdentityProvider example us-west-2_abc123:CorpAD
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderArgs {
        /// The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
        #[builder(into, default)]
        pub attribute_mapping: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of identity providers.
        #[builder(into, default)]
        pub idp_identifiers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The map of identity details, such as access token
        #[builder(into)]
        pub provider_details: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The provider name
        #[builder(into)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
        #[builder(into)]
        pub provider_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user pool id
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderResult {
        /// The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
        pub attribute_mapping: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The list of identity providers.
        pub idp_identifiers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The map of identity details, such as access token
        pub provider_details: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider name
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
        pub provider_type: pulumi_gestalt_rust::Output<String>,
        /// The user pool id
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityProviderArgs,
    ) -> IdentityProviderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attribute_mapping_binding = args.attribute_mapping.get_output(context);
        let idp_identifiers_binding = args.idp_identifiers.get_output(context);
        let provider_details_binding = args.provider_details.get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let provider_type_binding = args.provider_type.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/identityProvider:IdentityProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributeMapping".into(),
                    value: attribute_mapping_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpIdentifiers".into(),
                    value: idp_identifiers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerDetails".into(),
                    value: provider_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerType".into(),
                    value: provider_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: user_pool_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityProviderResult {
            attribute_mapping: o.get_field("attributeMapping"),
            idp_identifiers: o.get_field("idpIdentifiers"),
            provider_details: o.get_field("providerDetails"),
            provider_name: o.get_field("providerName"),
            provider_type: o.get_field("providerType"),
            user_pool_id: o.get_field("userPoolId"),
        }
    }
}
