//! Provides a Cloudflare Teams Location resource. Teams Locations are
//! referenced when creating secure web gateway policies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ZeroTrustDnsLocation("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "office",
//!     clientDefault: true,
//!     ecsSupport: false,
//!     networks: [
//!         {
//!             network: "203.0.113.1/32",
//!         },
//!         {
//!             network: "203.0.113.2/32",
//!         },
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ZeroTrustDnsLocation("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="office",
//!     client_default=True,
//!     ecs_support=False,
//!     networks=[
//!         {
//!             "network": "203.0.113.1/32",
//!         },
//!         {
//!             "network": "203.0.113.2/32",
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
//!     var example = new Cloudflare.ZeroTrustDnsLocation("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "office",
//!         ClientDefault = true,
//!         EcsSupport = false,
//!         Networks = new[]
//!         {
//!             new Cloudflare.Inputs.ZeroTrustDnsLocationNetworkArgs
//!             {
//!                 Network = "203.0.113.1/32",
//!             },
//!             new Cloudflare.Inputs.ZeroTrustDnsLocationNetworkArgs
//!             {
//!                 Network = "203.0.113.2/32",
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
//! 		_, err := cloudflare.NewZeroTrustDnsLocation(ctx, "example", &cloudflare.ZeroTrustDnsLocationArgs{
//! 			AccountId:     pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:          pulumi.String("office"),
//! 			ClientDefault: pulumi.Bool(true),
//! 			EcsSupport:    pulumi.Bool(false),
//! 			Networks: cloudflare.ZeroTrustDnsLocationNetworkArray{
//! 				&cloudflare.ZeroTrustDnsLocationNetworkArgs{
//! 					Network: pulumi.String("203.0.113.1/32"),
//! 				},
//! 				&cloudflare.ZeroTrustDnsLocationNetworkArgs{
//! 					Network: pulumi.String("203.0.113.2/32"),
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
//! import com.pulumi.cloudflare.ZeroTrustDnsLocation;
//! import com.pulumi.cloudflare.ZeroTrustDnsLocationArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustDnsLocationNetworkArgs;
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
//!         var example = new ZeroTrustDnsLocation("example", ZeroTrustDnsLocationArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("office")
//!             .clientDefault(true)
//!             .ecsSupport(false)
//!             .networks(            
//!                 ZeroTrustDnsLocationNetworkArgs.builder()
//!                     .network("203.0.113.1/32")
//!                     .build(),
//!                 ZeroTrustDnsLocationNetworkArgs.builder()
//!                     .network("203.0.113.2/32")
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
//!     type: cloudflare:ZeroTrustDnsLocation
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: office
//!       clientDefault: true
//!       ecsSupport: false
//!       networks:
//!         - network: 203.0.113.1/32
//!         - network: 203.0.113.2/32
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDnsLocation:ZeroTrustDnsLocation example <account_id>/<teams_location_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDnsLocationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicator that this is the default location.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    /// Indicator that this location needs to resolve EDNS queries.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the teams location.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that comprise the location.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDnsLocationNetwork>>>,
}

pub struct ZeroTrustDnsLocationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicator that anonymized logs are enabled.
    pub anonymized_logs_enabled: pulumi_wasm_rust::Output<bool>,
    /// Indicator that this is the default location.
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    /// The FQDN that DoH clients should be pointed at.
    pub doh_subdomain: pulumi_wasm_rust::Output<String>,
    /// Indicator that this location needs to resolve EDNS queries.
    pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
    /// Client IP address.
    pub ip: pulumi_wasm_rust::Output<String>,
    /// IP to direct all IPv4 DNS queries to.
    pub ipv4_destination: pulumi_wasm_rust::Output<String>,
    /// Name of the teams location.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that comprise the location.
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustDnsLocationNetwork>>>,
    pub policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustDnsLocationArgs) -> ZeroTrustDnsLocationResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_dns_location::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_dns_location::Args {
        account_id: &args.account_id.get_inner(),
        client_default: &args.client_default.get_inner(),
        ecs_support: &args.ecs_support.get_inner(),
        name: &args.name.get_inner(),
        networks: &args.networks.get_inner(),
    });

    ZeroTrustDnsLocationResult {
        account_id: crate::into_domain(result.account_id),
        anonymized_logs_enabled: crate::into_domain(result.anonymized_logs_enabled),
        client_default: crate::into_domain(result.client_default),
        doh_subdomain: crate::into_domain(result.doh_subdomain),
        ecs_support: crate::into_domain(result.ecs_support),
        ip: crate::into_domain(result.ip),
        ipv4_destination: crate::into_domain(result.ipv4_destination),
        name: crate::into_domain(result.name),
        networks: crate::into_domain(result.networks),
        policy_ids: crate::into_domain(result.policy_ids),
    }
}
