//! Provides a Cloudflare Device Posture Rule resource. Device posture rules configure security policies for device posture checks.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const eaxmple = new cloudflare.ZeroTrustDevicePostureRule("eaxmple", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Corporate devices posture rule",
//!     type: "os_version",
//!     description: "Device posture rule for corporate devices.",
//!     schedule: "24h",
//!     expiration: "24h",
//!     matches: [{
//!         platform: "linux",
//!     }],
//!     inputs: [{
//!         id: corporateDevices.id,
//!         version: "1.0.0",
//!         operator: "<",
//!         osDistroName: "ubuntu",
//!         osDistroRevision: "1.0.0",
//!         osVersionExtra: "(a)",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! eaxmple = cloudflare.ZeroTrustDevicePostureRule("eaxmple",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Corporate devices posture rule",
//!     type="os_version",
//!     description="Device posture rule for corporate devices.",
//!     schedule="24h",
//!     expiration="24h",
//!     matches=[{
//!         "platform": "linux",
//!     }],
//!     inputs=[{
//!         "id": corporate_devices["id"],
//!         "version": "1.0.0",
//!         "operator": "<",
//!         "os_distro_name": "ubuntu",
//!         "os_distro_revision": "1.0.0",
//!         "os_version_extra": "(a)",
//!     }])
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
//!     var eaxmple = new Cloudflare.ZeroTrustDevicePostureRule("eaxmple", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Corporate devices posture rule",
//!         Type = "os_version",
//!         Description = "Device posture rule for corporate devices.",
//!         Schedule = "24h",
//!         Expiration = "24h",
//!         Matches = new[]
//!         {
//!             new Cloudflare.Inputs.ZeroTrustDevicePostureRuleMatchArgs
//!             {
//!                 Platform = "linux",
//!             },
//!         },
//!         Inputs = new[]
//!         {
//!             new Cloudflare.Inputs.ZeroTrustDevicePostureRuleInputArgs
//!             {
//!                 Id = corporateDevices.Id,
//!                 Version = "1.0.0",
//!                 Operator = "<",
//!                 OsDistroName = "ubuntu",
//!                 OsDistroRevision = "1.0.0",
//!                 OsVersionExtra = "(a)",
//!             },
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
//! 		_, err := cloudflare.NewZeroTrustDevicePostureRule(ctx, "eaxmple", &cloudflare.ZeroTrustDevicePostureRuleArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("Corporate devices posture rule"),
//! 			Type:        pulumi.String("os_version"),
//! 			Description: pulumi.String("Device posture rule for corporate devices."),
//! 			Schedule:    pulumi.String("24h"),
//! 			Expiration:  pulumi.String("24h"),
//! 			Matches: cloudflare.ZeroTrustDevicePostureRuleMatchArray{
//! 				&cloudflare.ZeroTrustDevicePostureRuleMatchArgs{
//! 					Platform: pulumi.String("linux"),
//! 				},
//! 			},
//! 			Inputs: cloudflare.ZeroTrustDevicePostureRuleInputTypeArray{
//! 				&cloudflare.ZeroTrustDevicePostureRuleInputTypeArgs{
//! 					Id:               pulumi.Any(corporateDevices.Id),
//! 					Version:          pulumi.String("1.0.0"),
//! 					Operator:         pulumi.String("<"),
//! 					OsDistroName:     pulumi.String("ubuntu"),
//! 					OsDistroRevision: pulumi.String("1.0.0"),
//! 					OsVersionExtra:   pulumi.String("(a)"),
//! 				},
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
//! import com.pulumi.cloudflare.ZeroTrustDevicePostureRule;
//! import com.pulumi.cloudflare.ZeroTrustDevicePostureRuleArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustDevicePostureRuleMatchArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustDevicePostureRuleInputArgs;
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
//!         var eaxmple = new ZeroTrustDevicePostureRule("eaxmple", ZeroTrustDevicePostureRuleArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Corporate devices posture rule")
//!             .type("os_version")
//!             .description("Device posture rule for corporate devices.")
//!             .schedule("24h")
//!             .expiration("24h")
//!             .matches(ZeroTrustDevicePostureRuleMatchArgs.builder()
//!                 .platform("linux")
//!                 .build())
//!             .inputs(ZeroTrustDevicePostureRuleInputArgs.builder()
//!                 .id(corporateDevices.id())
//!                 .version("1.0.0")
//!                 .operator("<")
//!                 .osDistroName("ubuntu")
//!                 .osDistroRevision("1.0.0")
//!                 .osVersionExtra("(a)")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   eaxmple:
//!     type: cloudflare:ZeroTrustDevicePostureRule
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Corporate devices posture rule
//!       type: os_version
//!       description: Device posture rule for corporate devices.
//!       schedule: 24h
//!       expiration: 24h
//!       matches:
//!         - platform: linux
//!       inputs:
//!         - id: ${corporateDevices.id}
//!           version: 1.0.0
//!           operator: <
//!           osDistroName: ubuntu
//!           osDistroRevision: 1.0.0
//!           osVersionExtra: (a)
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDevicePostureRule:ZeroTrustDevicePostureRule example <account_id>/<device_posture_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDevicePostureRuleArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub inputs: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleInput>>>,
    /// The conditions that the client must match to run the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleMatch>>>,
    /// Name of the device posture rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustDevicePostureRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    pub inputs: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustDevicePostureRuleInput>>,
    /// The conditions that the client must match to run the rule.
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDevicePostureRuleMatch>>>,
    /// Name of the device posture rule.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustDevicePostureRuleArgs) -> ZeroTrustDevicePostureRuleResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_device_posture_rule::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_device_posture_rule::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        expiration: &args.expiration.get_inner(),
        inputs: &args.inputs.get_inner(),
        matches: &args.matches.get_inner(),
        name: &args.name.get_inner(),
        schedule: &args.schedule.get_inner(),
        type_: &args.type_.get_inner(),
    });

    ZeroTrustDevicePostureRuleResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        expiration: crate::into_domain(result.expiration),
        inputs: crate::into_domain(result.inputs),
        matches: crate::into_domain(result.matches),
        name: crate::into_domain(result.name),
        schedule: crate::into_domain(result.schedule),
        type_: crate::into_domain(result.type_),
    }
}