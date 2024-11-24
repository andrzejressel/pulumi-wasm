//! Provides the ability to manage Cloudflare Workers KV Namespace features.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.WorkersKvNamespace("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     title: "test-namespace",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.WorkersKvNamespace("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     title="test-namespace")
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
//!     var example = new Cloudflare.WorkersKvNamespace("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Title = "test-namespace",
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
//! 		_, err := cloudflare.NewWorkersKvNamespace(ctx, "example", &cloudflare.WorkersKvNamespaceArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Title:     pulumi.String("test-namespace"),
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
//! import com.pulumi.cloudflare.WorkersKvNamespace;
//! import com.pulumi.cloudflare.WorkersKvNamespaceArgs;
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
//!         var example = new WorkersKvNamespace("example", WorkersKvNamespaceArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .title("test-namespace")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:WorkersKvNamespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       title: test-namespace
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersKvNamespace:WorkersKvNamespace example <account_id>/<namespace_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersKvNamespaceArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    #[builder(into)]
    pub title: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvNamespaceResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    pub title: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersKvNamespaceArgs) -> WorkersKvNamespaceResult {

    let result = crate::bindings::pulumi::cloudflare::workers_kv_namespace::invoke(name, &crate::bindings::pulumi::cloudflare::workers_kv_namespace::Args {
        account_id: &args.account_id.get_inner(),
        title: &args.title.get_inner(),
    });

    WorkersKvNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        title: crate::into_domain(result.title),
    }
}
