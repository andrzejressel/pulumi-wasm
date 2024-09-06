//! Provides a Cloudflare Spectrum Application. You can extend the power
//! of Cloudflare's DDoS, TLS, and IP Firewall to your other TCP-based
//! services.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.SpectrumApplication("example", {
//!     dns: {
//!         name: "ssh.example.com",
//!         type: "CNAME",
//!     },
//!     edgeIps: {
//!         ips: [
//!             "203.0.113.1",
//!             "203.0.113.2",
//!         ],
//!         type: "static",
//!     },
//!     originDirects: ["tcp://192.0.2.1:22"],
//!     protocol: "tcp/22",
//!     trafficType: "direct",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.SpectrumApplication("example",
//!     dns=cloudflare.SpectrumApplicationDnsArgs(
//!         name="ssh.example.com",
//!         type="CNAME",
//!     ),
//!     edge_ips=cloudflare.SpectrumApplicationEdgeIpsArgs(
//!         ips=[
//!             "203.0.113.1",
//!             "203.0.113.2",
//!         ],
//!         type="static",
//!     ),
//!     origin_directs=["tcp://192.0.2.1:22"],
//!     protocol="tcp/22",
//!     traffic_type="direct",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     var example = new Cloudflare.SpectrumApplication("example", new()
//!     {
//!         Dns = new Cloudflare.Inputs.SpectrumApplicationDnsArgs
//!         {
//!             Name = "ssh.example.com",
//!             Type = "CNAME",
//!         },
//!         EdgeIps = new Cloudflare.Inputs.SpectrumApplicationEdgeIpsArgs
//!         {
//!             Ips = new[]
//!             {
//!                 "203.0.113.1",
//!                 "203.0.113.2",
//!             },
//!             Type = "static",
//!         },
//!         OriginDirects = new[]
//!         {
//!             "tcp://192.0.2.1:22",
//!         },
//!         Protocol = "tcp/22",
//!         TrafficType = "direct",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
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
//! 		_, err := cloudflare.NewSpectrumApplication(ctx, "example", &cloudflare.SpectrumApplicationArgs{
//! 			Dns: &cloudflare.SpectrumApplicationDnsArgs{
//! 				Name: pulumi.String("ssh.example.com"),
//! 				Type: pulumi.String("CNAME"),
//! 			},
//! 			EdgeIps: &cloudflare.SpectrumApplicationEdgeIpsArgs{
//! 				Ips: pulumi.StringArray{
//! 					pulumi.String("203.0.113.1"),
//! 					pulumi.String("203.0.113.2"),
//! 				},
//! 				Type: pulumi.String("static"),
//! 			},
//! 			OriginDirects: pulumi.StringArray{
//! 				pulumi.String("tcp://192.0.2.1:22"),
//! 			},
//! 			Protocol:    pulumi.String("tcp/22"),
//! 			TrafficType: pulumi.String("direct"),
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.SpectrumApplication;
//! import com.pulumi.cloudflare.SpectrumApplicationArgs;
//! import com.pulumi.cloudflare.inputs.SpectrumApplicationDnsArgs;
//! import com.pulumi.cloudflare.inputs.SpectrumApplicationEdgeIpsArgs;
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
//!         var example = new SpectrumApplication("example", SpectrumApplicationArgs.builder()        
//!             .dns(SpectrumApplicationDnsArgs.builder()
//!                 .name("ssh.example.com")
//!                 .type("CNAME")
//!                 .build())
//!             .edgeIps(SpectrumApplicationEdgeIpsArgs.builder()
//!                 .ips(                
//!                     "203.0.113.1",
//!                     "203.0.113.2")
//!                 .type("static")
//!                 .build())
//!             .originDirects("tcp://192.0.2.1:22")
//!             .protocol("tcp/22")
//!             .trafficType("direct")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:SpectrumApplication
//!     properties:
//!       dns:
//!         name: ssh.example.com
//!         type: CNAME
//!       edgeIps:
//!         ips:
//!           - 203.0.113.1
//!           - 203.0.113.2
//!         type: static
//!       originDirects:
//!         - tcp://192.0.2.1:22
//!       protocol: tcp/22
//!       trafficType: direct
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/spectrumApplication:SpectrumApplication example <zone_id>/<spectrum_application_id>
//! ```
//!

