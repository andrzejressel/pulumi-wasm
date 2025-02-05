/// The [Risk Score Integration](https://developers.cloudflare.com/cloudflare-one/insights/risk-score/#send-risk-score-to-okta) resource allows you to transmit changes in User Risk Score to a specified vendor such as Okta.
pub mod zero_trust_risk_score_integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustRiskScoreIntegrationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether this integration is enabled. If disabled, no risk changes will be exported to the third-party.
        #[builder(into, default)]
        pub active: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The type of integration, e.g. 'Okta'. Full list of allowed values can be found here: https://developers.cloudflare.com/api/operations/dlp-zt-risk-score-integration-create#request-body
        #[builder(into)]
        pub integration_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// A reference id that can be supplied by the client. Currently this should be set to the Access-Okta IDP ID (a UUIDv4). If omitted, a random UUIDv4 is used. https://developers.cloudflare.com/api/operations/access-identity-providers-get-an-access-identity-provider
        #[builder(into, default)]
        pub reference_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The base url of the tenant, e.g. 'https://tenant.okta.com'. Must be your Okta Tenant URL and not your custom domain.
        #[builder(into)]
        pub tenant_url: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustRiskScoreIntegrationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether this integration is enabled. If disabled, no risk changes will be exported to the third-party.
        pub active: pulumi_wasm_rust::Output<bool>,
        /// The type of integration, e.g. 'Okta'. Full list of allowed values can be found here: https://developers.cloudflare.com/api/operations/dlp-zt-risk-score-integration-create#request-body
        pub integration_type: pulumi_wasm_rust::Output<String>,
        /// A reference id that can be supplied by the client. Currently this should be set to the Access-Okta IDP ID (a UUIDv4). If omitted, a random UUIDv4 is used. https://developers.cloudflare.com/api/operations/access-identity-providers-get-an-access-identity-provider
        pub reference_id: pulumi_wasm_rust::Output<String>,
        /// The base url of the tenant, e.g. 'https://tenant.okta.com'. Must be your Okta Tenant URL and not your custom domain.
        pub tenant_url: pulumi_wasm_rust::Output<String>,
        /// The URL for the Shared Signals Framework configuration, e.g. '/.well-known/sse-configuration/{integration*uuid}/'. https://openid.net/specs/openid-sse-framework-1*0.html#rfc.section.6.2.1
        pub well_known_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustRiskScoreIntegrationArgs,
    ) -> ZeroTrustRiskScoreIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let active_binding = args.active.get_output(context).get_inner();
        let integration_type_binding = args
            .integration_type
            .get_output(context)
            .get_inner();
        let reference_id_binding = args.reference_id.get_output(context).get_inner();
        let tenant_url_binding = args.tenant_url.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustRiskScoreIntegration:ZeroTrustRiskScoreIntegration"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "integrationType".into(),
                    value: &integration_type_binding,
                },
                register_interface::ObjectField {
                    name: "referenceId".into(),
                    value: &reference_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantUrl".into(),
                    value: &tenant_url_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustRiskScoreIntegrationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            active: pulumi_wasm_rust::__private::into_domain(o.extract_field("active")),
            integration_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("integrationType"),
            ),
            reference_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("referenceId"),
            ),
            tenant_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantUrl"),
            ),
            well_known_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("wellKnownUrl"),
            ),
        }
    }
}
