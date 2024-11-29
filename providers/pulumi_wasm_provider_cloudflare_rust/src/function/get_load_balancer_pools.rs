//! A datasource to find Load Balancer Pools.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getLoadBalancerPools({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     filter: {
//!         name: "example-lb-pool",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_load_balancer_pools(account_id="f037e56e89293a057740de681ac9abbe",
//!     filter={
//!         "name": "example-lb-pool",
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
//!     var example = Cloudflare.GetLoadBalancerPools.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Filter = new Cloudflare.Inputs.GetLoadBalancerPoolsFilterInputArgs
//!         {
//!             Name = "example-lb-pool",
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
//! 		_, err := cloudflare.GetLoadBalancerPools(ctx, &cloudflare.GetLoadBalancerPoolsArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 			Filter: cloudflare.GetLoadBalancerPoolsFilter{
//! 				Name: pulumi.StringRef("example-lb-pool"),
//! 			},
//! 		}, nil)
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
//! import com.pulumi.cloudflare.inputs.GetLoadBalancerPoolsArgs;
//! import com.pulumi.cloudflare.inputs.GetLoadBalancerPoolsFilterArgs;
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
//!         final var example = CloudflareFunctions.getLoadBalancerPools(GetLoadBalancerPoolsArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .filter(GetLoadBalancerPoolsFilterArgs.builder()
//!                 .name("example-lb-pool")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getLoadBalancerPools
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         filter:
//!           name: example-lb-pool
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsArgs {
    /// The account identifier to target for the datasource lookups.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetLoadBalancerPoolsFilter>>,
    /// A list of Load Balancer Pools details.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::GetLoadBalancerPoolsPool>>>,
}

pub struct GetLoadBalancerPoolsResult {
    /// The account identifier to target for the datasource lookups.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetLoadBalancerPoolsFilter>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of Load Balancer Pools details.
    pub pools: pulumi_wasm_rust::Output<Vec<crate::types::GetLoadBalancerPoolsPool>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetLoadBalancerPoolsArgs
) -> GetLoadBalancerPoolsResult {

    let result = crate::bindings::pulumi::cloudflare::get_load_balancer_pools::invoke(
        &crate::bindings::pulumi::cloudflare::get_load_balancer_pools::Args {
                account_id: &args.account_id.get_inner(),
                filter: &args.filter.get_inner(),
                pools: &args.pools.get_inner(),
        }
    );

    GetLoadBalancerPoolsResult {
        account_id: crate::into_domain(result.account_id),
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        pools: crate::into_domain(result.pools),
    }
}