pub struct SpectrumApplicationArgs {
    /// Enables Argo Smart Routing.
    pub argo_smart_routing: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name and type of DNS record for the Spectrum application.
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    /// The anycast edge IP configuration for the hostname of this application.
    pub edge_ips: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationEdgeIps>>,
    /// Enables the IP Firewall for this application.
    pub ip_firewall: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A destination DNS addresses to the origin.
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
    pub origin_port_range:
        pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
    /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
    pub protocol: pulumi_wasm_rust::Output<String>,
    /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
    pub proxy_protocol: pulumi_wasm_rust::Output<Option<String>>,
    /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
    pub tls: pulumi_wasm_rust::Output<Option<String>>,
    /// Sets application type. Available values: `direct`, `http`, `https`.
    pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct SpectrumApplicationResult {
    /// Enables Argo Smart Routing.
    pub argo_smart_routing: pulumi_wasm_rust::Output<bool>,
    /// The name and type of DNS record for the Spectrum application.
    pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
    /// The anycast edge IP configuration for the hostname of this application.
    pub edge_ips: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationEdgeIps>,
    /// Enables the IP Firewall for this application.
    pub ip_firewall: pulumi_wasm_rust::Output<bool>,
    /// A list of destination addresses to the origin. e.g. `tcp://192.0.2.1:22`.
    pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A destination DNS addresses to the origin.
    pub origin_dns: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
    /// Origin port to proxy traffice to. Conflicts with `origin_port_range`.
    pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Origin port range to proxy traffice to. When using a range, the protocol field must also specify a range, e.g. `tcp/22-23`. Conflicts with `origin_port`.
    pub origin_port_range:
        pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
    /// The port configuration at Cloudflare's edge. e.g. `tcp/22`.
    pub protocol: pulumi_wasm_rust::Output<String>,
    /// Enables a proxy protocol to the origin. Available values: `off`, `v1`, `v2`, `simple`.
    pub proxy_protocol: pulumi_wasm_rust::Output<String>,
    /// TLS configuration option for Cloudflare to connect to your origin. Available values: `off`, `flexible`, `full`, `strict`.
    pub tls: pulumi_wasm_rust::Output<String>,
    /// Sets application type. Available values: `direct`, `http`, `https`.
    pub traffic_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SpectrumApplicationArgs) -> SpectrumApplicationResult {
    let result = crate::bindings::pulumi::cloudflare::spectrum_application::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::spectrum_application::Args {
            argo_smart_routing: &args.argo_smart_routing.get_inner(),
            dns: &args.dns.get_inner(),
            edge_ips: &args.edge_ips.get_inner(),
            ip_firewall: &args.ip_firewall.get_inner(),
            origin_directs: &args.origin_directs.get_inner(),
            origin_dns: &args.origin_dns.get_inner(),
            origin_port: &args.origin_port.get_inner(),
            origin_port_range: &args.origin_port_range.get_inner(),
            protocol: &args.protocol.get_inner(),
            proxy_protocol: &args.proxy_protocol.get_inner(),
            tls: &args.tls.get_inner(),
            traffic_type: &args.traffic_type.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    SpectrumApplicationResult {
        argo_smart_routing: crate::into_domain(result.argo_smart_routing),
        dns: crate::into_domain(result.dns),
        edge_ips: crate::into_domain(result.edge_ips),
        ip_firewall: crate::into_domain(result.ip_firewall),
        origin_directs: crate::into_domain(result.origin_directs),
        origin_dns: crate::into_domain(result.origin_dns),
        origin_port: crate::into_domain(result.origin_port),
        origin_port_range: crate::into_domain(result.origin_port_range),
        protocol: crate::into_domain(result.protocol),
        proxy_protocol: crate::into_domain(result.proxy_protocol),
        tls: crate::into_domain(result.tls),
        traffic_type: crate::into_domain(result.traffic_type),
        zone_id: crate::into_domain(result.zone_id),
    }
}
