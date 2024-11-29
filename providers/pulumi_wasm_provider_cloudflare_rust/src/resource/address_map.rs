//! Provides the ability to manage IP addresses that can be used by DNS records when
//! they are proxied through Cloudflare.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.AddressMap("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     description: "My address map",
//!     defaultSni: "*.example.com",
//!     enabled: true,
//!     ips: [
//!         {
//!             ip: "192.0.2.1",
//!         },
//!         {
//!             ip: "203.0.113.1",
//!         },
//!     ],
//!     memberships: [
//!         {
//!             identifier: "92f17202ed8bd63d69a66b86a49a8f6b",
//!             kind: "account",
//!         },
//!         {
//!             identifier: "023e105f4ecef8ad9ca31a8372d0c353",
//!             kind: "zone",
//!         },
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.AddressMap("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     description="My address map",
//!     default_sni="*.example.com",
//!     enabled=True,
//!     ips=[
//!         {
//!             "ip": "192.0.2.1",
//!         },
//!         {
//!             "ip": "203.0.113.1",
//!         },
//!     ],
//!     memberships=[
//!         {
//!             "identifier": "92f17202ed8bd63d69a66b86a49a8f6b",
//!             "kind": "account",
//!         },
//!         {
//!             "identifier": "023e105f4ecef8ad9ca31a8372d0c353",
//!             "kind": "zone",
//!         },
//!     ])
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
//!     var example = new Cloudflare.AddressMap("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Description = "My address map",
//!         DefaultSni = "*.example.com",
//!         Enabled = true,
//!         Ips = new[]
//!         {
//!             new Cloudflare.Inputs.AddressMapIpArgs
//!             {
//!                 Ip = "192.0.2.1",
//!             },
//!             new Cloudflare.Inputs.AddressMapIpArgs
//!             {
//!                 Ip = "203.0.113.1",
//!             },
//!         },
//!         Memberships = new[]
//!         {
//!             new Cloudflare.Inputs.AddressMapMembershipArgs
//!             {
//!                 Identifier = "92f17202ed8bd63d69a66b86a49a8f6b",
//!                 Kind = "account",
//!             },
//!             new Cloudflare.Inputs.AddressMapMembershipArgs
//!             {
//!                 Identifier = "023e105f4ecef8ad9ca31a8372d0c353",
//!                 Kind = "zone",
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
//! 		_, err := cloudflare.NewAddressMap(ctx, "example", &cloudflare.AddressMapArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Description: pulumi.String("My address map"),
//! 			DefaultSni:  pulumi.String("*.example.com"),
//! 			Enabled:     pulumi.Bool(true),
//! 			Ips: cloudflare.AddressMapIpArray{
//! 				&cloudflare.AddressMapIpArgs{
//! 					Ip: pulumi.String("192.0.2.1"),
//! 				},
//! 				&cloudflare.AddressMapIpArgs{
//! 					Ip: pulumi.String("203.0.113.1"),
//! 				},
//! 			},
//! 			Memberships: cloudflare.AddressMapMembershipArray{
//! 				&cloudflare.AddressMapMembershipArgs{
//! 					Identifier: pulumi.String("92f17202ed8bd63d69a66b86a49a8f6b"),
//! 					Kind:       pulumi.String("account"),
//! 				},
//! 				&cloudflare.AddressMapMembershipArgs{
//! 					Identifier: pulumi.String("023e105f4ecef8ad9ca31a8372d0c353"),
//! 					Kind:       pulumi.String("zone"),
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
//! import com.pulumi.cloudflare.AddressMap;
//! import com.pulumi.cloudflare.AddressMapArgs;
//! import com.pulumi.cloudflare.inputs.AddressMapIpArgs;
//! import com.pulumi.cloudflare.inputs.AddressMapMembershipArgs;
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
//!         var example = new AddressMap("example", AddressMapArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("My address map")
//!             .defaultSni("*.example.com")
//!             .enabled(true)
//!             .ips(            
//!                 AddressMapIpArgs.builder()
//!                     .ip("192.0.2.1")
//!                     .build(),
//!                 AddressMapIpArgs.builder()
//!                     .ip("203.0.113.1")
//!                     .build())
//!             .memberships(            
//!                 AddressMapMembershipArgs.builder()
//!                     .identifier("92f17202ed8bd63d69a66b86a49a8f6b")
//!                     .kind("account")
//!                     .build(),
//!                 AddressMapMembershipArgs.builder()
//!                     .identifier("023e105f4ecef8ad9ca31a8372d0c353")
//!                     .kind("zone")
//!                     .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:AddressMap
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: My address map
//!       defaultSni: '*.example.com'
//!       enabled: true
//!       ips:
//!         - ip: 192.0.2.1
//!         - ip: 203.0.113.1
//!       memberships:
//!         - identifier: 92f17202ed8bd63d69a66b86a49a8f6b
//!           kind: account
//!         - identifier: 023e105f4ecef8ad9ca31a8372d0c353
//!           kind: zone
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/addressMap:AddressMap example <account_id>/<address_map_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AddressMapArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

pub struct AddressMapResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps.
    pub can_delete: pulumi_wasm_rust::Output<bool>,
    /// If set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps.
    pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AddressMapArgs) -> AddressMapResult {

    let result = crate::bindings::pulumi::cloudflare::address_map::invoke(name, &crate::bindings::pulumi::cloudflare::address_map::Args {
        account_id: &args.account_id.get_inner(),
        default_sni: &args.default_sni.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        ips: &args.ips.get_inner(),
        memberships: &args.memberships.get_inner(),
    });

    AddressMapResult {
        account_id: crate::into_domain(result.account_id),
        can_delete: crate::into_domain(result.can_delete),
        can_modify_ips: crate::into_domain(result.can_modify_ips),
        default_sni: crate::into_domain(result.default_sni),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        ips: crate::into_domain(result.ips),
        memberships: crate::into_domain(result.memberships),
    }
}
