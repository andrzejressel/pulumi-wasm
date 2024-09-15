//! Provides the ability to manage Cloudflare Workers Queue features.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.Queue("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "my-queue",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.Queue("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="my-queue")
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
//!     var example = new Cloudflare.Queue("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "my-queue",
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
//! 		_, err := cloudflare.NewQueue(ctx, "example", &cloudflare.QueueArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("my-queue"),
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
//! import com.pulumi.cloudflare.Queue;
//! import com.pulumi.cloudflare.QueueArgs;
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
//!         var example = new Queue("example", QueueArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("my-queue")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:Queue
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: my-queue
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/queue:Queue example <account_id>/<queue_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct QueueArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the queue.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct QueueResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the queue.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: QueueArgs) -> QueueResult {

    let result = crate::bindings::pulumi::cloudflare::queue::invoke(name, &crate::bindings::pulumi::cloudflare::queue::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    QueueResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
