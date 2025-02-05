/// Provides a Cloudflare Teams Account resource. The Teams Account
/// resource defines configuration for secure web gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod zero_trust_gateway_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewaySettingsArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether to enable the activity log.
        #[builder(into, default)]
        pub activity_log_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        #[builder(into, default)]
        pub antivirus: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
        >,
        /// Configuration for a custom block page.
        #[builder(into, default)]
        pub block_page: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
        >,
        /// Configuration for body scanning.
        #[builder(into, default)]
        pub body_scanning: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        #[builder(into, default)]
        pub certificate: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        #[builder(into, default)]
        pub custom_certificate: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        #[builder(into, default)]
        pub extended_email_matching: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsExtendedEmailMatching>,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        #[builder(into, default)]
        pub fips: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsFips>,
        >,
        #[builder(into, default)]
        pub logging: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsLogging>,
        >,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        #[builder(into, default)]
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        #[builder(into, default)]
        pub payload_log: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        #[builder(into, default)]
        pub protocol_detection_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        #[builder(into, default)]
        pub proxy: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsProxy>,
        >,
        /// Configuration for SSH Session Logging.
        #[builder(into, default)]
        pub ssh_session_log: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
        >,
        /// Indicator that decryption of TLS traffic is enabled.
        #[builder(into, default)]
        pub tls_decrypt_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
        #[builder(into, default)]
        pub url_browser_isolation_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewaySettingsResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable the activity log.
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        pub antivirus: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsAntivirus>,
        >,
        /// Configuration for a custom block page.
        pub block_page: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsBlockPage>,
        >,
        /// Configuration for body scanning.
        pub body_scanning: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        pub certificate: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        pub custom_certificate: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        pub extended_email_matching: pulumi_wasm_rust::Output<
            super::types::ZeroTrustGatewaySettingsExtendedEmailMatching,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        pub fips: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsFips>,
        >,
        pub logging: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsLogging>,
        >,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        pub payload_log: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        pub proxy: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsProxy>,
        >,
        /// Configuration for SSH Session Logging.
        pub ssh_session_log: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustGatewaySettingsSshSessionLog>,
        >,
        /// Indicator that decryption of TLS traffic is enabled.
        pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
        pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustGatewaySettingsArgs,
    ) -> ZeroTrustGatewaySettingsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let activity_log_enabled_binding = args
            .activity_log_enabled
            .get_output(context)
            .get_inner();
        let antivirus_binding = args.antivirus.get_output(context).get_inner();
        let block_page_binding = args.block_page.get_output(context).get_inner();
        let body_scanning_binding = args.body_scanning.get_output(context).get_inner();
        let certificate_binding = args.certificate.get_output(context).get_inner();
        let custom_certificate_binding = args
            .custom_certificate
            .get_output(context)
            .get_inner();
        let extended_email_matching_binding = args
            .extended_email_matching
            .get_output(context)
            .get_inner();
        let fips_binding = args.fips.get_output(context).get_inner();
        let logging_binding = args.logging.get_output(context).get_inner();
        let non_identity_browser_isolation_enabled_binding = args
            .non_identity_browser_isolation_enabled
            .get_output(context)
            .get_inner();
        let payload_log_binding = args.payload_log.get_output(context).get_inner();
        let protocol_detection_enabled_binding = args
            .protocol_detection_enabled
            .get_output(context)
            .get_inner();
        let proxy_binding = args.proxy.get_output(context).get_inner();
        let ssh_session_log_binding = args
            .ssh_session_log
            .get_output(context)
            .get_inner();
        let tls_decrypt_enabled_binding = args
            .tls_decrypt_enabled
            .get_output(context)
            .get_inner();
        let url_browser_isolation_enabled_binding = args
            .url_browser_isolation_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "activityLogEnabled".into(),
                    value: &activity_log_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "antivirus".into(),
                    value: &antivirus_binding,
                },
                register_interface::ObjectField {
                    name: "blockPage".into(),
                    value: &block_page_binding,
                },
                register_interface::ObjectField {
                    name: "bodyScanning".into(),
                    value: &body_scanning_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "customCertificate".into(),
                    value: &custom_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "extendedEmailMatching".into(),
                    value: &extended_email_matching_binding,
                },
                register_interface::ObjectField {
                    name: "fips".into(),
                    value: &fips_binding,
                },
                register_interface::ObjectField {
                    name: "logging".into(),
                    value: &logging_binding,
                },
                register_interface::ObjectField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                    value: &non_identity_browser_isolation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "payloadLog".into(),
                    value: &payload_log_binding,
                },
                register_interface::ObjectField {
                    name: "protocolDetectionEnabled".into(),
                    value: &protocol_detection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "proxy".into(),
                    value: &proxy_binding,
                },
                register_interface::ObjectField {
                    name: "sshSessionLog".into(),
                    value: &ssh_session_log_binding,
                },
                register_interface::ObjectField {
                    name: "tlsDecryptEnabled".into(),
                    value: &tls_decrypt_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "urlBrowserIsolationEnabled".into(),
                    value: &url_browser_isolation_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustGatewaySettingsResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            activity_log_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activityLogEnabled"),
            ),
            antivirus: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("antivirus"),
            ),
            block_page: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blockPage"),
            ),
            body_scanning: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bodyScanning"),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            custom_certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customCertificate"),
            ),
            extended_email_matching: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("extendedEmailMatching"),
            ),
            fips: pulumi_wasm_rust::__private::into_domain(o.extract_field("fips")),
            logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logging"),
            ),
            non_identity_browser_isolation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nonIdentityBrowserIsolationEnabled"),
            ),
            payload_log: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("payloadLog"),
            ),
            protocol_detection_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocolDetectionEnabled"),
            ),
            proxy: pulumi_wasm_rust::__private::into_domain(o.extract_field("proxy")),
            ssh_session_log: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sshSessionLog"),
            ),
            tls_decrypt_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tlsDecryptEnabled"),
            ),
            url_browser_isolation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("urlBrowserIsolationEnabled"),
            ),
        }
    }
}
