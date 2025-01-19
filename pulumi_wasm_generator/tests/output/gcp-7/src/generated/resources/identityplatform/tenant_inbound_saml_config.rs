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
pub mod tenant_inbound_saml_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantInboundSamlConfigArgs {
        /// Human friendly display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// If this config allows users to sign in with the provider.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// SAML IdP configuration when the project acts as the relying party
        /// Structure is documented below.
        #[builder(into)]
        pub idp_config: pulumi_wasm_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigIdpConfig,
        >,
        /// The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,
        /// hyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an
        /// alphanumeric character, and have at least 2 characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// SAML SP (Service Provider) configuration when the project acts as the relying party to receive
        /// and accept an authentication assertion issued by a SAML identity provider.
        /// Structure is documented below.
        #[builder(into)]
        pub sp_config: pulumi_wasm_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigSpConfig,
        >,
        /// The name of the tenant where this inbound SAML config resource exists
        #[builder(into)]
        pub tenant: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TenantInboundSamlConfigResult {
        /// Human friendly display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// If this config allows users to sign in with the provider.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// SAML IdP configuration when the project acts as the relying party
        /// Structure is documented below.
        pub idp_config: pulumi_wasm_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigIdpConfig,
        >,
        /// The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,
        /// hyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an
        /// alphanumeric character, and have at least 2 characters.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// SAML SP (Service Provider) configuration when the project acts as the relying party to receive
        /// and accept an authentication assertion issued by a SAML identity provider.
        /// Structure is documented below.
        pub sp_config: pulumi_wasm_rust::Output<
            super::super::types::identityplatform::TenantInboundSamlConfigSpConfig,
        >,
        /// The name of the tenant where this inbound SAML config resource exists
        pub tenant: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TenantInboundSamlConfigArgs,
    ) -> TenantInboundSamlConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let idp_config_binding = args.idp_config.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let sp_config_binding = args.sp_config.get_inner();
        let tenant_binding = args.tenant.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:identityplatform/tenantInboundSamlConfig:TenantInboundSamlConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "idpConfig".into(),
                    value: &idp_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "spConfig".into(),
                    value: &sp_config_binding,
                },
                register_interface::ObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "idpConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "spConfig".into(),
                },
                register_interface::ResultField {
                    name: "tenant".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TenantInboundSamlConfigResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            idp_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idpConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            sp_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spConfig").unwrap(),
            ),
            tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenant").unwrap(),
            ),
        }
    }
}
