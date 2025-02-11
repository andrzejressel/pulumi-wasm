/// Inbound SAML configuration for a Identity Toolkit tenant.
///
/// You must enable the
/// [Google Identity Platform](https://console.cloud.google.com/marketplace/details/google-cloud-platform/customer-identity) in
/// the marketplace prior to using this resource.
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Tenant Inbound Saml Config Basic
///
///
/// ```yaml
/// resources:
///   tenant:
///     type: gcp:identityplatform:Tenant
///     properties:
///       displayName: tenant
///   tenantSamlConfig:
///     type: gcp:identityplatform:TenantInboundSamlConfig
///     name: tenant_saml_config
///     properties:
///       name: saml.tf-config
///       displayName: Display Name
///       tenant: ${tenant.name}
///       idpConfig:
///         idpEntityId: tf-idp
///         signRequest: true
///         ssoUrl: https://example.com
///         idpCertificates:
///           - x509Certificate:
///               fn::invoke:
///                 function: std:file
///                 arguments:
///                   input: test-fixtures/rsa_cert.pem
///                 return: result
///       spConfig:
///         spEntityId: tf-sp
///         callbackUri: https://example.com
/// ```
///
/// ## Import
///
/// TenantInboundSamlConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/tenants/{{tenant}}/inboundSamlConfigs/{{name}}`
///
/// * `{{project}}/{{tenant}}/{{name}}`
///
/// * `{{tenant}}/{{name}}`
///
/// When using the `pulumi import` command, TenantInboundSamlConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantInboundSamlConfig:TenantInboundSamlConfig default projects/{{project}}/tenants/{{tenant}}/inboundSamlConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantInboundSamlConfig:TenantInboundSamlConfig default {{project}}/{{tenant}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenantInboundSamlConfig:TenantInboundSamlConfig default {{tenant}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tenant_inbound_saml_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantInboundSamlConfigArgs {
        /// Human friendly display name.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If this config allows users to sign in with the provider.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// SAML IdP configuration when the project acts as the relying party
        /// Structure is documented below.
        #[builder(into)]
        pub idp_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::identityplatform::TenantInboundSamlConfigIdpConfig,
        >,
        /// The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,
        /// hyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an
        /// alphanumeric character, and have at least 2 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SAML SP (Service Provider) configuration when the project acts as the relying party to receive
        /// and accept an authentication assertion issued by a SAML identity provider.
        /// Structure is documented below.
        #[builder(into)]
        pub sp_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::identityplatform::TenantInboundSamlConfigSpConfig,
        >,
        /// The name of the tenant where this inbound SAML config resource exists
        #[builder(into)]
        pub tenant: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TenantInboundSamlConfigResult {
        /// Human friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// If this config allows users to sign in with the provider.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// SAML IdP configuration when the project acts as the relying party
        /// Structure is documented below.
        pub idp_config: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigIdpConfig,
        >,
        /// The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,
        /// hyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an
        /// alphanumeric character, and have at least 2 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// SAML SP (Service Provider) configuration when the project acts as the relying party to receive
        /// and accept an authentication assertion issued by a SAML identity provider.
        /// Structure is documented below.
        pub sp_config: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigSpConfig,
        >,
        /// The name of the tenant where this inbound SAML config resource exists
        pub tenant: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TenantInboundSamlConfigArgs,
    ) -> TenantInboundSamlConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let idp_config_binding = args.idp_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let sp_config_binding = args.sp_config.get_output(context);
        let tenant_binding = args.tenant.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:identityplatform/tenantInboundSamlConfig:TenantInboundSamlConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpConfig".into(),
                    value: &idp_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spConfig".into(),
                    value: &sp_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TenantInboundSamlConfigResult {
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            idp_config: o.get_field("idpConfig"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            sp_config: o.get_field("spConfig"),
            tenant: o.get_field("tenant"),
        }
    }
}
