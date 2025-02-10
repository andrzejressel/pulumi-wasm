#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityPolicyArgs {
        /// The name of the security policy. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self_link of the security policy. Provide either this or a `name`
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecurityPolicyResult {
        pub adaptive_protection_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfig,
            >,
        >,
        pub advanced_options_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyAdvancedOptionsConfig,
            >,
        >,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub recaptcha_options_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyRecaptchaOptionsConfig,
            >,
        >,
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSecurityPolicyRule>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecurityPolicyArgs,
    ) -> GetSecurityPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getSecurityPolicy:getSecurityPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: self_link_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecurityPolicyResult {
            adaptive_protection_configs: o.get_field("adaptiveProtectionConfigs"),
            advanced_options_configs: o.get_field("advancedOptionsConfigs"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            recaptcha_options_configs: o.get_field("recaptchaOptionsConfigs"),
            rules: o.get_field("rules"),
            self_link: o.get_field("selfLink"),
            type_: o.get_field("type"),
        }
    }
}
