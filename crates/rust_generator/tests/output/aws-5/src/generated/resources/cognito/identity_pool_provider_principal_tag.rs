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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
    ) -> IdentityPoolProviderPrincipalTagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identity_pool_id_binding = args
            .identity_pool_id
            .get_output(context)
            .get_inner();
        let identity_provider_name_binding = args
            .identity_provider_name
            .get_output(context)
            .get_inner();
        let principal_tags_binding = args.principal_tags.get_output(context).get_inner();
        let use_defaults_binding = args.use_defaults.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityPoolId".into(),
                    value: &identity_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityProviderName".into(),
                    value: &identity_provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "principalTags".into(),
                    value: &principal_tags_binding,
                },
                register_interface::ObjectField {
                    name: "useDefaults".into(),
                    value: &use_defaults_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IdentityPoolProviderPrincipalTagResult {
            identity_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityPoolId"),
            ),
            identity_provider_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityProviderName"),
            ),
            principal_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalTags"),
            ),
            use_defaults: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useDefaults"),
            ),
        }
    }
}
