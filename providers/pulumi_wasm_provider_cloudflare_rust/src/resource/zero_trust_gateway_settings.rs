//! Provides a Cloudflare Teams Account resource. The Teams Account
//! resource defines configuration for secure web gateway.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ZeroTrustGatewaySettings("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     tlsDecryptEnabled: true,
//!     protocolDetectionEnabled: true,
//!     blockPage: {
//!         footerText: "hello",
//!         headerText: "hello",
//!         logoPath: "https://example.com/logo.jpg",
//!         backgroundColor: "#000000",
//!     },
//!     bodyScanning: {
//!         inspectionMode: "deep",
//!     },
//!     antivirus: {
//!         enabledDownloadPhase: true,
//!         enabledUploadPhase: false,
//!         failClosed: true,
//!         notificationSettings: {
//!             enabled: true,
//!             message: "you are blocked",
//!             supportUrl: "https://example.com/blocked",
//!         },
//!     },
//!     fips: {
//!         tls: true,
//!     },
//!     proxy: {
//!         tcp: true,
//!         udp: true,
//!         rootCa: true,
//!         virtualIp: false,
//!         disableForTime: 3600,
//!     },
//!     urlBrowserIsolationEnabled: true,
//!     logging: {
//!         redactPii: true,
//!         settingsByRuleType: {
//!             dns: {
//!                 logAll: false,
//!                 logBlocks: true,
//!             },
//!             http: {
//!                 logAll: true,
//!                 logBlocks: true,
//!             },
//!             l4: {
//!                 logAll: false,
//!                 logBlocks: true,
//!             },
//!         },
//!     },
//!     extendedEmailMatching: {
//!         enabled: true,
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ZeroTrustGatewaySettings("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     tls_decrypt_enabled=True,
//!     protocol_detection_enabled=True,
//!     block_page={
//!         "footer_text": "hello",
//!         "header_text": "hello",
//!         "logo_path": "https://example.com/logo.jpg",
//!         "background_color": "#000000",
//!     },
//!     body_scanning={
//!         "inspection_mode": "deep",
//!     },
//!     antivirus={
//!         "enabled_download_phase": True,
//!         "enabled_upload_phase": False,
//!         "fail_closed": True,
//!         "notification_settings": {
//!             "enabled": True,
//!             "message": "you are blocked",
//!             "support_url": "https://example.com/blocked",
//!         },
//!     },
//!     fips={
//!         "tls": True,
//!     },
//!     proxy={
//!         "tcp": True,
//!         "udp": True,
//!         "root_ca": True,
//!         "virtual_ip": False,
//!         "disable_for_time": 3600,
//!     },
//!     url_browser_isolation_enabled=True,
//!     logging={
//!         "redact_pii": True,
//!         "settings_by_rule_type": {
//!             "dns": {
//!                 "log_all": False,
//!                 "log_blocks": True,
//!             },
//!             "http": {
//!                 "log_all": True,
//!                 "log_blocks": True,
//!             },
//!             "l4": {
//!                 "log_all": False,
//!                 "log_blocks": True,
//!             },
//!         },
//!     },
//!     extended_email_matching={
//!         "enabled": True,
//!     })
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var example = new Cloudflare.ZeroTrustGatewaySettings("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         TlsDecryptEnabled = true,
//!         ProtocolDetectionEnabled = true,
//!         BlockPage = new Cloudflare.Inputs.ZeroTrustGatewaySettingsBlockPageArgs
//!         {
//!             FooterText = "hello",
//!             HeaderText = "hello",
//!             LogoPath = "https://example.com/logo.jpg",
//!             BackgroundColor = "#000000",
//!         },
//!         BodyScanning = new Cloudflare.Inputs.ZeroTrustGatewaySettingsBodyScanningArgs
//!         {
//!             InspectionMode = "deep",
//!         },
//!         Antivirus = new Cloudflare.Inputs.ZeroTrustGatewaySettingsAntivirusArgs
//!         {
//!             EnabledDownloadPhase = true,
//!             EnabledUploadPhase = false,
//!             FailClosed = true,
//!             NotificationSettings = new Cloudflare.Inputs.ZeroTrustGatewaySettingsAntivirusNotificationSettingsArgs
//!             {
//!                 Enabled = true,
//!                 Message = "you are blocked",
//!                 SupportUrl = "https://example.com/blocked",
//!             },
//!         },
//!         Fips = new Cloudflare.Inputs.ZeroTrustGatewaySettingsFipsArgs
//!         {
//!             Tls = true,
//!         },
//!         Proxy = new Cloudflare.Inputs.ZeroTrustGatewaySettingsProxyArgs
//!         {
//!             Tcp = true,
//!             Udp = true,
//!             RootCa = true,
//!             VirtualIp = false,
//!             DisableForTime = 3600,
//!         },
//!         UrlBrowserIsolationEnabled = true,
//!         Logging = new Cloudflare.Inputs.ZeroTrustGatewaySettingsLoggingArgs
//!         {
//!             RedactPii = true,
//!             SettingsByRuleType = new Cloudflare.Inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeArgs
//!             {
//!                 Dns = new Cloudflare.Inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDnsArgs
//!                 {
//!                     LogAll = false,
//!                     LogBlocks = true,
//!                 },
//!                 Http = new Cloudflare.Inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttpArgs
//!                 {
//!                     LogAll = true,
//!                     LogBlocks = true,
//!                 },
//!                 L4 = new Cloudflare.Inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4Args
//!                 {
//!                     LogAll = false,
//!                     LogBlocks = true,
//!                 },
//!             },
//!         },
//!         ExtendedEmailMatching = new Cloudflare.Inputs.ZeroTrustGatewaySettingsExtendedEmailMatchingArgs
//!         {
//!             Enabled = true,
//!         },
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewZeroTrustGatewaySettings(ctx, "example", &cloudflare.ZeroTrustGatewaySettingsArgs{
//! 			AccountId:                pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			TlsDecryptEnabled:        pulumi.Bool(true),
//! 			ProtocolDetectionEnabled: pulumi.Bool(true),
//! 			BlockPage: &cloudflare.ZeroTrustGatewaySettingsBlockPageArgs{
//! 				FooterText:      pulumi.String("hello"),
//! 				HeaderText:      pulumi.String("hello"),
//! 				LogoPath:        pulumi.String("https://example.com/logo.jpg"),
//! 				BackgroundColor: pulumi.String("#000000"),
//! 			},
//! 			BodyScanning: &cloudflare.ZeroTrustGatewaySettingsBodyScanningArgs{
//! 				InspectionMode: pulumi.String("deep"),
//! 			},
//! 			Antivirus: &cloudflare.ZeroTrustGatewaySettingsAntivirusArgs{
//! 				EnabledDownloadPhase: pulumi.Bool(true),
//! 				EnabledUploadPhase:   pulumi.Bool(false),
//! 				FailClosed:           pulumi.Bool(true),
//! 				NotificationSettings: &cloudflare.ZeroTrustGatewaySettingsAntivirusNotificationSettingsArgs{
//! 					Enabled:    pulumi.Bool(true),
//! 					Message:    pulumi.String("you are blocked"),
//! 					SupportUrl: pulumi.String("https://example.com/blocked"),
//! 				},
//! 			},
//! 			Fips: &cloudflare.ZeroTrustGatewaySettingsFipsArgs{
//! 				Tls: pulumi.Bool(true),
//! 			},
//! 			Proxy: &cloudflare.ZeroTrustGatewaySettingsProxyArgs{
//! 				Tcp:            pulumi.Bool(true),
//! 				Udp:            pulumi.Bool(true),
//! 				RootCa:         pulumi.Bool(true),
//! 				VirtualIp:      pulumi.Bool(false),
//! 				DisableForTime: pulumi.Int(3600),
//! 			},
//! 			UrlBrowserIsolationEnabled: pulumi.Bool(true),
//! 			Logging: &cloudflare.ZeroTrustGatewaySettingsLoggingArgs{
//! 				RedactPii: pulumi.Bool(true),
//! 				SettingsByRuleType: &cloudflare.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeArgs{
//! 					Dns: &cloudflare.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDnsArgs{
//! 						LogAll:    pulumi.Bool(false),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 					Http: &cloudflare.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttpArgs{
//! 						LogAll:    pulumi.Bool(true),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 					L4: &cloudflare.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4Args{
//! 						LogAll:    pulumi.Bool(false),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 				},
//! 			},
//! 			ExtendedEmailMatching: &cloudflare.ZeroTrustGatewaySettingsExtendedEmailMatchingArgs{
//! 				Enabled: pulumi.Bool(true),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.ZeroTrustGatewaySettings;
//! import com.pulumi.cloudflare.ZeroTrustGatewaySettingsArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsBlockPageArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsBodyScanningArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsAntivirusArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsAntivirusNotificationSettingsArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsFipsArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsProxyArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsLoggingArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDnsArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttpArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4Args;
//! import com.pulumi.cloudflare.inputs.ZeroTrustGatewaySettingsExtendedEmailMatchingArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var example = new ZeroTrustGatewaySettings("example", ZeroTrustGatewaySettingsArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .tlsDecryptEnabled(true)
//!             .protocolDetectionEnabled(true)
//!             .blockPage(ZeroTrustGatewaySettingsBlockPageArgs.builder()
//!                 .footerText("hello")
//!                 .headerText("hello")
//!                 .logoPath("https://example.com/logo.jpg")
//!                 .backgroundColor("#000000")
//!                 .build())
//!             .bodyScanning(ZeroTrustGatewaySettingsBodyScanningArgs.builder()
//!                 .inspectionMode("deep")
//!                 .build())
//!             .antivirus(ZeroTrustGatewaySettingsAntivirusArgs.builder()
//!                 .enabledDownloadPhase(true)
//!                 .enabledUploadPhase(false)
//!                 .failClosed(true)
//!                 .notificationSettings(ZeroTrustGatewaySettingsAntivirusNotificationSettingsArgs.builder()
//!                     .enabled(true)
//!                     .message("you are blocked")
//!                     .supportUrl("https://example.com/blocked")
//!                     .build())
//!                 .build())
//!             .fips(ZeroTrustGatewaySettingsFipsArgs.builder()
//!                 .tls(true)
//!                 .build())
//!             .proxy(ZeroTrustGatewaySettingsProxyArgs.builder()
//!                 .tcp(true)
//!                 .udp(true)
//!                 .rootCa(true)
//!                 .virtualIp(false)
//!                 .disableForTime(3600)
//!                 .build())
//!             .urlBrowserIsolationEnabled(true)
//!             .logging(ZeroTrustGatewaySettingsLoggingArgs.builder()
//!                 .redactPii(true)
//!                 .settingsByRuleType(ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeArgs.builder()
//!                     .dns(ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeDnsArgs.builder()
//!                         .logAll(false)
//!                         .logBlocks(true)
//!                         .build())
//!                     .http(ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeHttpArgs.builder()
//!                         .logAll(true)
//!                         .logBlocks(true)
//!                         .build())
//!                     .l4(ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4Args.builder()
//!                         .logAll(false)
//!                         .logBlocks(true)
//!                         .build())
//!                     .build())
//!                 .build())
//!             .extendedEmailMatching(ZeroTrustGatewaySettingsExtendedEmailMatchingArgs.builder()
//!                 .enabled(true)
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ZeroTrustGatewaySettings
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       tlsDecryptEnabled: true
//!       protocolDetectionEnabled: true
//!       blockPage:
//!         footerText: hello
//!         headerText: hello
//!         logoPath: https://example.com/logo.jpg
//!         backgroundColor: '#000000'
//!       bodyScanning:
//!         inspectionMode: deep
//!       antivirus:
//!         enabledDownloadPhase: true
//!         enabledUploadPhase: false
//!         failClosed: true
//!         notificationSettings:
//!           enabled: true
//!           message: you are blocked
//!           supportUrl: https://example.com/blocked
//!       fips:
//!         tls: true
//!       proxy:
//!         tcp: true
//!         udp: true
//!         rootCa: true
//!         virtualIp: false
//!         disableForTime: 3600
//!       urlBrowserIsolationEnabled: true
//!       logging:
//!         redactPii: true
//!         settingsByRuleType:
//!           dns:
//!             logAll: false
//!             logBlocks: true
//!           http:
//!             logAll: true
//!             logBlocks: true
//!           l4:
//!             logAll: false
//!             logBlocks: true
//!       extendedEmailMatching:
//!         enabled: true
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustGatewaySettings:ZeroTrustGatewaySettings example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to enable the activity log.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for antivirus traffic scanning.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsAntivirus>>,
    /// Configuration for a custom block page.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub block_page: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBlockPage>>,
    /// Configuration for body scanning.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsBodyScanning>>,
    /// Configuration for TLS interception certificate. This will be required starting Feb 2025.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCertificate>>,
    /// Configuration for custom certificates / BYO-PKI. Conflicts with `certificate`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_certificate: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsCustomCertificate>>,
    /// Configuration for extended e-mail matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub extended_email_matching: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsExtendedEmailMatching>>,
    /// Configure compliance with Federal Information Processing Standards.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub fips: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsFips>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logging: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsLogging>>,
    /// Enable non-identity onramp for Browser Isolation. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration for DLP Payload Logging.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsPayloadLog>>,
    /// Indicator that protocol detection is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configuration block for specifying which protocols are proxied.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub proxy: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsProxy>>,
    /// Configuration for SSH Session Logging.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ssh_session_log: pulumi_wasm_rust::Output<Option<crate::types::ZeroTrustGatewaySettingsSshSessionLog>>,
    /// Indicator that decryption of TLS traffic is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Safely browse websites in Browser Isolation through a URL. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
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