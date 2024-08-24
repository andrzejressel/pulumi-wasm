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
//! const example = new cloudflare.TeamsAccount("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
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
//!     blockPage: {
//!         backgroundColor: "#000000",
//!         footerText: "hello",
//!         headerText: "hello",
//!         logoPath: "https://example.com/logo.jpg",
//!     },
//!     bodyScanning: {
//!         inspectionMode: "deep",
//!     },
//!     extendedEmailMatching: {
//!         enabled: true,
//!     },
//!     fips: {
//!         tls: true,
//!     },
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
//!     protocolDetectionEnabled: true,
//!     proxy: {
//!         rootCa: true,
//!         tcp: true,
//!         udp: true,
//!     },
//!     tlsDecryptEnabled: true,
//!     urlBrowserIsolationEnabled: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.TeamsAccount("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     antivirus=cloudflare.TeamsAccountAntivirusArgs(
//!         enabled_download_phase=True,
//!         enabled_upload_phase=False,
//!         fail_closed=True,
//!         notification_settings=cloudflare.TeamsAccountAntivirusNotificationSettingsArgs(
//!             enabled=True,
//!             message="you are blocked",
//!             support_url="https://example.com/blocked",
//!         ),
//!     ),
//!     block_page=cloudflare.TeamsAccountBlockPageArgs(
//!         background_color="#000000",
//!         footer_text="hello",
//!         header_text="hello",
//!         logo_path="https://example.com/logo.jpg",
//!     ),
//!     body_scanning=cloudflare.TeamsAccountBodyScanningArgs(
//!         inspection_mode="deep",
//!     ),
//!     extended_email_matching=cloudflare.TeamsAccountExtendedEmailMatchingArgs(
//!         enabled=True,
//!     ),
//!     fips=cloudflare.TeamsAccountFipsArgs(
//!         tls=True,
//!     ),
//!     logging=cloudflare.TeamsAccountLoggingArgs(
//!         redact_pii=True,
//!         settings_by_rule_type=cloudflare.TeamsAccountLoggingSettingsByRuleTypeArgs(
//!             dns=cloudflare.TeamsAccountLoggingSettingsByRuleTypeDnsArgs(
//!                 log_all=False,
//!                 log_blocks=True,
//!             ),
//!             http=cloudflare.TeamsAccountLoggingSettingsByRuleTypeHttpArgs(
//!                 log_all=True,
//!                 log_blocks=True,
//!             ),
//!             l4=cloudflare.TeamsAccountLoggingSettingsByRuleTypeL4Args(
//!                 log_all=False,
//!                 log_blocks=True,
//!             ),
//!         ),
//!     ),
//!     protocol_detection_enabled=True,
//!     proxy=cloudflare.TeamsAccountProxyArgs(
//!         root_ca=True,
//!         tcp=True,
//!         udp=True,
//!     ),
//!     tls_decrypt_enabled=True,
//!     url_browser_isolation_enabled=True)
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
//!     var example = new Cloudflare.TeamsAccount("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Antivirus = new Cloudflare.Inputs.TeamsAccountAntivirusArgs
//!         {
//!             EnabledDownloadPhase = true,
//!             EnabledUploadPhase = false,
//!             FailClosed = true,
//!             NotificationSettings = new Cloudflare.Inputs.TeamsAccountAntivirusNotificationSettingsArgs
//!             {
//!                 Enabled = true,
//!                 Message = "you are blocked",
//!                 SupportUrl = "https://example.com/blocked",
//!             },
//!         },
//!         BlockPage = new Cloudflare.Inputs.TeamsAccountBlockPageArgs
//!         {
//!             BackgroundColor = "#000000",
//!             FooterText = "hello",
//!             HeaderText = "hello",
//!             LogoPath = "https://example.com/logo.jpg",
//!         },
//!         BodyScanning = new Cloudflare.Inputs.TeamsAccountBodyScanningArgs
//!         {
//!             InspectionMode = "deep",
//!         },
//!         ExtendedEmailMatching = new Cloudflare.Inputs.TeamsAccountExtendedEmailMatchingArgs
//!         {
//!             Enabled = true,
//!         },
//!         Fips = new Cloudflare.Inputs.TeamsAccountFipsArgs
//!         {
//!             Tls = true,
//!         },
//!         Logging = new Cloudflare.Inputs.TeamsAccountLoggingArgs
//!         {
//!             RedactPii = true,
//!             SettingsByRuleType = new Cloudflare.Inputs.TeamsAccountLoggingSettingsByRuleTypeArgs
//!             {
//!                 Dns = new Cloudflare.Inputs.TeamsAccountLoggingSettingsByRuleTypeDnsArgs
//!                 {
//!                     LogAll = false,
//!                     LogBlocks = true,
//!                 },
//!                 Http = new Cloudflare.Inputs.TeamsAccountLoggingSettingsByRuleTypeHttpArgs
//!                 {
//!                     LogAll = true,
//!                     LogBlocks = true,
//!                 },
//!                 L4 = new Cloudflare.Inputs.TeamsAccountLoggingSettingsByRuleTypeL4Args
//!                 {
//!                     LogAll = false,
//!                     LogBlocks = true,
//!                 },
//!             },
//!         },
//!         ProtocolDetectionEnabled = true,
//!         Proxy = new Cloudflare.Inputs.TeamsAccountProxyArgs
//!         {
//!             RootCa = true,
//!             Tcp = true,
//!             Udp = true,
//!         },
//!         TlsDecryptEnabled = true,
//!         UrlBrowserIsolationEnabled = true,
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
//! 		_, err := cloudflare.NewTeamsAccount(ctx, "example", &cloudflare.TeamsAccountArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Antivirus: &cloudflare.TeamsAccountAntivirusArgs{
//! 				EnabledDownloadPhase: pulumi.Bool(true),
//! 				EnabledUploadPhase:   pulumi.Bool(false),
//! 				FailClosed:           pulumi.Bool(true),
//! 				NotificationSettings: &cloudflare.TeamsAccountAntivirusNotificationSettingsArgs{
//! 					Enabled:    pulumi.Bool(true),
//! 					Message:    pulumi.String("you are blocked"),
//! 					SupportUrl: pulumi.String("https://example.com/blocked"),
//! 				},
//! 			},
//! 			BlockPage: &cloudflare.TeamsAccountBlockPageArgs{
//! 				BackgroundColor: pulumi.String("#000000"),
//! 				FooterText:      pulumi.String("hello"),
//! 				HeaderText:      pulumi.String("hello"),
//! 				LogoPath:        pulumi.String("https://example.com/logo.jpg"),
//! 			},
//! 			BodyScanning: &cloudflare.TeamsAccountBodyScanningArgs{
//! 				InspectionMode: pulumi.String("deep"),
//! 			},
//! 			ExtendedEmailMatching: &cloudflare.TeamsAccountExtendedEmailMatchingArgs{
//! 				Enabled: pulumi.Bool(true),
//! 			},
//! 			Fips: &cloudflare.TeamsAccountFipsArgs{
//! 				Tls: pulumi.Bool(true),
//! 			},
//! 			Logging: &cloudflare.TeamsAccountLoggingArgs{
//! 				RedactPii: pulumi.Bool(true),
//! 				SettingsByRuleType: &cloudflare.TeamsAccountLoggingSettingsByRuleTypeArgs{
//! 					Dns: &cloudflare.TeamsAccountLoggingSettingsByRuleTypeDnsArgs{
//! 						LogAll:    pulumi.Bool(false),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 					Http: &cloudflare.TeamsAccountLoggingSettingsByRuleTypeHttpArgs{
//! 						LogAll:    pulumi.Bool(true),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 					L4: &cloudflare.TeamsAccountLoggingSettingsByRuleTypeL4Args{
//! 						LogAll:    pulumi.Bool(false),
//! 						LogBlocks: pulumi.Bool(true),
//! 					},
//! 				},
//! 			},
//! 			ProtocolDetectionEnabled: pulumi.Bool(true),
//! 			Proxy: &cloudflare.TeamsAccountProxyArgs{
//! 				RootCa: pulumi.Bool(true),
//! 				Tcp:    pulumi.Bool(true),
//! 				Udp:    pulumi.Bool(true),
//! 			},
//! 			TlsDecryptEnabled:          pulumi.Bool(true),
//! 			UrlBrowserIsolationEnabled: pulumi.Bool(true),
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
//! import com.pulumi.cloudflare.TeamsAccount;
//! import com.pulumi.cloudflare.TeamsAccountArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountAntivirusArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountAntivirusNotificationSettingsArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountBlockPageArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountBodyScanningArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountExtendedEmailMatchingArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountFipsArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountLoggingArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountLoggingSettingsByRuleTypeArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountLoggingSettingsByRuleTypeDnsArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountLoggingSettingsByRuleTypeHttpArgs;
//! import com.pulumi.cloudflare.inputs.TeamsAccountLoggingSettingsByRuleTypeL4Args;
//! import com.pulumi.cloudflare.inputs.TeamsAccountProxyArgs;
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
//!         var example = new TeamsAccount("example", TeamsAccountArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .antivirus(TeamsAccountAntivirusArgs.builder()
//!                 .enabledDownloadPhase(true)
//!                 .enabledUploadPhase(false)
//!                 .failClosed(true)
//!                 .notificationSettings(TeamsAccountAntivirusNotificationSettingsArgs.builder()
//!                     .enabled(true)
//!                     .message("you are blocked")
//!                     .supportUrl("https://example.com/blocked")
//!                     .build())
//!                 .build())
//!             .blockPage(TeamsAccountBlockPageArgs.builder()
//!                 .backgroundColor("#000000")
//!                 .footerText("hello")
//!                 .headerText("hello")
//!                 .logoPath("https://example.com/logo.jpg")
//!                 .build())
//!             .bodyScanning(TeamsAccountBodyScanningArgs.builder()
//!                 .inspectionMode("deep")
//!                 .build())
//!             .extendedEmailMatching(TeamsAccountExtendedEmailMatchingArgs.builder()
//!                 .enabled(true)
//!                 .build())
//!             .fips(TeamsAccountFipsArgs.builder()
//!                 .tls(true)
//!                 .build())
//!             .logging(TeamsAccountLoggingArgs.builder()
//!                 .redactPii(true)
//!                 .settingsByRuleType(TeamsAccountLoggingSettingsByRuleTypeArgs.builder()
//!                     .dns(TeamsAccountLoggingSettingsByRuleTypeDnsArgs.builder()
//!                         .logAll(false)
//!                         .logBlocks(true)
//!                         .build())
//!                     .http(TeamsAccountLoggingSettingsByRuleTypeHttpArgs.builder()
//!                         .logAll(true)
//!                         .logBlocks(true)
//!                         .build())
//!                     .l4(TeamsAccountLoggingSettingsByRuleTypeL4Args.builder()
//!                         .logAll(false)
//!                         .logBlocks(true)
//!                         .build())
//!                     .build())
//!                 .build())
//!             .protocolDetectionEnabled(true)
//!             .proxy(TeamsAccountProxyArgs.builder()
//!                 .rootCa(true)
//!                 .tcp(true)
//!                 .udp(true)
//!                 .build())
//!             .tlsDecryptEnabled(true)
//!             .urlBrowserIsolationEnabled(true)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:TeamsAccount
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       antivirus:
//!         enabledDownloadPhase: true
//!         enabledUploadPhase: false
//!         failClosed: true
//!         notificationSettings:
//!           enabled: true
//!           message: you are blocked
//!           supportUrl: https://example.com/blocked
//!       blockPage:
//!         backgroundColor: '#000000'
//!         footerText: hello
//!         headerText: hello
//!         logoPath: https://example.com/logo.jpg
//!       bodyScanning:
//!         inspectionMode: deep
//!       extendedEmailMatching:
//!         enabled: true
//!       fips:
//!         tls: true
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
//!       protocolDetectionEnabled: true
//!       proxy:
//!         rootCa: true
//!         tcp: true
//!         udp: true
//!       tlsDecryptEnabled: true
//!       urlBrowserIsolationEnabled: true
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsAccount:TeamsAccount example <account_id>
//! ```
//! 

