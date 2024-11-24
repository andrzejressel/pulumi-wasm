//! Provides a resource, that manages GRE tunnels for Magic Transit.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.GreTunnel("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "GRE_1",
//!     customerGreEndpoint: "203.0.113.1",
//!     cloudflareGreEndpoint: "203.0.113.2",
//!     interfaceAddress: "192.0.2.0/31",
//!     description: "Tunnel for ISP X",
//!     ttl: 64,
//!     mtu: 1476,
//!     healthCheckEnabled: true,
//!     healthCheckTarget: "203.0.113.1",
//!     healthCheckType: "reply",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.GreTunnel("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="GRE_1",
//!     customer_gre_endpoint="203.0.113.1",
//!     cloudflare_gre_endpoint="203.0.113.2",
//!     interface_address="192.0.2.0/31",
//!     description="Tunnel for ISP X",
//!     ttl=64,
//!     mtu=1476,
//!     health_check_enabled=True,
//!     health_check_target="203.0.113.1",
//!     health_check_type="reply")
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
//!     var example = new Cloudflare.GreTunnel("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "GRE_1",
//!         CustomerGreEndpoint = "203.0.113.1",
//!         CloudflareGreEndpoint = "203.0.113.2",
//!         InterfaceAddress = "192.0.2.0/31",
//!         Description = "Tunnel for ISP X",
//!         Ttl = 64,
//!         Mtu = 1476,
//!         HealthCheckEnabled = true,
//!         HealthCheckTarget = "203.0.113.1",
//!         HealthCheckType = "reply",
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
//! 		_, err := cloudflare.NewGreTunnel(ctx, "example", &cloudflare.GreTunnelArgs{
//! 			AccountId:             pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:                  pulumi.String("GRE_1"),
//! 			CustomerGreEndpoint:   pulumi.String("203.0.113.1"),
//! 			CloudflareGreEndpoint: pulumi.String("203.0.113.2"),
//! 			InterfaceAddress:      pulumi.String("192.0.2.0/31"),
//! 			Description:           pulumi.String("Tunnel for ISP X"),
//! 			Ttl:                   pulumi.Int(64),
//! 			Mtu:                   pulumi.Int(1476),
//! 			HealthCheckEnabled:    pulumi.Bool(true),
//! 			HealthCheckTarget:     pulumi.String("203.0.113.1"),
//! 			HealthCheckType:       pulumi.String("reply"),
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
//! import com.pulumi.cloudflare.GreTunnel;
//! import com.pulumi.cloudflare.GreTunnelArgs;
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
//!         var example = new GreTunnel("example", GreTunnelArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("GRE_1")
//!             .customerGreEndpoint("203.0.113.1")
//!             .cloudflareGreEndpoint("203.0.113.2")
//!             .interfaceAddress("192.0.2.0/31")
//!             .description("Tunnel for ISP X")
//!             .ttl(64)
//!             .mtu(1476)
//!             .healthCheckEnabled(true)
//!             .healthCheckTarget("203.0.113.1")
//!             .healthCheckType("reply")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:GreTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: GRE_1
//!       customerGreEndpoint: 203.0.113.1
//!       cloudflareGreEndpoint: 203.0.113.2
//!       interfaceAddress: 192.0.2.0/31
//!       description: Tunnel for ISP X
//!       ttl: 64
//!       mtu: 1476
//!       healthCheckEnabled: true
//!       healthCheckTarget: 203.0.113.1
//!       healthCheckType: reply
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/greTunnel:GreTunnel example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GreTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address assigned to the Cloudflare side of the GRE tunnel.
    #[builder(into)]
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// The IP address assigned to the customer side of the GRE tunnel.
    #[builder(into)]
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// Description of the GRE tunnel intent.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The IP address of the customer endpoint that will receive tunnel health checks.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    #[builder(into)]
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
    /// Name of the GRE tunnel.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Time To Live (TTL) in number of hops of the GRE tunnel.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct GreTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address assigned to the Cloudflare side of the GRE tunnel.
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// The IP address assigned to the customer side of the GRE tunnel.
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// Description of the GRE tunnel intent.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled.
    pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
    /// The IP address of the customer endpoint that will receive tunnel health checks.
    pub health_check_target: pulumi_wasm_rust::Output<String>,
    /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
    pub health_check_type: pulumi_wasm_rust::Output<String>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
    pub mtu: pulumi_wasm_rust::Output<i32>,
    /// Name of the GRE tunnel.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Time To Live (TTL) in number of hops of the GRE tunnel.
    pub ttl: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: GreTunnelArgs) -> GreTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::gre_tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::gre_tunnel::Args {
        account_id: &args.account_id.get_inner(),
        cloudflare_gre_endpoint: &args.cloudflare_gre_endpoint.get_inner(),
        customer_gre_endpoint: &args.customer_gre_endpoint.get_inner(),
        description: &args.description.get_inner(),
        health_check_enabled: &args.health_check_enabled.get_inner(),
        health_check_target: &args.health_check_target.get_inner(),
        health_check_type: &args.health_check_type.get_inner(),
        interface_address: &args.interface_address.get_inner(),
        mtu: &args.mtu.get_inner(),
        name: &args.name.get_inner(),
        ttl: &args.ttl.get_inner(),
    });

    GreTunnelResult {
        account_id: crate::into_domain(result.account_id),
        cloudflare_gre_endpoint: crate::into_domain(result.cloudflare_gre_endpoint),
        customer_gre_endpoint: crate::into_domain(result.customer_gre_endpoint),
        description: crate::into_domain(result.description),
        health_check_enabled: crate::into_domain(result.health_check_enabled),
        health_check_target: crate::into_domain(result.health_check_target),
        health_check_type: crate::into_domain(result.health_check_type),
        interface_address: crate::into_domain(result.interface_address),
        mtu: crate::into_domain(result.mtu),
        name: crate::into_domain(result.name),
        ttl: crate::into_domain(result.ttl),
    }
}
