//! Use this data source to retrieve all Gateway application types for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getGatewayAppTypes({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_gateway_app_types(account_id="f037e56e89293a057740de681ac9abbe")
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
//!     var example = Cloudflare.GetGatewayAppTypes.Invoke(new()
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
//! 		_, err := cloudflare.GetGatewayAppTypes(ctx, &cloudflare.GetGatewayAppTypesArgs{
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
//! import com.pulumi.cloudflare.inputs.GetGatewayAppTypesArgs;
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
//!         final var example = CloudflareFunctions.getGatewayAppTypes(GetGatewayAppTypesArgs.builder()
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
//!       Function: cloudflare:getGatewayAppTypes
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayAppTypesArgs {
    /// The account ID to fetch Gateway App Types from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetGatewayAppTypesResult {
    /// The account ID to fetch Gateway App Types from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of Gateway App Types.
    pub app_types: pulumi_wasm_rust::Output<Vec<crate::types::GetGatewayAppTypesAppType>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetGatewayAppTypesArgs
) -> GetGatewayAppTypesResult {

    let result = crate::bindings::pulumi::cloudflare::get_gateway_app_types::invoke(
        &crate::bindings::pulumi::cloudflare::get_gateway_app_types::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetGatewayAppTypesResult {
        account_id: crate::into_domain(result.account_id),
        app_types: crate::into_domain(result.app_types),
        id: crate::into_domain(result.id),
    }
}
