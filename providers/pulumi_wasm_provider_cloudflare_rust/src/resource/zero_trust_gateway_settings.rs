//! Provides a Cloudflare Teams Account resource. The Teams Account
//! resource defines configuration for secure web gateway.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zero_trust_gateway_settings::create(
//!         "example",
//!         ZeroTrustGatewaySettingsArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .antivirus(
//!                 ZeroTrustGatewaySettingsAntivirus::builder()
//!                     .enabledDownloadPhase(true)
//!                     .enabledUploadPhase(false)
//!                     .failClosed(true)
//!                     .notificationSettings(
//!                         ZeroTrustGatewaySettingsAntivirusNotificationSettings::builder()
//!                             .enabled(true)
//!                             .message("you are blocked")
//!                             .supportUrl("https://example.com/blocked")
//!                             .build_struct(),
//!                     )
//!                     .build_struct(),
//!             )
//!             .block_page(
//!                 ZeroTrustGatewaySettingsBlockPage::builder()
//!                     .backgroundColor("#000000")
//!                     .footerText("hello")
//!                     .headerText("hello")
//!                     .logoPath("https://example.com/logo.jpg")
//!                     .build_struct(),
//!             )
//!             .body_scanning(
//!                 ZeroTrustGatewaySettingsBodyScanning::builder()
//!                     .inspectionMode("deep")
//!                     .build_struct(),
//!             )
//!             .extended_email_matching(
//!                 ZeroTrustGatewaySettingsExtendedEmailMatching::builder()
//!                     .enabled(true)
//!                     .build_struct(),
//!             )
//!             .fips(ZeroTrustGatewaySettingsFips::builder().tls(true).build_struct())
//!             .logging(
//!                 ZeroTrustGatewaySettingsLogging::builder()
//!                     .redactPii(true)
//!                     .settingsByRuleType(
//!                         ZeroTrustGatewaySettingsLoggingSettingsByRuleType::builder()
//!                             .dns(
//!                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDns::builder()
//!                                     .logAll(false)
//!                                     .logBlocks(true)
//!                                     .build_struct(),
//!                             )
//!                             .http(
//!                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttp::builder()
//!                                     .logAll(true)
//!                                     .logBlocks(true)
//!                                     .build_struct(),
//!                             )
//!                             .l4(
//!                                 ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4::builder()
//!                                     .logAll(false)
//!                                     .logBlocks(true)
//!                                     .build_struct(),
//!                             )
//!                             .build_struct(),
//!                     )
//!                     .build_struct(),
//!             )
//!             .protocol_detection_enabled(true)
//!             .proxy(
//!                 ZeroTrustGatewaySettingsProxy::builder()
//!                     .disableForTime(3600)
//!                     .rootCa(true)
//!                     .tcp(true)
//!                     .udp(true)
//!                     .virtualIp(false)
//!                     .build_struct(),
//!             )
//!             .tls_decrypt_enabled(true)
//!             .url_browser_isolation_enabled(true)
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings example <account_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    #[builder(into, default)]
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    #[builder(into, default)]
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsAntivirus>>,
    /// Configuration for a custom block page.
    #[builder(into, default)]
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBlockPage>>,
    /// Configuration for body scanning.
    #[builder(into, default)]
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBodyScanning>>,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    #[builder(into, default)]
    pub certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCertificate>>,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    #[builder(into, default)]
    pub custom_certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCustomCertificate>>,
    /// Configuration for extended e-mail matching.
    #[builder(into, default)]
    pub extended_email_matching: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsExtendedEmailMatching>>,
    /// Configure compliance with Federal Information Processing Standards.
    #[builder(into, default)]
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsFips>>,
    #[builder(into, default)]
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsLogging>>,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    #[builder(into, default)]
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    #[builder(into, default)]
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsPayloadLog>>,
    /// Indicator that protocol detection is enabled.
    #[builder(into, default)]
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    #[builder(into, default)]
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsProxy>>,
    /// Configuration for SSH Session Logging.
    #[builder(into, default)]
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsSshSessionLog>>,
    /// Indicator that decryption of TLS traffic is enabled.
    #[builder(into, default)]
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    #[builder(into, default)]
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct ZeroTrustGatewaySettingsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsAntivirus>>,
    /// Configuration for a custom block page.
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBlockPage>>,
    /// Configuration for body scanning.
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBodyScanning>>,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    pub certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCertificate>>,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    pub custom_certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCustomCertificate>>,
    /// Configuration for extended e-mail matching.
    pub extended_email_matching: pulumi_wasm_rust::Output<crate::types::ZeroTrustGatewaySettingsExtendedEmailMatching>,
    /// Configure compliance with Federal Information Processing Standards.
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsFips>>,
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsLogging>>,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsPayloadLog>>,
    /// Indicator that protocol detection is enabled.
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsProxy>>,
    /// Configuration for SSH Session Logging.
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsSshSessionLog>>,
    /// Indicator that decryption of TLS traffic is enabled.
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustGatewaySettingsArgs) -> ZeroTrustGatewaySettingsResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_gateway_settings::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_gateway_settings::Args {
        account_id: &args.account_id.get_inner(),
        activity_log_enabled: &args.activity_log_enabled.get_inner(),
        antivirus: &args.antivirus.get_inner(),
        block_page: &args.block_page.get_inner(),
        body_scanning: &args.body_scanning.get_inner(),
        certificate: &args.certificate.get_inner(),
        custom_certificate: &args.custom_certificate.get_inner(),
        extended_email_matching: &args.extended_email_matching.get_inner(),
        fips: &args.fips.get_inner(),
        logging: &args.logging.get_inner(),
        non_identity_browser_isolation_enabled: &args.non_identity_browser_isolation_enabled.get_inner(),
        payload_log: &args.payload_log.get_inner(),
        protocol_detection_enabled: &args.protocol_detection_enabled.get_inner(),
        proxy: &args.proxy.get_inner(),
        ssh_session_log: &args.ssh_session_log.get_inner(),
        tls_decrypt_enabled: &args.tls_decrypt_enabled.get_inner(),
        url_browser_isolation_enabled: &args.url_browser_isolation_enabled.get_inner(),
    });

    ZeroTrustGatewaySettingsResult {
        account_id: crate::into_domain(result.account_id),
        activity_log_enabled: crate::into_domain(result.activity_log_enabled),
        antivirus: crate::into_domain(result.antivirus),
        block_page: crate::into_domain(result.block_page),
        body_scanning: crate::into_domain(result.body_scanning),
        certificate: crate::into_domain(result.certificate),
        custom_certificate: crate::into_domain(result.custom_certificate),
        extended_email_matching: crate::into_domain(result.extended_email_matching),
        fips: crate::into_domain(result.fips),
        logging: crate::into_domain(result.logging),
        non_identity_browser_isolation_enabled: crate::into_domain(result.non_identity_browser_isolation_enabled),
        payload_log: crate::into_domain(result.payload_log),
        protocol_detection_enabled: crate::into_domain(result.protocol_detection_enabled),
        proxy: crate::into_domain(result.proxy),
        ssh_session_log: crate::into_domain(result.ssh_session_log),
        tls_decrypt_enabled: crate::into_domain(result.tls_decrypt_enabled),
        url_browser_isolation_enabled: crate::into_domain(result.url_browser_isolation_enabled),
    }
}
