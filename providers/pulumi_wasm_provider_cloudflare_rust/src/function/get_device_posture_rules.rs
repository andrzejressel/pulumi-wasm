//! Use this data source to lookup a list of [Device Posture Rule](https://developers.cloudflare.com/cloudflare-one/identity/devices)
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getDevicePostureRules({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "check for /dev/random",
//!     type: "file",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_device_posture_rules(account_id="f037e56e89293a057740de681ac9abbe",
//!     name="check for /dev/random",
//!     type="file")
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
//!     var example = Cloudflare.GetDevicePostureRules.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "check for /dev/random",
//!         Type = "file",
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
//! 		_, err := cloudflare.GetDevicePostureRules(ctx, &cloudflare.GetDevicePostureRulesArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 			Name:      pulumi.StringRef("check for /dev/random"),
//! 			Type:      pulumi.StringRef("file"),
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetDevicePostureRulesArgs;
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
//!         final var example = CloudflareFunctions.getDevicePostureRules(GetDevicePostureRulesArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("check for /dev/random")
//!             .type("file")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getDevicePostureRules
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         name: check for /dev/random
//!         type: file
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetDevicePostureRulesArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the Device Posture Rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetDevicePostureRulesResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Name of the Device Posture Rule.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// A list of matching Device Posture Rules.
    pub rules: pulumi_wasm_rust::Output<Vec<crate::types::GetDevicePostureRulesRule>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDevicePostureRulesArgs
) -> GetDevicePostureRulesResult {

    let result = crate::bindings::pulumi::cloudflare::get_device_posture_rules::invoke(
        &crate::bindings::pulumi::cloudflare::get_device_posture_rules::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
                type_: &args.type_.get_inner(),
        }
    );

    GetDevicePostureRulesResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        rules: crate::into_domain(result.rules),
        type_: crate::into_domain(result.type_),
    }
}
