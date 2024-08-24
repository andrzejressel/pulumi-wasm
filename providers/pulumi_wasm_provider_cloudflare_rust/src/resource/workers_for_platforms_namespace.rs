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
//! import * as fs from "fs";
//!
//! const example = new cloudflare.WorkersForPlatformsNamespace("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example-namespace",
//! });
//! const customerWorker1 = new cloudflare.WorkerScript("customerWorker1", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "customer-worker-1",
//!     content: fs.readFileSync("script.js", "utf8"),
//!     dispatchNamespace: example.name,
//!     tags: ["free"],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.WorkersForPlatformsNamespace("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example-namespace")
//! customer_worker1 = cloudflare.WorkerScript("customerWorker1",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="customer-worker-1",
//!     content=(lambda path: open(path).read())("script.js"),
//!     dispatch_namespace=example.name,
//!     tags=["free"])
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.IO;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var example = new Cloudflare.WorkersForPlatformsNamespace("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example-namespace",
//!     });
//!
//!     var customerWorker1 = new Cloudflare.WorkerScript("customerWorker1", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "customer-worker-1",
//!         Content = File.ReadAllText("script.js"),
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
//! 	"os"
//!
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func readFileOrPanic(path string) pulumi.StringPtrInput {
//! 	data, err := os.ReadFile(path)
//! 	if err != nil {
//! 		panic(err.Error())
//! 	}
//! 	return pulumi.String(string(data))
//! }
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
//! 		_, err = cloudflare.NewWorkerScript(ctx, "customerWorker1", &cloudflare.WorkerScriptArgs{
//! 			AccountId:         pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:              pulumi.String("customer-worker-1"),
//! 			Content:           readFileOrPanic("script.js"),
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
//! import com.pulumi.cloudflare.WorkerScript;
//! import com.pulumi.cloudflare.WorkerScriptArgs;
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
//!         var customerWorker1 = new WorkerScript("customerWorker1", WorkerScriptArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("customer-worker-1")
//!             .content(Files.readString(Paths.get("script.js")))
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
//!     type: cloudflare:WorkerScript
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: customer-worker-1
//!       content:
//!         fn::readFile: script.js
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

pub struct WorkersForPlatformsNamespaceArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Workers for Platforms namespace.
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
pub fn create(
    name: &str,
    args: WorkersForPlatformsNamespaceArgs,
) -> WorkersForPlatformsNamespaceResult {
    let result = crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::Args {
            account_id: args.account_id.get_inner(),
            name: args.name.get_inner(),
        },
    );

    WorkersForPlatformsNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
