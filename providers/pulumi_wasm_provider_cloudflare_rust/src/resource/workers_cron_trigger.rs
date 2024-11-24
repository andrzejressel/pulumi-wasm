//! Worker Cron Triggers allow users to map a cron expression to a Worker script
//! using a `ScheduledEvent` listener that enables Workers to be executed on a
//! schedule. Worker Cron Triggers are ideal for running periodic jobs for
//! maintenance or calling third-party APIs to collect up-to-date data.
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
//! const exampleScript = new cloudflare.WorkersScript("example_script", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example-script",
//!     content: std.file({
//!         input: "path/to/my.js",
//!     }).then(invoke => invoke.result),
//! });
//! const exampleTrigger = new cloudflare.WorkersCronTrigger("example_trigger", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     scriptName: exampleScript.name,
//!     schedules: [
//!         "*/5 * * * *",
//!         "10 7 * * mon-fri",
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! import pulumi_std as std
//! 
//! example_script = cloudflare.WorkersScript("example_script",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example-script",
//!     content=std.file(input="path/to/my.js").result)
//! example_trigger = cloudflare.WorkersCronTrigger("example_trigger",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     script_name=example_script.name,
//!     schedules=[
//!         "*/5 * * * *",
//!         "10 7 * * mon-fri",
//!     ])
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
//!     var exampleScript = new Cloudflare.WorkersScript("example_script", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example-script",
//!         Content = Std.File.Invoke(new()
//!         {
//!             Input = "path/to/my.js",
//!         }).Apply(invoke => invoke.Result),
//!     });
//! 
//!     var exampleTrigger = new Cloudflare.WorkersCronTrigger("example_trigger", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ScriptName = exampleScript.Name,
//!         Schedules = new[]
//!         {
//!             "*/5 * * * *",
//!             "10 7 * * mon-fri",
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
//! 		invokeFile, err := std.File(ctx, &std.FileArgs{
//! 			Input: "path/to/my.js",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		exampleScript, err := cloudflare.NewWorkersScript(ctx, "example_script", &cloudflare.WorkersScriptArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("example-script"),
//! 			Content:   pulumi.String(invokeFile.Result),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewWorkersCronTrigger(ctx, "example_trigger", &cloudflare.WorkersCronTriggerArgs{
//! 			AccountId:  pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ScriptName: exampleScript.Name,
//! 			Schedules: pulumi.StringArray{
//! 				pulumi.String("*/5 * * * *"),
//! 				pulumi.String("10 7 * * mon-fri"),
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
//! import com.pulumi.cloudflare.WorkersScript;
//! import com.pulumi.cloudflare.WorkersScriptArgs;
//! import com.pulumi.cloudflare.WorkersCronTrigger;
//! import com.pulumi.cloudflare.WorkersCronTriggerArgs;
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
//!         var exampleScript = new WorkersScript("exampleScript", WorkersScriptArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example-script")
//!             .content(StdFunctions.file(FileArgs.builder()
//!                 .input("path/to/my.js")
//!                 .build()).result())
//!             .build());
//! 
//!         var exampleTrigger = new WorkersCronTrigger("exampleTrigger", WorkersCronTriggerArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .scriptName(exampleScript.name())
//!             .schedules(            
//!                 "*/5 * * * *",
//!                 "10 7 * * mon-fri")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleScript:
//!     type: cloudflare:WorkersScript
//!     name: example_script
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example-script
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: path/to/my.js
//!           Return: result
//!   exampleTrigger:
//!     type: cloudflare:WorkersCronTrigger
//!     name: example_trigger
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       scriptName: ${exampleScript.name}
//!       schedules:
//!         - '*/5 * * * *'
//!         - 10 7 * * mon-fri
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersCronTrigger:WorkersCronTrigger example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersCronTriggerArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Cron expressions to execute the Worker script.
    #[builder(into)]
    pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
    /// Worker script to target for the schedules.
    #[builder(into)]
    pub script_name: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersCronTriggerResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Cron expressions to execute the Worker script.
    pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
    /// Worker script to target for the schedules.
    pub script_name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersCronTriggerArgs) -> WorkersCronTriggerResult {

    let result = crate::bindings::pulumi::cloudflare::workers_cron_trigger::invoke(name, &crate::bindings::pulumi::cloudflare::workers_cron_trigger::Args {
        account_id: &args.account_id.get_inner(),
        schedules: &args.schedules.get_inner(),
        script_name: &args.script_name.get_inner(),
    });

    WorkersCronTriggerResult {
        account_id: crate::into_domain(result.account_id),
        schedules: crate::into_domain(result.schedules),
        script_name: crate::into_domain(result.script_name),
    }
}
