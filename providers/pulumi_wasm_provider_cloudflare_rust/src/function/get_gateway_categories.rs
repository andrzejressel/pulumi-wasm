//! Use this data source to retrieve all Gateway categories for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getGatewayCategories({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_gateway_categories(account_id="f037e56e89293a057740de681ac9abbe")
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
//!     var example = Cloudflare.GetGatewayCategories.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
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
//! 		_, err := cloudflare.GetGatewayCategories(ctx, &cloudflare.GetGatewayCategoriesArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
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
//! import com.pulumi.cloudflare.inputs.GetGatewayCategoriesArgs;
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
//!         final var example = CloudflareFunctions.getGatewayCategories(GetGatewayCategoriesArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
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
//!       Function: cloudflare:getGatewayCategories
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayCategoriesArgs {
    /// The account ID to fetch Gateway Categories from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetGatewayCategoriesResult {
    /// The account ID to fetch Gateway Categories from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of Gateway Categories.
    pub categories: pulumi_wasm_rust::Output<Vec<crate::types::GetGatewayCategoriesCategory>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetGatewayCategoriesArgs
) -> GetGatewayCategoriesResult {

    let result = crate::bindings::pulumi::cloudflare::get_gateway_categories::invoke(
        &crate::bindings::pulumi::cloudflare::get_gateway_categories::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetGatewayCategoriesResult {
        account_id: crate::into_domain(result.account_id),
        categories: crate::into_domain(result.categories),
        id: crate::into_domain(result.id),
    }
}
