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
//! const example = new cloudflare.ZeroTrustInfrastructureAccessTarget("example", {
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
//! const ipv4OnlyExample = new cloudflare.ZeroTrustInfrastructureAccessTarget("ipv4_only_example", {
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
//! example = cloudflare.ZeroTrustInfrastructureAccessTarget("example",
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
//! ipv4_only_example = cloudflare.ZeroTrustInfrastructureAccessTarget("ipv4_only_example",
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
//!     var example = new Cloudflare.ZeroTrustInfrastructureAccessTarget("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Hostname = "example-target",
//!         Ip = new Cloudflare.Inputs.ZeroTrustInfrastructureAccessTargetIpArgs
//!         {
//!             Ipv4 = new Cloudflare.Inputs.ZeroTrustInfrastructureAccessTargetIpIpv4Args
//!             {
//!                 IpAddr = "198.51.100.1",
//!                 VirtualNetworkId = "238dccd1-149b-463d-8228-560ab83a54fd",
//!             },
//!             Ipv6 = new Cloudflare.Inputs.ZeroTrustInfrastructureAccessTargetIpIpv6Args
//!             {
//!                 IpAddr = "2001:db8::",
//!                 VirtualNetworkId = "238dccd1-149b-463d-8228-560ab83a54fd",
//!             },
//!         },
//!     });
//! 
//!     var ipv4OnlyExample = new Cloudflare.ZeroTrustInfrastructureAccessTarget("ipv4_only_example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Hostname = "example-ipv4-only",
//!         Ip = new Cloudflare.Inputs.ZeroTrustInfrastructureAccessTargetIpArgs
//!         {
//!             Ipv4 = new Cloudflare.Inputs.ZeroTrustInfrastructureAccessTargetIpIpv4Args
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
//! 		_, err := cloudflare.NewZeroTrustInfrastructureAccessTarget(ctx, "example", &cloudflare.ZeroTrustInfrastructureAccessTargetArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Hostname:  pulumi.String("example-target"),
//! 			Ip: &cloudflare.ZeroTrustInfrastructureAccessTargetIpArgs{
//! 				Ipv4: &cloudflare.ZeroTrustInfrastructureAccessTargetIpIpv4Args{
//! 					IpAddr:           pulumi.String("198.51.100.1"),
//! 					VirtualNetworkId: pulumi.String("238dccd1-149b-463d-8228-560ab83a54fd"),
//! 				},
//! 				Ipv6: &cloudflare.ZeroTrustInfrastructureAccessTargetIpIpv6Args{
//! 					IpAddr:           pulumi.String("2001:db8::"),
//! 					VirtualNetworkId: pulumi.String("238dccd1-149b-463d-8228-560ab83a54fd"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewZeroTrustInfrastructureAccessTarget(ctx, "ipv4_only_example", &cloudflare.ZeroTrustInfrastructureAccessTargetArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Hostname:  pulumi.String("example-ipv4-only"),
//! 			Ip: &cloudflare.ZeroTrustInfrastructureAccessTargetIpArgs{
//! 				Ipv4: &cloudflare.ZeroTrustInfrastructureAccessTargetIpIpv4Args{
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
//! import com.pulumi.cloudflare.ZeroTrustInfrastructureAccessTarget;
//! import com.pulumi.cloudflare.ZeroTrustInfrastructureAccessTargetArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustInfrastructureAccessTargetIpArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustInfrastructureAccessTargetIpIpv4Args;
//! import com.pulumi.cloudflare.inputs.ZeroTrustInfrastructureAccessTargetIpIpv6Args;
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
//!         var example = new ZeroTrustInfrastructureAccessTarget("example", ZeroTrustInfrastructureAccessTargetArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-target")
//!             .ip(ZeroTrustInfrastructureAccessTargetIpArgs.builder()
//!                 .ipv4(ZeroTrustInfrastructureAccessTargetIpIpv4Args.builder()
//!                     .ipAddr("198.51.100.1")
//!                     .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                     .build())
//!                 .ipv6(ZeroTrustInfrastructureAccessTargetIpIpv6Args.builder()
//!                     .ipAddr("2001:db8::")
//!                     .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!         var ipv4OnlyExample = new ZeroTrustInfrastructureAccessTarget("ipv4OnlyExample", ZeroTrustInfrastructureAccessTargetArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-ipv4-only")
//!             .ip(ZeroTrustInfrastructureAccessTargetIpArgs.builder()
//!                 .ipv4(ZeroTrustInfrastructureAccessTargetIpIpv4Args.builder()
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
//!     type: cloudflare:ZeroTrustInfrastructureAccessTarget
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
//!     type: cloudflare:ZeroTrustInfrastructureAccessTarget
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
//! $ pulumi import cloudflare:index/zeroTrustInfrastructureAccessTarget:ZeroTrustInfrastructureAccessTarget example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustInfrastructureAccessTargetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    pub ip: pulumi_wasm_rust::Output<crate::types::ZeroTrustInfrastructureAccessTargetIp>,
}

pub struct ZeroTrustInfrastructureAccessTargetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time at which the target was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    pub ip: pulumi_wasm_rust::Output<crate::types::ZeroTrustInfrastructureAccessTargetIp>,
    /// The date and time at which the target was last modified.
    pub modified_at: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustInfrastructureAccessTargetArgs) -> ZeroTrustInfrastructureAccessTargetResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_infrastructure_access_target::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_infrastructure_access_target::Args {
        account_id: &args.account_id.get_inner(),
        hostname: &args.hostname.get_inner(),
        ip: &args.ip.get_inner(),
    });

    ZeroTrustInfrastructureAccessTargetResult {
        account_id: crate::into_domain(result.account_id),
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ip: crate::into_domain(result.ip),
        modified_at: crate::into_domain(result.modified_at),
    }
}
