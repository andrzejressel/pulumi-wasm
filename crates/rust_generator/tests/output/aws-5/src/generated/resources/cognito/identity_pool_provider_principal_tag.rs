/// Provides an AWS Cognito Identity Principal Mapping.
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID and provider name. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag example us-west-2_abc123:CorpAD
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_pool_provider_principal_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagArgs {
        /// An identity pool ID.
        #[builder(into)]
        pub identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the identity provider.
        #[builder(into)]
        pub identity_provider_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// String to string map of variables.
        #[builder(into, default)]
        pub principal_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        #[builder(into, default)]
        pub use_defaults: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagResult {
        /// An identity pool ID.
        pub identity_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the identity provider.
        pub identity_provider_name: pulumi_gestalt_rust::Output<String>,
        /// String to string map of variables.
        pub principal_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        pub use_defaults: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
    ) -> IdentityPoolProviderPrincipalTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_pool_id_binding = args.identity_pool_id.get_output(context);
        let identity_provider_name_binding = args
            .identity_provider_name
            .get_output(context);
        let principal_tags_binding = args.principal_tags.get_output(context);
        let use_defaults_binding = args.use_defaults.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityPoolId".into(),
                    value: identity_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityProviderName".into(),
                    value: identity_provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalTags".into(),
                    value: principal_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useDefaults".into(),
                    value: use_defaults_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityPoolProviderPrincipalTagResult {
            identity_pool_id: o.get_field("identityPoolId"),
            identity_provider_name: o.get_field("identityProviderName"),
            principal_tags: o.get_field("principalTags"),
            use_defaults: o.get_field("useDefaults"),
        }
    }
}
