//! Use this data source to lookup a [List](https://developers.cloudflare.com/api/operations/lists-get-lists).
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getList({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "list_name",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_list(account_id="f037e56e89293a057740de681ac9abbe",
//!     name="list_name")
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
//!     var example = Cloudflare.GetList.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "list_name",
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
//! 		_, err := cloudflare.LookupList(ctx, &cloudflare.LookupListArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 			Name:      "list_name",
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
//! import com.pulumi.cloudflare.inputs.GetListArgs;
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
//!         final var example = CloudflareFunctions.getList(GetListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("list_name")
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
//!       Function: cloudflare:getList
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         name: list_name
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// List description.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// List kind.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Number of items in list.
    pub numitems: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetListArgs) -> GetListResult {

    let result = crate::bindings::pulumi::cloudflare::get_list::invoke(&crate::bindings::pulumi::cloudflare::get_list::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    GetListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        id: crate::into_domain(result.id),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        numitems: crate::into_domain(result.numitems),
    }
}
