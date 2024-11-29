//! The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
//! to manage Cloudflare Workers for Platforms namespaces.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! import * as std from "@pulumi/std";
//! 
//! const example = new cloudflare.WorkersForPlatformsNamespace("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example-namespace",
//! });
//! const customerWorker1 = new cloudflare.WorkersScript("customer_worker_1", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "customer-worker-1",
//!     content: std.file({
//!         input: "script.js",
//!     }).then(invoke => invoke.result),
//!     dispatchNamespace: example.name,
//!     tags: ["free"],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! import pulumi_std as std
//! 
//! example = cloudflare.WorkersForPlatformsNamespace("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example-namespace")
//! customer_worker1 = cloudflare.WorkersScript("customer_worker_1",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="customer-worker-1",
//!     content=std.file(input="script.js").result,
//!     dispatch_namespace=example.name,
//!     tags=["free"])
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! using Std = Pulumi.Std;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var example = new Cloudflare.WorkersForPlatformsNamespace("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example-namespace",
//!     });
//! 
//!     var customerWorker1 = new Cloudflare.WorkersScript("customer_worker_1", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "customer-worker-1",
//!         Content = Std.File.Invoke(new()
//!         {
//!             Input = "script.js",
//!         }).Apply(invoke => invoke.Result),
//!         DispatchNamespace = example.Name,
//!         Tags = new[]
//!         {
//!             "free",
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
//! 	"github.com/pulumi/pulumi-std/sdk/go/std"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		example, err := cloudflare.NewWorkersForPlatformsNamespace(ctx, "example", &cloudflare.WorkersForPlatformsNamespaceArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("example-namespace"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		invokeFile, err := std.File(ctx, &std.FileArgs{
//! 			Input: "script.js",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewWorkersScript(ctx, "customer_worker_1", &cloudflare.WorkersScriptArgs{
//! 			AccountId:         pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:              pulumi.String("customer-worker-1"),
//! 			Content:           pulumi.String(invokeFile.Result),
//! 			DispatchNamespace: example.Name,
//! 			Tags: pulumi.StringArray{
//! 				pulumi.String("free"),
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
//! import com.pulumi.cloudflare.WorkersForPlatformsNamespace;
//! import com.pulumi.cloudflare.WorkersForPlatformsNamespaceArgs;
//! import com.pulumi.cloudflare.WorkersScript;
//! import com.pulumi.cloudflare.WorkersScriptArgs;
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
//!         var example = new WorkersForPlatformsNamespace("example", WorkersForPlatformsNamespaceArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example-namespace")
//!             .build());
//! 
//!         var customerWorker1 = new WorkersScript("customerWorker1", WorkersScriptArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("customer-worker-1")
//!             .content(StdFunctions.file(FileArgs.builder()
//!                 .input("script.js")
//!                 .build()).result())
//!             .dispatchNamespace(example.name())
//!             .tags("free")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:WorkersForPlatformsNamespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example-namespace
//!   customerWorker1:
//!     type: cloudflare:WorkersScript
//!     name: customer_worker_1
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: customer-worker-1
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: script.js
//!           Return: result
//!       dispatchNamespace: ${example.name}
//!       tags:
//!         - free
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace example <account_id>/<namespace_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersForPlatformsNamespaceArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Workers for Platforms namespace.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersForPlatformsNamespaceResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Workers for Platforms namespace.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersForPlatformsNamespaceArgs) -> WorkersForPlatformsNamespaceResult {

    let result = crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::invoke(name, &crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    WorkersForPlatformsNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
