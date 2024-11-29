//! The [Infrastructure Access Target](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/use-cases/ssh/ssh-infrastructure-access/#4-add-a-target) resource allows you to configure Infrastructure Access Targets for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.InfrastructureAccessTarget("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     hostname: "example-target",
//!     ip: {
//!         ipv4: {
//!             ipAddr: "198.51.100.1",
//!             virtualNetworkId: "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
//!         ipv6: {
//!             ipAddr: "2001:db8::",
//!             virtualNetworkId: "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
//!     },
//! });
//! const ipv4OnlyExample = new cloudflare.InfrastructureAccessTarget("ipv4_only_example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     hostname: "example-ipv4-only",
//!     ip: {
//!         ipv4: {
//!             ipAddr: "198.51.100.1",
//!             virtualNetworkId: "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.InfrastructureAccessTarget("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     hostname="example-target",
//!     ip={
//!         "ipv4": {
//!             "ip_addr": "198.51.100.1",
//!             "virtual_network_id": "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
//!         "ipv6": {
//!             "ip_addr": "2001:db8::",
//!             "virtual_network_id": "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
//!     })
//! ipv4_only_example = cloudflare.InfrastructureAccessTarget("ipv4_only_example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     hostname="example-ipv4-only",
//!     ip={
//!         "ipv4": {
//!             "ip_addr": "198.51.100.1",
//!             "virtual_network_id": "238dccd1-149b-463d-8228-560ab83a54fd",
//!         },
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
//!     var example = new Cloudflare.InfrastructureAccessTarget("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Hostname = "example-target",
//!         Ip = new Cloudflare.Inputs.InfrastructureAccessTargetIpArgs
//!         {
//!             Ipv4 = new Cloudflare.Inputs.InfrastructureAccessTargetIpIpv4Args
//!             {
//!                 IpAddr = "198.51.100.1",
//!                 VirtualNetworkId = "238dccd1-149b-463d-8228-560ab83a54fd",
//!             },
//!             Ipv6 = new Cloudflare.Inputs.InfrastructureAccessTargetIpIpv6Args
//!             {
//!                 IpAddr = "2001:db8::",
//!                 VirtualNetworkId = "238dccd1-149b-463d-8228-560ab83a54fd",
//!             },
//!         },
//!     });
//! 
//!     var ipv4OnlyExample = new Cloudflare.InfrastructureAccessTarget("ipv4_only_example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Hostname = "example-ipv4-only",
//!         Ip = new Cloudflare.Inputs.InfrastructureAccessTargetIpArgs
//!         {
//!             Ipv4 = new Cloudflare.Inputs.InfrastructureAccessTargetIpIpv4Args
//!             {
//!                 IpAddr = "198.51.100.1",
//!                 VirtualNetworkId = "238dccd1-149b-463d-8228-560ab83a54fd",
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
//! 		_, err := cloudflare.NewInfrastructureAccessTarget(ctx, "example", &cloudflare.InfrastructureAccessTargetArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Hostname:  pulumi.String("example-target"),
//! 			Ip: &cloudflare.InfrastructureAccessTargetIpArgs{
//! 				Ipv4: &cloudflare.InfrastructureAccessTargetIpIpv4Args{
//! 					IpAddr:           pulumi.String("198.51.100.1"),
//! 					VirtualNetworkId: pulumi.String("238dccd1-149b-463d-8228-560ab83a54fd"),
//! 				},
//! 				Ipv6: &cloudflare.InfrastructureAccessTargetIpIpv6Args{
//! 					IpAddr:           pulumi.String("2001:db8::"),
//! 					VirtualNetworkId: pulumi.String("238dccd1-149b-463d-8228-560ab83a54fd"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewInfrastructureAccessTarget(ctx, "ipv4_only_example", &cloudflare.InfrastructureAccessTargetArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Hostname:  pulumi.String("example-ipv4-only"),
//! 			Ip: &cloudflare.InfrastructureAccessTargetIpArgs{
//! 				Ipv4: &cloudflare.InfrastructureAccessTargetIpIpv4Args{
//! 					IpAddr:           pulumi.String("198.51.100.1"),
//! 					VirtualNetworkId: pulumi.String("238dccd1-149b-463d-8228-560ab83a54fd"),
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
//! import com.pulumi.cloudflare.InfrastructureAccessTarget;
//! import com.pulumi.cloudflare.InfrastructureAccessTargetArgs;
//! import com.pulumi.cloudflare.inputs.InfrastructureAccessTargetIpArgs;
//! import com.pulumi.cloudflare.inputs.InfrastructureAccessTargetIpIpv4Args;
//! import com.pulumi.cloudflare.inputs.InfrastructureAccessTargetIpIpv6Args;
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
//!         var example = new InfrastructureAccessTarget("example", InfrastructureAccessTargetArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-target")
//!             .ip(InfrastructureAccessTargetIpArgs.builder()
//!                 .ipv4(InfrastructureAccessTargetIpIpv4Args.builder()
//!                     .ipAddr("198.51.100.1")
//!                     .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                     .build())
//!                 .ipv6(InfrastructureAccessTargetIpIpv6Args.builder()
//!                     .ipAddr("2001:db8::")
//!                     .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!         var ipv4OnlyExample = new InfrastructureAccessTarget("ipv4OnlyExample", InfrastructureAccessTargetArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-ipv4-only")
//!             .ip(InfrastructureAccessTargetIpArgs.builder()
//!                 .ipv4(InfrastructureAccessTargetIpIpv4Args.builder()
//!                     .ipAddr("198.51.100.1")
//!                     .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                     .build())
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
//!     type: cloudflare:InfrastructureAccessTarget
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       hostname: example-target
//!       ip:
//!         ipv4:
//!           ipAddr: 198.51.100.1
//!           virtualNetworkId: 238dccd1-149b-463d-8228-560ab83a54fd
//!         ipv6:
//!           ipAddr: '2001:db8::'
//!           virtualNetworkId: 238dccd1-149b-463d-8228-560ab83a54fd
//!   ipv4OnlyExample:
//!     type: cloudflare:InfrastructureAccessTarget
//!     name: ipv4_only_example
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       hostname: example-ipv4-only
//!       ip:
//!         ipv4:
//!           ipAddr: 198.51.100.1
//!           virtualNetworkId: 238dccd1-149b-463d-8228-560ab83a54fd
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct InfrastructureAccessTargetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    pub ip: pulumi_wasm_rust::Output<crate::types::InfrastructureAccessTargetIp>,
}

pub struct InfrastructureAccessTargetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time at which the target was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    pub ip: pulumi_wasm_rust::Output<crate::types::InfrastructureAccessTargetIp>,
    /// The date and time at which the target was last modified.
    pub modified_at: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: InfrastructureAccessTargetArgs) -> InfrastructureAccessTargetResult {

    let result = crate::bindings::pulumi::cloudflare::infrastructure_access_target::invoke(name, &crate::bindings::pulumi::cloudflare::infrastructure_access_target::Args {
        account_id: &args.account_id.get_inner(),
        hostname: &args.hostname.get_inner(),
        ip: &args.ip.get_inner(),
    });

    InfrastructureAccessTargetResult {
        account_id: crate::into_domain(result.account_id),
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ip: crate::into_domain(result.ip),
        modified_at: crate::into_domain(result.modified_at),
    }
}
