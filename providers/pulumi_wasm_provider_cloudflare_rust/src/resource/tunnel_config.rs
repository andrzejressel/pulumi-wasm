//! Provides a Cloudflare Tunnel configuration resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const exampleTunnel = new cloudflare.Tunnel("exampleTunnel", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example_tunnel",
//!     secret: "<32 character secret>",
//! });
//! const exampleConfig = new cloudflare.TunnelConfig("exampleConfig", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     tunnelId: exampleTunnel.id,
//!     config: {
//!         warpRouting: {
//!             enabled: true,
//!         },
//!         originRequest: {
//!             connectTimeout: "1m0s",
//!             tlsTimeout: "1m0s",
//!             tcpKeepAlive: "1m0s",
//!             noHappyEyeballs: false,
//!             keepAliveConnections: 1024,
//!             keepAliveTimeout: "1m0s",
//!             httpHostHeader: "baz",
//!             originServerName: "foobar",
//!             caPool: "/path/to/unsigned/ca/pool",
//!             noTlsVerify: false,
//!             disableChunkedEncoding: false,
//!             bastionMode: false,
//!             proxyAddress: "10.0.0.1",
//!             proxyPort: 8123,
//!             proxyType: "socks",
//!             ipRules: [{
//!                 prefix: "/web",
//!                 ports: [
//!                     80,
//!                     443,
//!                 ],
//!                 allow: false,
//!             }],
//!         },
//!         ingressRules: [
//!             {
//!                 hostname: "foo",
//!                 path: "/bar",
//!                 service: "http://10.0.0.2:8080",
//!                 originRequest: {
//!                     connectTimeout: "2m0s",
//!                     access: {
//!                         required: true,
//!                         teamName: "terraform",
//!                         audTags: ["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"],
//!                     },
//!                 },
//!             },
//!             {
//!                 service: "https://10.0.0.3:8081",
//!             },
//!         ],
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example_tunnel = cloudflare.Tunnel("exampleTunnel",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example_tunnel",
//!     secret="<32 character secret>")
//! example_config = cloudflare.TunnelConfig("exampleConfig",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     tunnel_id=example_tunnel.id,
//!     config=cloudflare.TunnelConfigConfigArgs(
//!         warp_routing=cloudflare.TunnelConfigConfigWarpRoutingArgs(
//!             enabled=True,
//!         ),
//!         origin_request=cloudflare.TunnelConfigConfigOriginRequestArgs(
//!             connect_timeout="1m0s",
//!             tls_timeout="1m0s",
//!             tcp_keep_alive="1m0s",
//!             no_happy_eyeballs=False,
//!             keep_alive_connections=1024,
//!             keep_alive_timeout="1m0s",
//!             http_host_header="baz",
//!             origin_server_name="foobar",
//!             ca_pool="/path/to/unsigned/ca/pool",
//!             no_tls_verify=False,
//!             disable_chunked_encoding=False,
//!             bastion_mode=False,
//!             proxy_address="10.0.0.1",
//!             proxy_port=8123,
//!             proxy_type="socks",
//!             ip_rules=[cloudflare.TunnelConfigConfigOriginRequestIpRuleArgs(
//!                 prefix="/web",
//!                 ports=[
//!                     80,
//!                     443,
//!                 ],
//!                 allow=False,
//!             )],
//!         ),
//!         ingress_rules=[
//!             cloudflare.TunnelConfigConfigIngressRuleArgs(
//!                 hostname="foo",
//!                 path="/bar",
//!                 service="http://10.0.0.2:8080",
//!                 origin_request=cloudflare.TunnelConfigConfigIngressRuleOriginRequestArgs(
//!                     connect_timeout="2m0s",
//!                     access=cloudflare.TunnelConfigConfigIngressRuleOriginRequestAccessArgs(
//!                         required=True,
//!                         team_name="terraform",
//!                         aud_tags=["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"],
//!                     ),
//!                 ),
//!             ),
//!             cloudflare.TunnelConfigConfigIngressRuleArgs(
//!                 service="https://10.0.0.3:8081",
//!             ),
//!         ],
//!     ))
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
//!     var exampleTunnel = new Cloudflare.Tunnel("exampleTunnel", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example_tunnel",
//!         Secret = "<32 character secret>",
//!     });
//!
//!     var exampleConfig = new Cloudflare.TunnelConfig("exampleConfig", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         TunnelId = exampleTunnel.Id,
//!         Config = new Cloudflare.Inputs.TunnelConfigConfigArgs
//!         {
//!             WarpRouting = new Cloudflare.Inputs.TunnelConfigConfigWarpRoutingArgs
//!             {
//!                 Enabled = true,
//!             },
//!             OriginRequest = new Cloudflare.Inputs.TunnelConfigConfigOriginRequestArgs
//!             {
//!                 ConnectTimeout = "1m0s",
//!                 TlsTimeout = "1m0s",
//!                 TcpKeepAlive = "1m0s",
//!                 NoHappyEyeballs = false,
//!                 KeepAliveConnections = 1024,
//!                 KeepAliveTimeout = "1m0s",
//!                 HttpHostHeader = "baz",
//!                 OriginServerName = "foobar",
//!                 CaPool = "/path/to/unsigned/ca/pool",
//!                 NoTlsVerify = false,
//!                 DisableChunkedEncoding = false,
//!                 BastionMode = false,
//!                 ProxyAddress = "10.0.0.1",
//!                 ProxyPort = 8123,
//!                 ProxyType = "socks",
//!                 IpRules = new[]
//!                 {
//!                     new Cloudflare.Inputs.TunnelConfigConfigOriginRequestIpRuleArgs
//!                     {
//!                         Prefix = "/web",
//!                         Ports = new[]
//!                         {
//!                             80,
//!                             443,
//!                         },
//!                         Allow = false,
//!                     },
//!                 },
//!             },
//!             IngressRules = new[]
//!             {
//!                 new Cloudflare.Inputs.TunnelConfigConfigIngressRuleArgs
//!                 {
//!                     Hostname = "foo",
//!                     Path = "/bar",
//!                     Service = "http://10.0.0.2:8080",
//!                     OriginRequest = new Cloudflare.Inputs.TunnelConfigConfigIngressRuleOriginRequestArgs
//!                     {
//!                         ConnectTimeout = "2m0s",
//!                         Access = new Cloudflare.Inputs.TunnelConfigConfigIngressRuleOriginRequestAccessArgs
//!                         {
//!                             Required = true,
//!                             TeamName = "terraform",
//!                             AudTags = new[]
//!                             {
//!                                 "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
//!                             },
//!                         },
//!                     },
//!                 },
//!                 new Cloudflare.Inputs.TunnelConfigConfigIngressRuleArgs
//!                 {
//!                     Service = "https://10.0.0.3:8081",
//!                 },
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
//! 		exampleTunnel, err := cloudflare.NewTunnel(ctx, "exampleTunnel", &cloudflare.TunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("example_tunnel"),
//! 			Secret:    pulumi.String("<32 character secret>"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewTunnelConfig(ctx, "exampleConfig", &cloudflare.TunnelConfigArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			TunnelId:  exampleTunnel.ID(),
//! 			Config: &cloudflare.TunnelConfigConfigArgs{
//! 				WarpRouting: &cloudflare.TunnelConfigConfigWarpRoutingArgs{
//! 					Enabled: pulumi.Bool(true),
//! 				},
//! 				OriginRequest: &cloudflare.TunnelConfigConfigOriginRequestArgs{
//! 					ConnectTimeout:         pulumi.String("1m0s"),
//! 					TlsTimeout:             pulumi.String("1m0s"),
//! 					TcpKeepAlive:           pulumi.String("1m0s"),
//! 					NoHappyEyeballs:        pulumi.Bool(false),
//! 					KeepAliveConnections:   pulumi.Int(1024),
//! 					KeepAliveTimeout:       pulumi.String("1m0s"),
//! 					HttpHostHeader:         pulumi.String("baz"),
//! 					OriginServerName:       pulumi.String("foobar"),
//! 					CaPool:                 pulumi.String("/path/to/unsigned/ca/pool"),
//! 					NoTlsVerify:            pulumi.Bool(false),
//! 					DisableChunkedEncoding: pulumi.Bool(false),
//! 					BastionMode:            pulumi.Bool(false),
//! 					ProxyAddress:           pulumi.String("10.0.0.1"),
//! 					ProxyPort:              pulumi.Int(8123),
//! 					ProxyType:              pulumi.String("socks"),
//! 					IpRules: cloudflare.TunnelConfigConfigOriginRequestIpRuleArray{
//! 						&cloudflare.TunnelConfigConfigOriginRequestIpRuleArgs{
//! 							Prefix: pulumi.String("/web"),
//! 							Ports: pulumi.IntArray{
//! 								pulumi.Int(80),
//! 								pulumi.Int(443),
//! 							},
//! 							Allow: pulumi.Bool(false),
//! 						},
//! 					},
//! 				},
//! 				IngressRules: cloudflare.TunnelConfigConfigIngressRuleArray{
//! 					&cloudflare.TunnelConfigConfigIngressRuleArgs{
//! 						Hostname: pulumi.String("foo"),
//! 						Path:     pulumi.String("/bar"),
//! 						Service:  pulumi.String("http://10.0.0.2:8080"),
//! 						OriginRequest: &cloudflare.TunnelConfigConfigIngressRuleOriginRequestArgs{
//! 							ConnectTimeout: pulumi.String("2m0s"),
//! 							Access: &cloudflare.TunnelConfigConfigIngressRuleOriginRequestAccessArgs{
//! 								Required: pulumi.Bool(true),
//! 								TeamName: pulumi.String("terraform"),
//! 								AudTags: pulumi.StringArray{
//! 									pulumi.String("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
//! 								},
//! 							},
//! 						},
//! 					},
//! 					&cloudflare.TunnelConfigConfigIngressRuleArgs{
//! 						Service: pulumi.String("https://10.0.0.3:8081"),
//! 					},
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
//! import com.pulumi.cloudflare.Tunnel;
//! import com.pulumi.cloudflare.TunnelArgs;
//! import com.pulumi.cloudflare.TunnelConfig;
//! import com.pulumi.cloudflare.TunnelConfigArgs;
//! import com.pulumi.cloudflare.inputs.TunnelConfigConfigArgs;
//! import com.pulumi.cloudflare.inputs.TunnelConfigConfigWarpRoutingArgs;
//! import com.pulumi.cloudflare.inputs.TunnelConfigConfigOriginRequestArgs;
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
//!         var exampleTunnel = new Tunnel("exampleTunnel", TunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example_tunnel")
//!             .secret("<32 character secret>")
//!             .build());
//!
//!         var exampleConfig = new TunnelConfig("exampleConfig", TunnelConfigArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .tunnelId(exampleTunnel.id())
//!             .config(TunnelConfigConfigArgs.builder()
//!                 .warpRouting(TunnelConfigConfigWarpRoutingArgs.builder()
//!                     .enabled(true)
//!                     .build())
//!                 .originRequest(TunnelConfigConfigOriginRequestArgs.builder()
//!                     .connectTimeout("1m0s")
//!                     .tlsTimeout("1m0s")
//!                     .tcpKeepAlive("1m0s")
//!                     .noHappyEyeballs(false)
//!                     .keepAliveConnections(1024)
//!                     .keepAliveTimeout("1m0s")
//!                     .httpHostHeader("baz")
//!                     .originServerName("foobar")
//!                     .caPool("/path/to/unsigned/ca/pool")
//!                     .noTlsVerify(false)
//!                     .disableChunkedEncoding(false)
//!                     .bastionMode(false)
//!                     .proxyAddress("10.0.0.1")
//!                     .proxyPort("8123")
//!                     .proxyType("socks")
//!                     .ipRules(TunnelConfigConfigOriginRequestIpRuleArgs.builder()
//!                         .prefix("/web")
//!                         .ports(                        
//!                             80,
//!                             443)
//!                         .allow(false)
//!                         .build())
//!                     .build())
//!                 .ingressRules(                
//!                     TunnelConfigConfigIngressRuleArgs.builder()
//!                         .hostname("foo")
//!                         .path("/bar")
//!                         .service("http://10.0.0.2:8080")
//!                         .originRequest(TunnelConfigConfigIngressRuleOriginRequestArgs.builder()
//!                             .connectTimeout("2m0s")
//!                             .access(TunnelConfigConfigIngressRuleOriginRequestAccessArgs.builder()
//!                                 .required(true)
//!                                 .teamName("terraform")
//!                                 .audTags("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
//!                                 .build())
//!                             .build())
//!                         .build(),
//!                     TunnelConfigConfigIngressRuleArgs.builder()
//!                         .service("https://10.0.0.3:8081")
//!                         .build())
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleTunnel:
//!     type: cloudflare:Tunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_tunnel
//!       secret: <32 character secret>
//!   exampleConfig:
//!     type: cloudflare:TunnelConfig
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       tunnelId: ${exampleTunnel.id}
//!       config:
//!         warpRouting:
//!           enabled: true
//!         originRequest:
//!           connectTimeout: 1m0s
//!           tlsTimeout: 1m0s
//!           tcpKeepAlive: 1m0s
//!           noHappyEyeballs: false
//!           keepAliveConnections: 1024
//!           keepAliveTimeout: 1m0s
//!           httpHostHeader: baz
//!           originServerName: foobar
//!           caPool: /path/to/unsigned/ca/pool
//!           noTlsVerify: false
//!           disableChunkedEncoding: false
//!           bastionMode: false
//!           proxyAddress: 10.0.0.1
//!           proxyPort: '8123'
//!           proxyType: socks
//!           ipRules:
//!             - prefix: /web
//!               ports:
//!                 - 80
//!                 - 443
//!               allow: false
//!         ingressRules:
//!           - hostname: foo
//!             path: /bar
//!             service: http://10.0.0.2:8080
//!             originRequest:
//!               connectTimeout: 2m0s
//!               access:
//!                 required: true
//!                 teamName: terraform
//!                 audTags:
//!                   - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
//!           - service: https://10.0.0.3:8081
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/tunnelConfig:TunnelConfig example <account_id>/<tunnel_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TunnelConfigArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    #[builder(into)]
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelConfigResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TunnelConfigArgs) -> TunnelConfigResult {
    let result = crate::bindings::pulumi::cloudflare::tunnel_config::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tunnel_config::Args {
            account_id: &args.account_id.get_inner(),
            config: &args.config.get_inner(),
            tunnel_id: &args.tunnel_id.get_inner(),
        },
    );

    TunnelConfigResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        tunnel_id: crate::into_domain(result.tunnel_id),
    }
}
