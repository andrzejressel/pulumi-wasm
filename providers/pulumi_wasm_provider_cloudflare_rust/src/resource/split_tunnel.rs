//! Provides a Cloudflare Split Tunnel resource. Split tunnels are used to either
//! include or exclude lists of routes from the WARP client's tunnel.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Excluding *.example.com from WARP routes
//! const exampleSplitTunnelExclude = new cloudflare.SplitTunnel("exampleSplitTunnelExclude", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     mode: "exclude",
//!     tunnels: [{
//!         host: "*.example.com",
//!         description: "example domain",
//!     }],
//! });
//! // Including *.example.com in WARP routes
//! const exampleSplitTunnelIncludeSplitTunnel = new cloudflare.SplitTunnel("exampleSplitTunnelIncludeSplitTunnel", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     mode: "include",
//!     tunnels: [{
//!         host: "*.example.com",
//!         description: "example domain",
//!     }],
//! });
//! // Create a device policy
//! const developerWarpPolicy = new cloudflare.DeviceSettingsPolicy("developerWarpPolicy", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Developers",
//!     precedence: 10,
//!     match: "any(identity.groups.name[*] in {\"Developers\"})",
//!     switchLocked: true,
//! });
//! // Excluding *.example.com from WARP routes for a particular device policy
//! const exampleDeviceSettingsPolicySplitTunnelExclude = new cloudflare.SplitTunnel("exampleDeviceSettingsPolicySplitTunnelExclude", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     policyId: developerWarpPolicy.id,
//!     mode: "exclude",
//!     tunnels: [{
//!         host: "*.example.com",
//!         description: "example domain",
//!     }],
//! });
//! // Including *.example.com in WARP routes for a particular device policy
//! const exampleSplitTunnelIncludeIndex_splitTunnelSplitTunnel = new cloudflare.SplitTunnel("exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     policyId: cloudflare_device_policy.developer_warp_policy.id,
//!     mode: "include",
//!     tunnels: [{
//!         host: "*.example.com",
//!         description: "example domain",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Excluding *.example.com from WARP routes
//! example_split_tunnel_exclude = cloudflare.SplitTunnel("exampleSplitTunnelExclude",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     mode="exclude",
//!     tunnels=[cloudflare.SplitTunnelTunnelArgs(
//!         host="*.example.com",
//!         description="example domain",
//!     )])
//! # Including *.example.com in WARP routes
//! example_split_tunnel_include_split_tunnel = cloudflare.SplitTunnel("exampleSplitTunnelIncludeSplitTunnel",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     mode="include",
//!     tunnels=[cloudflare.SplitTunnelTunnelArgs(
//!         host="*.example.com",
//!         description="example domain",
//!     )])
//! # Create a device policy
//! developer_warp_policy = cloudflare.DeviceSettingsPolicy("developerWarpPolicy",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Developers",
//!     precedence=10,
//!     match="any(identity.groups.name[*] in {\"Developers\"})",
//!     switch_locked=True)
//! # Excluding *.example.com from WARP routes for a particular device policy
//! example_device_settings_policy_split_tunnel_exclude = cloudflare.SplitTunnel("exampleDeviceSettingsPolicySplitTunnelExclude",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     policy_id=developer_warp_policy.id,
//!     mode="exclude",
//!     tunnels=[cloudflare.SplitTunnelTunnelArgs(
//!         host="*.example.com",
//!         description="example domain",
//!     )])
//! # Including *.example.com in WARP routes for a particular device policy
//! example_split_tunnel_include_index_split_tunnel_split_tunnel = cloudflare.SplitTunnel("exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     policy_id=cloudflare_device_policy["developer_warp_policy"]["id"],
//!     mode="include",
//!     tunnels=[cloudflare.SplitTunnelTunnelArgs(
//!         host="*.example.com",
//!         description="example domain",
//!     )])
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
//!     // Excluding *.example.com from WARP routes
//!     var exampleSplitTunnelExclude = new Cloudflare.SplitTunnel("exampleSplitTunnelExclude", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Mode = "exclude",
//!         Tunnels = new[]
//!         {
//!             new Cloudflare.Inputs.SplitTunnelTunnelArgs
//!             {
//!                 Host = "*.example.com",
//!                 Description = "example domain",
//!             },
//!         },
//!     });
//! 
//!     // Including *.example.com in WARP routes
//!     var exampleSplitTunnelIncludeSplitTunnel = new Cloudflare.SplitTunnel("exampleSplitTunnelIncludeSplitTunnel", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Mode = "include",
//!         Tunnels = new[]
//!         {
//!             new Cloudflare.Inputs.SplitTunnelTunnelArgs
//!             {
//!                 Host = "*.example.com",
//!                 Description = "example domain",
//!             },
//!         },
//!     });
//! 
//!     // Create a device policy
//!     var developerWarpPolicy = new Cloudflare.DeviceSettingsPolicy("developerWarpPolicy", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Developers",
//!         Precedence = 10,
//!         Match = "any(identity.groups.name[*] in {\"Developers\"})",
//!         SwitchLocked = true,
//!     });
//! 
//!     // Excluding *.example.com from WARP routes for a particular device policy
//!     var exampleDeviceSettingsPolicySplitTunnelExclude = new Cloudflare.SplitTunnel("exampleDeviceSettingsPolicySplitTunnelExclude", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         PolicyId = developerWarpPolicy.Id,
//!         Mode = "exclude",
//!         Tunnels = new[]
//!         {
//!             new Cloudflare.Inputs.SplitTunnelTunnelArgs
//!             {
//!                 Host = "*.example.com",
//!                 Description = "example domain",
//!             },
//!         },
//!     });
//! 
//!     // Including *.example.com in WARP routes for a particular device policy
//!     var exampleSplitTunnelIncludeIndex_splitTunnelSplitTunnel = new Cloudflare.SplitTunnel("exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         PolicyId = cloudflare_device_policy.Developer_warp_policy.Id,
//!         Mode = "include",
//!         Tunnels = new[]
//!         {
//!             new Cloudflare.Inputs.SplitTunnelTunnelArgs
//!             {
//!                 Host = "*.example.com",
//!                 Description = "example domain",
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
//! 		// Excluding *.example.com from WARP routes
//! 		_, err := cloudflare.NewSplitTunnel(ctx, "exampleSplitTunnelExclude", &cloudflare.SplitTunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Mode:      pulumi.String("exclude"),
//! 			Tunnels: cloudflare.SplitTunnelTunnelArray{
//! 				&cloudflare.SplitTunnelTunnelArgs{
//! 					Host:        pulumi.String("*.example.com"),
//! 					Description: pulumi.String("example domain"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Including *.example.com in WARP routes
//! 		_, err = cloudflare.NewSplitTunnel(ctx, "exampleSplitTunnelIncludeSplitTunnel", &cloudflare.SplitTunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Mode:      pulumi.String("include"),
//! 			Tunnels: cloudflare.SplitTunnelTunnelArray{
//! 				&cloudflare.SplitTunnelTunnelArgs{
//! 					Host:        pulumi.String("*.example.com"),
//! 					Description: pulumi.String("example domain"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Create a device policy
//! 		developerWarpPolicy, err := cloudflare.NewDeviceSettingsPolicy(ctx, "developerWarpPolicy", &cloudflare.DeviceSettingsPolicyArgs{
//! 			AccountId:    pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:         pulumi.String("Developers"),
//! 			Precedence:   pulumi.Int(10),
//! 			Match:        pulumi.String("any(identity.groups.name[*] in {\"Developers\"})"),
//! 			SwitchLocked: pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Excluding *.example.com from WARP routes for a particular device policy
//! 		_, err = cloudflare.NewSplitTunnel(ctx, "exampleDeviceSettingsPolicySplitTunnelExclude", &cloudflare.SplitTunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			PolicyId:  developerWarpPolicy.ID(),
//! 			Mode:      pulumi.String("exclude"),
//! 			Tunnels: cloudflare.SplitTunnelTunnelArray{
//! 				&cloudflare.SplitTunnelTunnelArgs{
//! 					Host:        pulumi.String("*.example.com"),
//! 					Description: pulumi.String("example domain"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Including *.example.com in WARP routes for a particular device policy
//! 		_, err = cloudflare.NewSplitTunnel(ctx, "exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel", &cloudflare.SplitTunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			PolicyId:  pulumi.Any(cloudflare_device_policy.Developer_warp_policy.Id),
//! 			Mode:      pulumi.String("include"),
//! 			Tunnels: cloudflare.SplitTunnelTunnelArray{
//! 				&cloudflare.SplitTunnelTunnelArgs{
//! 					Host:        pulumi.String("*.example.com"),
//! 					Description: pulumi.String("example domain"),
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
//! import com.pulumi.cloudflare.SplitTunnel;
//! import com.pulumi.cloudflare.SplitTunnelArgs;
//! import com.pulumi.cloudflare.inputs.SplitTunnelTunnelArgs;
//! import com.pulumi.cloudflare.DeviceSettingsPolicy;
//! import com.pulumi.cloudflare.DeviceSettingsPolicyArgs;
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
//!         // Excluding *.example.com from WARP routes
//!         var exampleSplitTunnelExclude = new SplitTunnel("exampleSplitTunnelExclude", SplitTunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .mode("exclude")
//!             .tunnels(SplitTunnelTunnelArgs.builder()
//!                 .host("*.example.com")
//!                 .description("example domain")
//!                 .build())
//!             .build());
//! 
//!         // Including *.example.com in WARP routes
//!         var exampleSplitTunnelIncludeSplitTunnel = new SplitTunnel("exampleSplitTunnelIncludeSplitTunnel", SplitTunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .mode("include")
//!             .tunnels(SplitTunnelTunnelArgs.builder()
//!                 .host("*.example.com")
//!                 .description("example domain")
//!                 .build())
//!             .build());
//! 
//!         // Create a device policy
//!         var developerWarpPolicy = new DeviceSettingsPolicy("developerWarpPolicy", DeviceSettingsPolicyArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Developers")
//!             .precedence(10)
//!             .match("any(identity.groups.name[*] in {\"Developers\"})")
//!             .switchLocked(true)
//!             .build());
//! 
//!         // Excluding *.example.com from WARP routes for a particular device policy
//!         var exampleDeviceSettingsPolicySplitTunnelExclude = new SplitTunnel("exampleDeviceSettingsPolicySplitTunnelExclude", SplitTunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .policyId(developerWarpPolicy.id())
//!             .mode("exclude")
//!             .tunnels(SplitTunnelTunnelArgs.builder()
//!                 .host("*.example.com")
//!                 .description("example domain")
//!                 .build())
//!             .build());
//! 
//!         // Including *.example.com in WARP routes for a particular device policy
//!         var exampleSplitTunnelIncludeIndex_splitTunnelSplitTunnel = new SplitTunnel("exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel", SplitTunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .policyId(cloudflare_device_policy.developer_warp_policy().id())
//!             .mode("include")
//!             .tunnels(SplitTunnelTunnelArgs.builder()
//!                 .host("*.example.com")
//!                 .description("example domain")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Excluding *.example.com from WARP routes
//!   exampleSplitTunnelExclude:
//!     type: cloudflare:SplitTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       mode: exclude
//!       tunnels:
//!         - host: '*.example.com'
//!           description: example domain
//!   # Including *.example.com in WARP routes
//!   exampleSplitTunnelIncludeSplitTunnel:
//!     type: cloudflare:SplitTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       mode: include
//!       tunnels:
//!         - host: '*.example.com'
//!           description: example domain
//!   # Create a device policy
//!   developerWarpPolicy:
//!     type: cloudflare:DeviceSettingsPolicy
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Developers
//!       precedence: 10
//!       match: any(identity.groups.name[*] in {"Developers"})
//!       switchLocked: true
//!   # Excluding *.example.com from WARP routes for a particular device policy
//!   exampleDeviceSettingsPolicySplitTunnelExclude:
//!     type: cloudflare:SplitTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       policyId: ${developerWarpPolicy.id}
//!       mode: exclude
//!       tunnels:
//!         - host: '*.example.com'
//!           description: example domain
//!   # Including *.example.com in WARP routes for a particular device policy
//!   exampleSplitTunnelIncludeIndex/splitTunnelSplitTunnel:
//!     type: cloudflare:SplitTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       policyId: ${cloudflare_device_policy.developer_warp_policy.id}
//!       mode: include
//!       tunnels:
//!         - host: '*.example.com'
//!           description: example domain
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Split Tunnels for default device policies must use "default" as the policy ID.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/splitTunnel:SplitTunnel example <account_id>/<policy_id>/<mode>
//! ```
//! 

pub struct SplitTunnelArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

pub struct SplitTunnelResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SplitTunnelArgs) -> SplitTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::split_tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::split_tunnel::Args {
        account_id: args.account_id.get_inner(),
        mode: args.mode.get_inner(),
        policy_id: args.policy_id.get_inner(),
        tunnels: args.tunnels.get_inner(),
    });

    SplitTunnelResult {
        account_id: crate::into_domain(result.account_id),
        mode: crate::into_domain(result.mode),
        policy_id: crate::into_domain(result.policy_id),
        tunnels: crate::into_domain(result.tunnels),
    }
}
