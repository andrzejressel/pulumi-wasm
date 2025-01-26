/// Provides an AWS Cognito Identity Principal Mapping.
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID and provider name. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPoolProviderPrincipalTag:IdentityPoolProviderPrincipalTag example us-west-2_abc123:CorpAD
/// ```
pub mod identity_pool_provider_principal_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagArgs {
        /// An identity pool ID.
        #[builder(into)]
        pub identity_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the identity provider.
        #[builder(into)]
        pub identity_provider_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// String to string map of variables.
        #[builder(into, default)]
        pub principal_tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        #[builder(into, default)]
        pub use_defaults: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolProviderPrincipalTagResult {
        /// An identity pool ID.
        pub identity_pool_id: pulumi_wasm_rust::Output<String>,
        /// The name of the identity provider.
        pub identity_provider_name: pulumi_wasm_rust::Output<String>,
        /// String to string map of variables.
        pub principal_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// use default (username and clientID) attribute mappings.
        pub use_defaults: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IdentityPoolProviderPrincipalTagArgs,
    ) -> IdentityPoolProviderPrincipalTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "identityPoolId".into(),
                },
                register_interface::ResultField {
                    name: "identityProviderName".into(),
                },
                register_interface::ResultField {
                    name: "principalTags".into(),
                },
                register_interface::ResultField {
                    name: "useDefaults".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityPoolProviderPrincipalTagResult {
            identity_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityPoolId").unwrap(),
            ),
            identity_provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityProviderName").unwrap(),
            ),
            principal_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalTags").unwrap(),
            ),
            use_defaults: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useDefaults").unwrap(),
            ),
        }
    }
}
