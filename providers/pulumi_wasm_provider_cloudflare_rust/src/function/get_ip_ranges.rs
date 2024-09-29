//! Use this data source to get the [IP ranges](https://www.cloudflare.com/ips/) of Cloudflare network.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! import * as example from "@pulumi/example";
//! 
//! const cloudflare = cloudflare.getIpRanges({});
//! const example = new example.index.Example_firewall_resource("example", {
//!     name: "from-cloudflare",
//!     network: "default",
//!     sourceRanges: cloudflare.ipv4CidrBlocks,
//!     allow: [{
//!         ports: "443",
//!         protocol: "tcp",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! import pulumi_example as example
//! 
//! cloudflare = cloudflare.get_ip_ranges()
//! example = example.index.Example_firewall_resource("example",
//!     name=from-cloudflare,
//!     network=default,
//!     source_ranges=cloudflare.ipv4_cidr_blocks,
//!     allow=[{
//!         ports: 443,
//!         protocol: tcp,
//!     }])
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! using Example = Pulumi.Example;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var cloudflare = Cloudflare.GetIpRanges.Invoke();
//! 
//!     var example = new Example.Index.Example_firewall_resource("example", new()
//!     {
//!         Name = "from-cloudflare",
//!         Network = "default",
//!         SourceRanges = cloudflare.Apply(getIpRangesResult => getIpRangesResult.Ipv4CidrBlocks),
//!         Allow = new[]
//!         {
//!             
//!             {
//!                 { "ports", "443" },
//!                 { "protocol", "tcp" },
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
//! 	"github.com/pulumi/pulumi-example/sdk/v1/go/example"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		cloudflare, err := cloudflare.GetIpRanges(ctx, nil, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = index.NewExample_firewall_resource(ctx, "example", &index.Example_firewall_resourceArgs{
//! 			Name:         "from-cloudflare",
//! 			Network:      "default",
//! 			SourceRanges: cloudflare.Ipv4CidrBlocks,
//! 			Allow: []map[string]interface{}{
//! 				map[string]interface{}{
//! 					"ports":    "443",
//! 					"protocol": "tcp",
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.example.example_firewall_resource;
//! import com.pulumi.example.Example_firewall_resourceArgs;
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
//!         final var cloudflare = CloudflareFunctions.getIpRanges();
//! 
//!         var example = new Example_firewall_resource("example", Example_firewall_resourceArgs.builder()        
//!             .name("from-cloudflare")
//!             .network("default")
//!             .sourceRanges(cloudflare.applyValue(getIpRangesResult -> getIpRangesResult.ipv4CidrBlocks()))
//!             .allow(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference))
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: example:example_firewall_resource
//!     properties:
//!       name: from-cloudflare
//!       network: default
//!       sourceRanges: ${cloudflare.ipv4CidrBlocks}
//!       allow:
//!         - ports: '443'
//!           protocol: tcp
//! variables:
//!   cloudflare:
//!     fn::invoke:
//!       Function: cloudflare:getIpRanges
//!       Arguments: {}
//! ```
//! <!--End PulumiCodeChooser -->


pub struct GetIpRangesResult {
    /// The lexically ordered list of only the IPv4 China CIDR blocks.
    pub china_ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of only the IPv6 China CIDR blocks.
    pub china_ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of all non-China CIDR blocks.
    pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The lexically ordered list of only the IPv4 CIDR blocks.
    pub ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of only the IPv6 CIDR blocks.
    pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> GetIpRangesResult {

    let result = crate::bindings::pulumi::cloudflare::get_ip_ranges::invoke(
    );

    GetIpRangesResult {
        china_ipv4_cidr_blocks: crate::into_domain(result.china_ipv4_cidr_blocks),
        china_ipv6_cidr_blocks: crate::into_domain(result.china_ipv6_cidr_blocks),
        cidr_blocks: crate::into_domain(result.cidr_blocks),
        id: crate::into_domain(result.id),
        ipv4_cidr_blocks: crate::into_domain(result.ipv4_cidr_blocks),
        ipv6_cidr_blocks: crate::into_domain(result.ipv6_cidr_blocks),
    }
}
