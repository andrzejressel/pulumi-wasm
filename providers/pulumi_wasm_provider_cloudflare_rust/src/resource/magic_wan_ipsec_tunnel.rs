//! Provides a resource, that manages IPsec tunnels for Magic Transit.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.MagicWanIpsecTunnel("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "IPsec_1",
//!     customerEndpoint: "203.0.113.1",
//!     cloudflareEndpoint: "203.0.113.1",
//!     interfaceAddress: "192.0.2.0/31",
//!     description: "Tunnel for ISP X",
//!     healthCheckEnabled: true,
//!     healthCheckTarget: "203.0.113.1",
//!     healthCheckType: "reply",
//!     psk: "asdf12341234",
//!     allowNullCipher: false,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.MagicWanIpsecTunnel("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="IPsec_1",
//!     customer_endpoint="203.0.113.1",
//!     cloudflare_endpoint="203.0.113.1",
//!     interface_address="192.0.2.0/31",
//!     description="Tunnel for ISP X",
//!     health_check_enabled=True,
//!     health_check_target="203.0.113.1",
//!     health_check_type="reply",
//!     psk="asdf12341234",
//!     allow_null_cipher=False)
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
//!     var example = new Cloudflare.MagicWanIpsecTunnel("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "IPsec_1",
//!         CustomerEndpoint = "203.0.113.1",
//!         CloudflareEndpoint = "203.0.113.1",
//!         InterfaceAddress = "192.0.2.0/31",
//!         Description = "Tunnel for ISP X",
//!         HealthCheckEnabled = true,
//!         HealthCheckTarget = "203.0.113.1",
//!         HealthCheckType = "reply",
//!         Psk = "asdf12341234",
//!         AllowNullCipher = false,
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
//! 		_, err := cloudflare.NewMagicWanIpsecTunnel(ctx, "example", &cloudflare.MagicWanIpsecTunnelArgs{
//! 			AccountId:          pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:               pulumi.String("IPsec_1"),
//! 			CustomerEndpoint:   pulumi.String("203.0.113.1"),
//! 			CloudflareEndpoint: pulumi.String("203.0.113.1"),
//! 			InterfaceAddress:   pulumi.String("192.0.2.0/31"),
//! 			Description:        pulumi.String("Tunnel for ISP X"),
//! 			HealthCheckEnabled: pulumi.Bool(true),
//! 			HealthCheckTarget:  pulumi.String("203.0.113.1"),
//! 			HealthCheckType:    pulumi.String("reply"),
//! 			Psk:                pulumi.String("asdf12341234"),
//! 			AllowNullCipher:    pulumi.Bool(false),
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
//! import com.pulumi.cloudflare.MagicWanIpsecTunnel;
//! import com.pulumi.cloudflare.MagicWanIpsecTunnelArgs;
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
//!         var example = new MagicWanIpsecTunnel("example", MagicWanIpsecTunnelArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("IPsec_1")
//!             .customerEndpoint("203.0.113.1")
//!             .cloudflareEndpoint("203.0.113.1")
//!             .interfaceAddress("192.0.2.0/31")
//!             .description("Tunnel for ISP X")
//!             .healthCheckEnabled(true)
//!             .healthCheckTarget("203.0.113.1")
//!             .healthCheckType("reply")
//!             .psk("asdf12341234")
//!             .allowNullCipher(false)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:MagicWanIpsecTunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: IPsec_1
//!       customerEndpoint: 203.0.113.1
//!       cloudflareEndpoint: 203.0.113.1
//!       interfaceAddress: 192.0.2.0/31
//!       description: Tunnel for ISP X
//!       healthCheckEnabled: true
//!       healthCheckTarget: 203.0.113.1
//!       healthCheckType: reply
//!       psk: asdf12341234
//!       allowNullCipher: false
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/magicWanIpsecTunnel:MagicWanIpsecTunnel example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct MagicWanIpsecTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_null_cipher: pulumi_wasm_rust::Output<Option<bool>>,
    /// IP address assigned to the Cloudflare side of the IPsec tunnel.
    #[builder(into)]
    pub cloudflare_endpoint: pulumi_wasm_rust::Output<String>,
    /// IP address assigned to the customer side of the IPsec tunnel.
    #[builder(into)]
    pub customer_endpoint: pulumi_wasm_rust::Output<String>,
    /// An optional description of the IPsec tunnel.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// `remote_id` in the form of a fqdn. This value is generated by cloudflare.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub fqdn_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies the direction for the health check. Available values: `unidirectional`, `bidirectional` Default: `unidirectional`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_direction: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled. Default: `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Specifies the ICMP rate for the health check. Available values: `low`, `mid`, `high` Default: `mid`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_rate: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
    /// `remote_id` as a hex string. This value is generated by cloudflare.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hex_id: pulumi_wasm_rust::Output<Option<String>>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    #[builder(into)]
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Name of the IPsec tunnel.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Pre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub psk: pulumi_wasm_rust::Output<Option<String>>,
    /// ID to be used while setting up the IPsec tunnel. This value is generated by cloudflare.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub remote_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if replay protection is enabled. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub replay_protection: pulumi_wasm_rust::Output<Option<bool>>,
    /// `remote_id` in the form of an email address. This value is generated by cloudflare.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub user_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct MagicWanIpsecTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if this tunnel may use a null cipher (ENCR_NULL) in Phase 2. Defaults to `false`.
    pub allow_null_cipher: pulumi_wasm_rust::Output<Option<bool>>,
    /// IP address assigned to the Cloudflare side of the IPsec tunnel.
    pub cloudflare_endpoint: pulumi_wasm_rust::Output<String>,
    /// IP address assigned to the customer side of the IPsec tunnel.
    pub customer_endpoint: pulumi_wasm_rust::Output<String>,
    /// An optional description of the IPsec tunnel.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// `remote_id` in the form of a fqdn. This value is generated by cloudflare.
    pub fqdn_id: pulumi_wasm_rust::Output<String>,
    /// Specifies the direction for the health check. Available values: `unidirectional`, `bidirectional` Default: `unidirectional`.
    pub health_check_direction: pulumi_wasm_rust::Output<String>,
    /// Specifies if ICMP tunnel health checks are enabled. Default: `true`.
    pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
    /// Specifies the ICMP rate for the health check. Available values: `low`, `mid`, `high` Default: `mid`.
    pub health_check_rate: pulumi_wasm_rust::Output<String>,
    /// The IP address of the customer endpoint that will receive tunnel health checks. Default: `<customer_gre_endpoint>`.
    pub health_check_target: pulumi_wasm_rust::Output<String>,
    /// Specifies the ICMP echo type for the health check (`request` or `reply`). Available values: `request`, `reply` Default: `reply`.
    pub health_check_type: pulumi_wasm_rust::Output<String>,
    /// `remote_id` as a hex string. This value is generated by cloudflare.
    pub hex_id: pulumi_wasm_rust::Output<String>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Name of the IPsec tunnel.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Pre shared key to be used with the IPsec tunnel. If left unset, it will be autogenerated.
    pub psk: pulumi_wasm_rust::Output<String>,
    /// ID to be used while setting up the IPsec tunnel. This value is generated by cloudflare.
    pub remote_id: pulumi_wasm_rust::Output<String>,
    /// Specifies if replay protection is enabled. Defaults to `false`.
    pub replay_protection: pulumi_wasm_rust::Output<Option<bool>>,
    /// `remote_id` in the form of an email address. This value is generated by cloudflare.
    pub user_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MagicWanIpsecTunnelArgs) -> MagicWanIpsecTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::magic_wan_ipsec_tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::magic_wan_ipsec_tunnel::Args {
        account_id: &args.account_id.get_inner(),
        allow_null_cipher: &args.allow_null_cipher.get_inner(),
        cloudflare_endpoint: &args.cloudflare_endpoint.get_inner(),
        customer_endpoint: &args.customer_endpoint.get_inner(),
        description: &args.description.get_inner(),
        fqdn_id: &args.fqdn_id.get_inner(),
        health_check_direction: &args.health_check_direction.get_inner(),
        health_check_enabled: &args.health_check_enabled.get_inner(),
        health_check_rate: &args.health_check_rate.get_inner(),
        health_check_target: &args.health_check_target.get_inner(),
        health_check_type: &args.health_check_type.get_inner(),
        hex_id: &args.hex_id.get_inner(),
        interface_address: &args.interface_address.get_inner(),
        name: &args.name.get_inner(),
        psk: &args.psk.get_inner(),
        remote_id: &args.remote_id.get_inner(),
        replay_protection: &args.replay_protection.get_inner(),
        user_id: &args.user_id.get_inner(),
    });

    MagicWanIpsecTunnelResult {
        account_id: crate::into_domain(result.account_id),
        allow_null_cipher: crate::into_domain(result.allow_null_cipher),
        cloudflare_endpoint: crate::into_domain(result.cloudflare_endpoint),
        customer_endpoint: crate::into_domain(result.customer_endpoint),
        description: crate::into_domain(result.description),
        fqdn_id: crate::into_domain(result.fqdn_id),
        health_check_direction: crate::into_domain(result.health_check_direction),
        health_check_enabled: crate::into_domain(result.health_check_enabled),
        health_check_rate: crate::into_domain(result.health_check_rate),
        health_check_target: crate::into_domain(result.health_check_target),
        health_check_type: crate::into_domain(result.health_check_type),
        hex_id: crate::into_domain(result.hex_id),
        interface_address: crate::into_domain(result.interface_address),
        name: crate::into_domain(result.name),
        psk: crate::into_domain(result.psk),
        remote_id: crate::into_domain(result.remote_id),
        replay_protection: crate::into_domain(result.replay_protection),
        user_id: crate::into_domain(result.user_id),
    }
}