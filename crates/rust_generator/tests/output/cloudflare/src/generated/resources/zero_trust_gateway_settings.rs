/// Provides a Cloudflare Teams Account resource. The Teams Account
/// resource defines configuration for secure web gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_gateway_settings::create(
///         "example",
///         ZeroTrustGatewaySettingsArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .antivirus(
///                 ZeroTrustGatewaySettingsAntivirus::builder()
///                     .enabledDownloadPhase(true)
///                     .enabledUploadPhase(false)
///                     .failClosed(true)
///                     .notificationSettings(
///                         ZeroTrustGatewaySettingsAntivirusNotificationSettings::builder()
///                             .enabled(true)
///                             .message("you are blocked")
///                             .supportUrl("https://example.com/blocked")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .block_page(
///                 ZeroTrustGatewaySettingsBlockPage::builder()
///                     .backgroundColor("#000000")
///                     .footerText("hello")
///                     .headerText("hello")
///                     .logoPath("https://example.com/logo.jpg")
///                     .build_struct(),
///             )
///             .body_scanning(
///                 ZeroTrustGatewaySettingsBodyScanning::builder()
///                     .inspectionMode("deep")
///                     .build_struct(),
///             )
///             .extended_email_matching(
///                 ZeroTrustGatewaySettingsExtendedEmailMatching::builder()
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .fips(ZeroTrustGatewaySettingsFips::builder().tls(true).build_struct())
///             .logging(
///                 ZeroTrustGatewaySettingsLogging::builder()
///                     .redactPii(true)
///                     .settingsByRuleType(
///                         ZeroTrustGatewaySettingsLoggingSettingsByRuleType::builder()
///                             .dns(
///                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDns::builder()
///                                     .logAll(false)
///                                     .logBlocks(true)
///                                     .build_struct(),
///                             )
///                             .http(
///                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttp::builder()
///                                     .logAll(true)
///                                     .logBlocks(true)
///                                     .build_struct(),
///                             )
///                             .l4(
///                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4::builder()
///                                     .logAll(false)
///                                     .logBlocks(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .protocol_detection_enabled(true)
///             .proxy(
///                 ZeroTrustGatewaySettingsProxy::builder()
///                     .disableForTime(3600)
///                     .rootCa(true)
///                     .tcp(true)
///                     .udp(true)
///                     .virtualIp(false)
///                     .build_struct(),
///             )
///             .tls_decrypt_enabled(true)
///             .url_browser_isolation_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings example <account_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_gateway_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewaySettingsArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable the activity log.
        #[builder(into, default)]
        pub activity_log_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        #[builder(into, default)]
        pub antivirus: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
        >,
        /// Configuration for a custom block page.
        #[builder(into, default)]
        pub block_page: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
        >,
        /// Configuration for body scanning.
        #[builder(into, default)]
        pub body_scanning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        #[builder(into, default)]
        pub custom_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        #[builder(into, default)]
        pub extended_email_matching: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsExtendedEmailMatching>,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        #[builder(into, default)]
        pub fips: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsFips>,
        >,
        #[builder(into, default)]
        pub logging: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsLogging>,
        >,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        #[builder(into, default)]
        pub non_identity_browser_isolation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        #[builder(into, default)]
        pub payload_log: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        #[builder(into, default)]
        pub protocol_detection_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        #[builder(into, default)]
        pub proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsProxy>,
        >,
        /// Configuration for SSH Session Logging.
        #[builder(into, default)]
        pub ssh_session_log: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
        >,
        /// Indicator that decryption of TLS traffic is enabled.
        #[builder(into, default)]
        pub tls_decrypt_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
        #[builder(into, default)]
        pub url_browser_isolation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewaySettingsResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the activity log.
        pub activity_log_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        pub antivirus: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
        >,
        /// Configuration for a custom block page.
        pub block_page: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
        >,
        /// Configuration for body scanning.
        pub body_scanning: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        pub certificate: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        pub custom_certificate: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        pub extended_email_matching: pulumi_gestalt_rust::Output<
            super::types::ZeroTrustGatewaySettingsExtendedEmailMatching,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        pub fips: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsFips>,
        >,
        pub logging: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsLogging>,
        >,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        pub non_identity_browser_isolation_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        pub payload_log: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        pub protocol_detection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        pub proxy: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsProxy>,
        >,
        /// Configuration for SSH Session Logging.
        pub ssh_session_log: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
        >,
        /// Indicator that decryption of TLS traffic is enabled.
        pub tls_decrypt_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
        pub url_browser_isolation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustGatewaySettingsArgs,
    ) -> ZeroTrustGatewaySettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let activity_log_enabled_binding = args.activity_log_enabled.get_output(context);
        let antivirus_binding = args.antivirus.get_output(context);
        let block_page_binding = args.block_page.get_output(context);
        let body_scanning_binding = args.body_scanning.get_output(context);
        let certificate_binding = args.certificate.get_output(context);
        let custom_certificate_binding = args.custom_certificate.get_output(context);
        let extended_email_matching_binding = args
            .extended_email_matching
            .get_output(context);
        let fips_binding = args.fips.get_output(context);
        let logging_binding = args.logging.get_output(context);
        let non_identity_browser_isolation_enabled_binding = args
            .non_identity_browser_isolation_enabled
            .get_output(context);
        let payload_log_binding = args.payload_log.get_output(context);
        let protocol_detection_enabled_binding = args
            .protocol_detection_enabled
            .get_output(context);
        let proxy_binding = args.proxy.get_output(context);
        let ssh_session_log_binding = args.ssh_session_log.get_output(context);
        let tls_decrypt_enabled_binding = args.tls_decrypt_enabled.get_output(context);
        let url_browser_isolation_enabled_binding = args
            .url_browser_isolation_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activityLogEnabled".into(),
                    value: activity_log_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "antivirus".into(),
                    value: antivirus_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPage".into(),
                    value: block_page_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bodyScanning".into(),
                    value: body_scanning_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customCertificate".into(),
                    value: custom_certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extendedEmailMatching".into(),
                    value: extended_email_matching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fips".into(),
                    value: fips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logging".into(),
                    value: logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                    value: non_identity_browser_isolation_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "payloadLog".into(),
                    value: payload_log_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocolDetectionEnabled".into(),
                    value: protocol_detection_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxy".into(),
                    value: proxy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshSessionLog".into(),
                    value: ssh_session_log_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsDecryptEnabled".into(),
                    value: tls_decrypt_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlBrowserIsolationEnabled".into(),
                    value: url_browser_isolation_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustGatewaySettingsResult {
            account_id: o.get_field("accountId"),
            activity_log_enabled: o.get_field("activityLogEnabled"),
            antivirus: o.get_field("antivirus"),
            block_page: o.get_field("blockPage"),
            body_scanning: o.get_field("bodyScanning"),
            certificate: o.get_field("certificate"),
            custom_certificate: o.get_field("customCertificate"),
            extended_email_matching: o.get_field("extendedEmailMatching"),
            fips: o.get_field("fips"),
            logging: o.get_field("logging"),
            non_identity_browser_isolation_enabled: o
                .get_field("nonIdentityBrowserIsolationEnabled"),
            payload_log: o.get_field("payloadLog"),
            protocol_detection_enabled: o.get_field("protocolDetectionEnabled"),
            proxy: o.get_field("proxy"),
            ssh_session_log: o.get_field("sshSessionLog"),
            tls_decrypt_enabled: o.get_field("tlsDecryptEnabled"),
            url_browser_isolation_enabled: o.get_field("urlBrowserIsolationEnabled"),
        }
    }
}