pub struct TeamsAccountArgs {
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
    /// Configuration for extended e-mail matching.
    pub extended_email_matching: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
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
    /// Configuration for extended e-mail matching.
    pub extended_email_matching: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
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
        account_id: args.account_id.get_inner(),
        activity_log_enabled: args.activity_log_enabled.get_inner(),
        antivirus: args.antivirus.get_inner(),
        block_page: args.block_page.get_inner(),
        body_scanning: args.body_scanning.get_inner(),
        extended_email_matching: args.extended_email_matching.get_inner(),
        fips: args.fips.get_inner(),
        logging: args.logging.get_inner(),
        non_identity_browser_isolation_enabled: args.non_identity_browser_isolation_enabled.get_inner(),
        payload_log: args.payload_log.get_inner(),
        protocol_detection_enabled: args.protocol_detection_enabled.get_inner(),
        proxy: args.proxy.get_inner(),
        ssh_session_log: args.ssh_session_log.get_inner(),
        tls_decrypt_enabled: args.tls_decrypt_enabled.get_inner(),
        url_browser_isolation_enabled: args.url_browser_isolation_enabled.get_inner(),
    });

    TeamsAccountResult {
        account_id: crate::into_domain(result.account_id),
        activity_log_enabled: crate::into_domain(result.activity_log_enabled),
        antivirus: crate::into_domain(result.antivirus),
        block_page: crate::into_domain(result.block_page),
        body_scanning: crate::into_domain(result.body_scanning),
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
