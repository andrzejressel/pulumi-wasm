//! Provides a Cloudflare Teams Account resource. The Teams Account
//! resource defines configuration for secure web gateway.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = teams_account::create(
//!         "example",
//!         TeamsAccountArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .antivirus(
//!                 TeamsAccountAntivirus::builder()
//!                     .enabledDownloadPhase(true)
//!                     .enabledUploadPhase(false)
//!                     .failClosed(true)
//!                     .notificationSettings(
//!                         TeamsAccountAntivirusNotificationSettings::builder()
//!                             .enabled(true)
//!                             .message("you are blocked")
//!                             .supportUrl("https://example.com/blocked")
//!                             .build_struct(),
//!                     )
//!                     .build_struct(),
//!             )
//!             .block_page(
//!                 TeamsAccountBlockPage::builder()
//!                     .backgroundColor("#000000")
//!                     .footerText("hello")
//!                     .headerText("hello")
//!                     .logoPath("https://example.com/logo.jpg")
//!                     .build_struct(),
//!             )
//!             .body_scanning(
//!                 TeamsAccountBodyScanning::builder().inspectionMode("deep").build_struct(),
//!             )
//!             .extended_email_matching(
//!                 TeamsAccountExtendedEmailMatching::builder().enabled(true).build_struct(),
//!             )
//!             .fips(TeamsAccountFips::builder().tls(true).build_struct())
//!             .logging(
//!                 TeamsAccountLogging::builder()
//!                     .redactPii(true)
//!                     .settingsByRuleType(
//!                         TeamsAccountLoggingSettingsByRuleType::builder()
//!                             .dns(
//!                                 TeamsAccountLoggingSettingsByRuleTypeDns::builder()
//!                                     .logAll(false)
//!                                     .logBlocks(true)
//!                                     .build_struct(),
//!                             )
//!                             .http(
//!                                 TeamsAccountLoggingSettingsByRuleTypeHttp::builder()
//!                                     .logAll(true)
//!                                     .logBlocks(true)
//!                                     .build_struct(),
//!                             )
//!                             .l4(
//!                                 TeamsAccountLoggingSettingsByRuleTypeL4::builder()
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
//!                 TeamsAccountProxy::builder()
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
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsAccount:TeamsAccount example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
    /// Configuration for a custom block page.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
    /// Configuration for body scanning.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub certificate: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountCertificate>>,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_certificate: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountCustomCertificate>>,
    /// Configuration for extended e-mail matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub extended_email_matching: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
    /// Configure compliance with Federal Information Processing Standards.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
    /// Indicator that protocol detection is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
    /// Configuration for SSH Session Logging.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
    /// Indicator that decryption of TLS traffic is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct TeamsAccountResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
    /// Configuration for a custom block page.
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
    /// Configuration for body scanning.
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    pub certificate: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountCertificate>>,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    pub custom_certificate: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountCustomCertificate>>,
    /// Configuration for extended e-mail matching.
    pub extended_email_matching: pulumi_wasm_rust::Output<crate::types::TeamsAccountExtendedEmailMatching>,
    /// Configure compliance with Federal Information Processing Standards.
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
    /// Indicator that protocol detection is enabled.
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
    /// Configuration for SSH Session Logging.
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
    /// Indicator that decryption of TLS traffic is enabled.
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsAccountArgs) -> TeamsAccountResult {

    let result = crate::bindings::pulumi::cloudflare::teams_account::invoke(name, &crate::bindings::pulumi::cloudflare::teams_account::Args {
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

    TeamsAccountResult {
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
