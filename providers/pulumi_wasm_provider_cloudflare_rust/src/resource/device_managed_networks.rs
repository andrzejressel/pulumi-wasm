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
//! const managedNetworks = new cloudflare.DeviceManagedNetworks("managedNetworks", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     config: {
//!         sha256: "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
//!         tlsSockaddr: "foobar:1234",
//!     },
//!     name: "managed-network-1",
//!     type: "tls",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! managed_networks = cloudflare.DeviceManagedNetworks("managedNetworks",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     config=cloudflare.DeviceManagedNetworksConfigArgs(
//!         sha256="b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
//!         tls_sockaddr="foobar:1234",
//!     ),
//!     name="managed-network-1",
//!     type="tls")
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
//!     var managedNetworks = new Cloudflare.DeviceManagedNetworks("managedNetworks", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Config = new Cloudflare.Inputs.DeviceManagedNetworksConfigArgs
//!         {
//!             Sha256 = "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
//!             TlsSockaddr = "foobar:1234",
//!         },
//!         Name = "managed-network-1",
//!         Type = "tls",
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
//! 		_, err := cloudflare.NewDeviceManagedNetworks(ctx, "managedNetworks", &cloudflare.DeviceManagedNetworksArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Config: &cloudflare.DeviceManagedNetworksConfigArgs{
//! 				Sha256:      pulumi.String("b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"),
//! 				TlsSockaddr: pulumi.String("foobar:1234"),
//! 			},
//! 			Name: pulumi.String("managed-network-1"),
//! 			Type: pulumi.String("tls"),
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
//! import com.pulumi.cloudflare.DeviceManagedNetworks;
//! import com.pulumi.cloudflare.DeviceManagedNetworksArgs;
//! import com.pulumi.cloudflare.inputs.DeviceManagedNetworksConfigArgs;
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
//!         var managedNetworks = new DeviceManagedNetworks("managedNetworks", DeviceManagedNetworksArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .config(DeviceManagedNetworksConfigArgs.builder()
//!                 .sha256("b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c")
//!                 .tlsSockaddr("foobar:1234")
//!                 .build())
//!             .name("managed-network-1")
//!             .type("tls")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   managedNetworks:
//!     type: cloudflare:DeviceManagedNetworks
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       config:
//!         sha256: b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
//!         tlsSockaddr: foobar:1234
//!       name: managed-network-1
//!       type: tls
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks example <account_id>/<device_managed_networks_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DeviceManagedNetworksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceManagedNetworksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DeviceManagedNetworksArgs) -> DeviceManagedNetworksResult {
    let result = crate::bindings::pulumi::cloudflare::device_managed_networks::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_managed_networks::Args {
            account_id: &args.account_id.get_inner(),
            config: &args.config.get_inner(),
            name: &args.name.get_inner(),
            type_: &args.type_.get_inner(),
        },
    );

    DeviceManagedNetworksResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
