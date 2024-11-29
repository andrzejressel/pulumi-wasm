//! Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const managedNetworks = new cloudflare.ZeroTrustDeviceManagedNetworks("managed_networks", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "managed-network-1",
//!     type: "tls",
//!     config: {
//!         tlsSockaddr: "foobar:1234",
//!         sha256: "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! managed_networks = cloudflare.ZeroTrustDeviceManagedNetworks("managed_networks",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="managed-network-1",
//!     type="tls",
//!     config={
//!         "tls_sockaddr": "foobar:1234",
//!         "sha256": "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
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
//!     var managedNetworks = new Cloudflare.ZeroTrustDeviceManagedNetworks("managed_networks", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "managed-network-1",
//!         Type = "tls",
//!         Config = new Cloudflare.Inputs.ZeroTrustDeviceManagedNetworksConfigArgs
//!         {
//!             TlsSockaddr = "foobar:1234",
//!             Sha256 = "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
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
//! 		_, err := cloudflare.NewZeroTrustDeviceManagedNetworks(ctx, "managed_networks", &cloudflare.ZeroTrustDeviceManagedNetworksArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("managed-network-1"),
//! 			Type:      pulumi.String("tls"),
//! 			Config: &cloudflare.ZeroTrustDeviceManagedNetworksConfigArgs{
//! 				TlsSockaddr: pulumi.String("foobar:1234"),
//! 				Sha256:      pulumi.String("b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"),
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
//! import com.pulumi.cloudflare.ZeroTrustDeviceManagedNetworks;
//! import com.pulumi.cloudflare.ZeroTrustDeviceManagedNetworksArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustDeviceManagedNetworksConfigArgs;
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
//!         var managedNetworks = new ZeroTrustDeviceManagedNetworks("managedNetworks", ZeroTrustDeviceManagedNetworksArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("managed-network-1")
//!             .type("tls")
//!             .config(ZeroTrustDeviceManagedNetworksConfigArgs.builder()
//!                 .tlsSockaddr("foobar:1234")
//!                 .sha256("b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   managedNetworks:
//!     type: cloudflare:ZeroTrustDeviceManagedNetworks
//!     name: managed_networks
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: managed-network-1
//!       type: tls
//!       config:
//!         tlsSockaddr: foobar:1234
//!         sha256: b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDeviceManagedNetworks:ZeroTrustDeviceManagedNetworks example <account_id>/<device_managed_networks_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDeviceManagedNetworksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<crate::types::ZeroTrustDeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustDeviceManagedNetworksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    pub config: pulumi_wasm_rust::Output<crate::types::ZeroTrustDeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustDeviceManagedNetworksArgs) -> ZeroTrustDeviceManagedNetworksResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_device_managed_networks::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_device_managed_networks::Args {
        account_id: &args.account_id.get_inner(),
        config: &args.config.get_inner(),
        name: &args.name.get_inner(),
        type_: &args.type_.get_inner(),
    });

    ZeroTrustDeviceManagedNetworksResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
