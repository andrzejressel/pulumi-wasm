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
///     let example = teams_account::create(
///         "example",
///         TeamsAccountArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .antivirus(
///                 TeamsAccountAntivirus::builder()
///                     .enabledDownloadPhase(true)
///                     .enabledUploadPhase(false)
///                     .failClosed(true)
///                     .notificationSettings(
///                         TeamsAccountAntivirusNotificationSettings::builder()
///                             .enabled(true)
///                             .message("you are blocked")
///                             .supportUrl("https://example.com/blocked")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .block_page(
///                 TeamsAccountBlockPage::builder()
///                     .backgroundColor("#000000")
///                     .footerText("hello")
///                     .headerText("hello")
///                     .logoPath("https://example.com/logo.jpg")
///                     .build_struct(),
///             )
///             .body_scanning(
///                 TeamsAccountBodyScanning::builder().inspectionMode("deep").build_struct(),
///             )
///             .extended_email_matching(
///                 TeamsAccountExtendedEmailMatching::builder().enabled(true).build_struct(),
///             )
///             .fips(TeamsAccountFips::builder().tls(true).build_struct())
///             .logging(
///                 TeamsAccountLogging::builder()
///                     .redactPii(true)
///                     .settingsByRuleType(
///                         TeamsAccountLoggingSettingsByRuleType::builder()
///                             .dns(
///                                 TeamsAccountLoggingSettingsByRuleTypeDns::builder()
///                                     .logAll(false)
///                                     .logBlocks(true)
///                                     .build_struct(),
///                             )
///                             .http(
///                                 TeamsAccountLoggingSettingsByRuleTypeHttp::builder()
///                                     .logAll(true)
///                                     .logBlocks(true)
///                                     .build_struct(),
///                             )
///                             .l4(
///                                 TeamsAccountLoggingSettingsByRuleTypeL4::builder()
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
///                 TeamsAccountProxy::builder()
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
/// $ pulumi import cloudflare:index/teamsAccount:TeamsAccount example <account_id>
/// ```
///
pub mod teams_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsAccountArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable the activity log.
        #[builder(into, default)]
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        #[builder(into, default)]
        pub antivirus: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountAntivirus>,
        >,
        /// Configuration for a custom block page.
        #[builder(into, default)]
        pub block_page: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountBlockPage>,
        >,
        /// Configuration for body scanning.
        #[builder(into, default)]
        pub body_scanning: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        #[builder(into, default)]
        pub certificate: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        #[builder(into, default)]
        pub custom_certificate: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        #[builder(into, default)]
        pub extended_email_matching: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountExtendedEmailMatching>,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        #[builder(into, default)]
        pub fips: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountFips>>,
        #[builder(into, default)]
        pub logging: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountLogging>>,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        #[builder(into, default)]
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        #[builder(into, default)]
        pub payload_log: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        #[builder(into, default)]
        pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        #[builder(into, default)]
        pub proxy: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountProxy>>,
        /// Configuration for SSH Session Logging.
        #[builder(into, default)]
        pub ssh_session_log: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountSshSessionLog>,
        >,
        /// Indicator that decryption of TLS traffic is enabled.
        #[builder(into, default)]
        pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
        #[builder(into, default)]
        pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TeamsAccountResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable the activity log.
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for antivirus traffic scanning.
        pub antivirus: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountAntivirus>,
        >,
        /// Configuration for a custom block page.
        pub block_page: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountBlockPage>,
        >,
        /// Configuration for body scanning.
        pub body_scanning: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountBodyScanning>,
        >,
        /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
        pub certificate: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountCertificate>,
        >,
        /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
        pub custom_certificate: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountCustomCertificate>,
        >,
        /// Configuration for extended e-mail matching.
        pub extended_email_matching: pulumi_wasm_rust::Output<
            super::types::TeamsAccountExtendedEmailMatching,
        >,
        /// Configure compliance with Federal Information Processing Standards.
        pub fips: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountFips>>,
        pub logging: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountLogging>>,
        /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Configuration for DLP Payload Logging.
        pub payload_log: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountPayloadLog>,
        >,
        /// Indicator that protocol detection is enabled.
        pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for specifying which protocols are proxied.
        pub proxy: pulumi_wasm_rust::Output<Option<super::types::TeamsAccountProxy>>,
        /// Configuration for SSH Session Logging.
        pub ssh_session_log: pulumi_wasm_rust::Output<
            Option<super::types::TeamsAccountSshSessionLog>,
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
    pub fn create(name: &str, args: TeamsAccountArgs) -> TeamsAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let activity_log_enabled_binding = args.activity_log_enabled.get_inner();
        let antivirus_binding = args.antivirus.get_inner();
        let block_page_binding = args.block_page.get_inner();
        let body_scanning_binding = args.body_scanning.get_inner();
        let certificate_binding = args.certificate.get_inner();
        let custom_certificate_binding = args.custom_certificate.get_inner();
        let extended_email_matching_binding = args.extended_email_matching.get_inner();
        let fips_binding = args.fips.get_inner();
        let logging_binding = args.logging.get_inner();
        let non_identity_browser_isolation_enabled_binding = args
            .non_identity_browser_isolation_enabled
            .get_inner();
        let payload_log_binding = args.payload_log.get_inner();
        let protocol_detection_enabled_binding = args
            .protocol_detection_enabled
            .get_inner();
        let proxy_binding = args.proxy.get_inner();
        let ssh_session_log_binding = args.ssh_session_log.get_inner();
        let tls_decrypt_enabled_binding = args.tls_decrypt_enabled.get_inner();
        let url_browser_isolation_enabled_binding = args
            .url_browser_isolation_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/teamsAccount:TeamsAccount".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "activityLogEnabled".into(),
                },
                register_interface::ResultField {
                    name: "antivirus".into(),
                },
                register_interface::ResultField {
                    name: "blockPage".into(),
                },
                register_interface::ResultField {
                    name: "bodyScanning".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "customCertificate".into(),
                },
                register_interface::ResultField {
                    name: "extendedEmailMatching".into(),
                },
                register_interface::ResultField {
                    name: "fips".into(),
                },
                register_interface::ResultField {
                    name: "logging".into(),
                },
                register_interface::ResultField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "payloadLog".into(),
                },
                register_interface::ResultField {
                    name: "protocolDetectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "proxy".into(),
                },
                register_interface::ResultField {
                    name: "sshSessionLog".into(),
                },
                register_interface::ResultField {
                    name: "tlsDecryptEnabled".into(),
                },
                register_interface::ResultField {
                    name: "urlBrowserIsolationEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TeamsAccountResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            activity_log_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activityLogEnabled").unwrap(),
            ),
            antivirus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("antivirus").unwrap(),
            ),
            block_page: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPage").unwrap(),
            ),
            body_scanning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bodyScanning").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            custom_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customCertificate").unwrap(),
            ),
            extended_email_matching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedEmailMatching").unwrap(),
            ),
            fips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fips").unwrap(),
            ),
            logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logging").unwrap(),
            ),
            non_identity_browser_isolation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonIdentityBrowserIsolationEnabled").unwrap(),
            ),
            payload_log: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("payloadLog").unwrap(),
            ),
            protocol_detection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolDetectionEnabled").unwrap(),
            ),
            proxy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxy").unwrap(),
            ),
            ssh_session_log: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshSessionLog").unwrap(),
            ),
            tls_decrypt_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsDecryptEnabled").unwrap(),
            ),
            url_browser_isolation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlBrowserIsolationEnabled").unwrap(),
            ),
        }
    }
}
