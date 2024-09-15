//! Provides a Cloudflare Device Posture Integration resource. Device
//! posture integrations configure third-party data providers for device
//! posture rules.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.DevicePostureIntegration("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     configs: [{
//!         apiUrl: "https://example.com/api",
//!         authUrl: "https://example.com/connect/token",
//!         clientId: "client-id",
//!         clientSecret: "client-secret",
//!     }],
//!     interval: "24h",
//!     name: "Device posture integration",
//!     type: "workspace_one",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.DevicePostureIntegration("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     configs=[cloudflare.DevicePostureIntegrationConfigArgs(
//!         api_url="https://example.com/api",
//!         auth_url="https://example.com/connect/token",
//!         client_id="client-id",
//!         client_secret="client-secret",
//!     )],
//!     interval="24h",
//!     name="Device posture integration",
//!     type="workspace_one")
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
//!     var example = new Cloudflare.DevicePostureIntegration("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.DevicePostureIntegrationConfigArgs
//!             {
//!                 ApiUrl = "https://example.com/api",
//!                 AuthUrl = "https://example.com/connect/token",
//!                 ClientId = "client-id",
//!                 ClientSecret = "client-secret",
//!             },
//!         },
//!         Interval = "24h",
//!         Name = "Device posture integration",
//!         Type = "workspace_one",
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
//! 		_, err := cloudflare.NewDevicePostureIntegration(ctx, "example", &cloudflare.DevicePostureIntegrationArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Configs: cloudflare.DevicePostureIntegrationConfigArray{
//! 				&cloudflare.DevicePostureIntegrationConfigArgs{
//! 					ApiUrl:       pulumi.String("https://example.com/api"),
//! 					AuthUrl:      pulumi.String("https://example.com/connect/token"),
//! 					ClientId:     pulumi.String("client-id"),
//! 					ClientSecret: pulumi.String("client-secret"),
//! 				},
//! 			},
//! 			Interval: pulumi.String("24h"),
//! 			Name:     pulumi.String("Device posture integration"),
//! 			Type:     pulumi.String("workspace_one"),
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
//! import com.pulumi.cloudflare.DevicePostureIntegration;
//! import com.pulumi.cloudflare.DevicePostureIntegrationArgs;
//! import com.pulumi.cloudflare.inputs.DevicePostureIntegrationConfigArgs;
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
//!         var example = new DevicePostureIntegration("example", DevicePostureIntegrationArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .configs(DevicePostureIntegrationConfigArgs.builder()
//!                 .apiUrl("https://example.com/api")
//!                 .authUrl("https://example.com/connect/token")
//!                 .clientId("client-id")
//!                 .clientSecret("client-secret")
//!                 .build())
//!             .interval("24h")
//!             .name("Device posture integration")
//!             .type("workspace_one")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:DevicePostureIntegration
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       configs:
//!         - apiUrl: https://example.com/api
//!           authUrl: https://example.com/connect/token
//!           clientId: client-id
//!           clientSecret: client-secret
//!       interval: 24h
//!       name: Device posture integration
//!       type: workspace_one
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/devicePostureIntegration:DevicePostureIntegration example <account_id>/<device_posture_integration_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureIntegrationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DevicePostureIntegrationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DevicePostureIntegrationArgs) -> DevicePostureIntegrationResult {

    let result = crate::bindings::pulumi::cloudflare::device_posture_integration::invoke(name, &crate::bindings::pulumi::cloudflare::device_posture_integration::Args {
        account_id: &args.account_id.get_inner(),
        configs: &args.configs.get_inner(),
        identifier: &args.identifier.get_inner(),
        interval: &args.interval.get_inner(),
        name: &args.name.get_inner(),
        type_: &args.type_.get_inner(),
    });

    DevicePostureIntegrationResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        identifier: crate::into_domain(result.identifier),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
