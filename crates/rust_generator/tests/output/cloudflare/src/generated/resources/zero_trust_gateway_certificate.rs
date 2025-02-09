/// Provides a Cloudflare Teams Gateway Certificate resource. A Teams Certificate can
/// be specified for Gateway TLS interception and block pages.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_gateway_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayCertificateArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
        #[builder(into, default)]
        pub activate: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
        #[builder(into, default)]
        pub custom: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
        #[builder(into, default)]
        pub gateway_managed: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub validity_period_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayCertificateResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
        pub activate: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The deployment status of the certificate on the edge Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
        pub binding_status: pulumi_gestalt_rust::Output<String>,
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
        pub custom: pulumi_gestalt_rust::Output<Option<bool>>,
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
        pub gateway_managed: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the certificate is in use by Gateway for TLS interception and the block page.
        pub in_use: pulumi_gestalt_rust::Output<bool>,
        pub qs_pack_id: pulumi_gestalt_rust::Output<String>,
        pub uploaded_on: pulumi_gestalt_rust::Output<String>,
        /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
        pub validity_period_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustGatewayCertificateArgs,
    ) -> ZeroTrustGatewayCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let activate_binding = args.activate.get_output(context);
        let custom_binding = args.custom.get_output(context);
        let gateway_managed_binding = args.gateway_managed.get_output(context);
        let validity_period_days_binding = args.validity_period_days.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayCertificate:ZeroTrustGatewayCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activate".into(),
                    value: activate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "custom".into(),
                    value: custom_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayManaged".into(),
                    value: gateway_managed_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validityPeriodDays".into(),
                    value: validity_period_days_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustGatewayCertificateResult {
            account_id: o.get_field("accountId"),
            activate: o.get_field("activate"),
            binding_status: o.get_field("bindingStatus"),
            created_at: o.get_field("createdAt"),
            custom: o.get_field("custom"),
            expires_on: o.get_field("expiresOn"),
            gateway_managed: o.get_field("gatewayManaged"),
            in_use: o.get_field("inUse"),
            qs_pack_id: o.get_field("qsPackId"),
            uploaded_on: o.get_field("uploadedOn"),
            validity_period_days: o.get_field("validityPeriodDays"),
        }
    }
}
