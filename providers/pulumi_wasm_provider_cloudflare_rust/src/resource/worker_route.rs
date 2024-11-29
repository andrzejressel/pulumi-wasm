//! Provides a Cloudflare worker route resource. A route will also require a `cloudflare.WorkerScript`.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const myScript = new cloudflare.WorkersScript("my_script", {});
//! // Runs the specified worker script for all URLs that match `example.com/*`
//! const myRoute = new cloudflare.WorkerRoute("my_route", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     pattern: "example.com/*",
//!     scriptName: myScript.name,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! my_script = cloudflare.WorkersScript("my_script")
//! # Runs the specified worker script for all URLs that match `example.com/*`
//! my_route = cloudflare.WorkerRoute("my_route",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     pattern="example.com/*",
//!     script_name=my_script.name)
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
//!     var myScript = new Cloudflare.WorkersScript("my_script");
//! 
//!     // Runs the specified worker script for all URLs that match `example.com/*`
//!     var myRoute = new Cloudflare.WorkerRoute("my_route", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Pattern = "example.com/*",
//!         ScriptName = myScript.Name,
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
//! 		myScript, err := cloudflare.NewWorkersScript(ctx, "my_script", nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Runs the specified worker script for all URLs that match `example.com/*`
//! 		_, err = cloudflare.NewWorkerRoute(ctx, "my_route", &cloudflare.WorkerRouteArgs{
//! 			ZoneId:     pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Pattern:    pulumi.String("example.com/*"),
//! 			ScriptName: myScript.Name,
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
//! import com.pulumi.cloudflare.WorkersScript;
//! import com.pulumi.cloudflare.WorkerRoute;
//! import com.pulumi.cloudflare.WorkerRouteArgs;
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
//!         var myScript = new WorkersScript("myScript");
//! 
//!         // Runs the specified worker script for all URLs that match `example.com/*`
//!         var myRoute = new WorkerRoute("myRoute", WorkerRouteArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .pattern("example.com/*")
//!             .scriptName(myScript.name())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Runs the specified worker script for all URLs that match `example.com/*`
//!   myRoute:
//!     type: cloudflare:WorkerRoute
//!     name: my_route
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       pattern: example.com/*
//!       scriptName: ${myScript.name}
//!   myScript:
//!     type: cloudflare:WorkersScript
//!     name: my_script
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workerRoute:WorkerRoute example <zone_id>/<route_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkerRouteArgs {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    #[builder(into)]
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerRouteResult {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkerRouteArgs) -> WorkerRouteResult {

    let result = crate::bindings::pulumi::cloudflare::worker_route::invoke(name, &crate::bindings::pulumi::cloudflare::worker_route::Args {
        pattern: &args.pattern.get_inner(),
        script_name: &args.script_name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WorkerRouteResult {
        pattern: crate::into_domain(result.pattern),
        script_name: crate::into_domain(result.script_name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
