/// Provides a resource which manages Cloudflare API tokens.
///
/// Read more about permission groups and their applicable scopes in the
/// [developer documentation](https://developers.cloudflare.com/api/tokens/create/permissions).
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiTokenArgs {
        /// Conditions under which the token should be considered valid.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ApiTokenCondition>,
        >,
        /// The expiration time on or after which the token MUST NOT be accepted for processing.
        #[builder(into, default)]
        pub expires_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the API Token.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time before which the token MUST NOT be accepted for processing.
        #[builder(into, default)]
        pub not_before: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Permissions policy. Multiple policy blocks can be defined.
        #[builder(into)]
        pub policies: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::ApiTokenPolicy>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApiTokenResult {
        /// Conditions under which the token should be considered valid.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::types::ApiTokenCondition>,
        >,
        /// The expiration time on or after which the token MUST NOT be accepted for processing.
        pub expires_on: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timestamp of when the token was issued.
        pub issued_on: pulumi_gestalt_rust::Output<String>,
        /// Timestamp of when the token was last modified.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// Name of the API Token.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The time before which the token MUST NOT be accepted for processing.
        pub not_before: pulumi_gestalt_rust::Output<Option<String>>,
        /// Permissions policy. Multiple policy blocks can be defined.
        pub policies: pulumi_gestalt_rust::Output<Vec<super::types::ApiTokenPolicy>>,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The value of the API Token.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTokenArgs,
    ) -> ApiTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let expires_on_binding = args.expires_on.get_output(context);
        let name_binding = args.name.get_output(context);
        let not_before_binding = args.not_before.get_output(context);
        let policies_binding = args.policies.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiToken:ApiToken".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiresOn".into(),
                    value: expires_on_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notBefore".into(),
                    value: not_before_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: policies_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiTokenResult {
            condition: o.get_field("condition"),
            expires_on: o.get_field("expiresOn"),
            issued_on: o.get_field("issuedOn"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            not_before: o.get_field("notBefore"),
            policies: o.get_field("policies"),
            status: o.get_field("status"),
            value: o.get_field("value"),
        }
    }
}
