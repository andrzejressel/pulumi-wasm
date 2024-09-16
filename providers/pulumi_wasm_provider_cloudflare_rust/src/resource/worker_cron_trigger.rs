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
//! import * as fs from "fs";
//! 
//! const exampleScript = new cloudflare.WorkerScript("exampleScript", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example-script",
//!     content: fs.readFileSync("path/to/my.js", "utf8"),
//! });
//! const exampleTrigger = new cloudflare.WorkerCronTrigger("exampleTrigger", {
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
//! 
//! example_script = cloudflare.WorkerScript("exampleScript",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example-script",
//!     content=(lambda path: open(path).read())("path/to/my.js"))
//! example_trigger = cloudflare.WorkerCronTrigger("exampleTrigger",
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
//! using System.IO;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var exampleScript = new Cloudflare.WorkerScript("exampleScript", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example-script",
//!         Content = File.ReadAllText("path/to/my.js"),
//!     });
//! 
//!     var exampleTrigger = new Cloudflare.WorkerCronTrigger("exampleTrigger", new()
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
//! 		exampleScript, err := cloudflare.NewWorkerScript(ctx, "exampleScript", &cloudflare.WorkerScriptArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("example-script"),
//! 			Content:   readFileOrPanic("path/to/my.js"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewWorkerCronTrigger(ctx, "exampleTrigger", &cloudflare.WorkerCronTriggerArgs{
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
//! import com.pulumi.cloudflare.WorkerScript;
//! import com.pulumi.cloudflare.WorkerScriptArgs;
//! import com.pulumi.cloudflare.WorkerCronTrigger;
//! import com.pulumi.cloudflare.WorkerCronTriggerArgs;
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
//!         var exampleScript = new WorkerScript("exampleScript", WorkerScriptArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example-script")
//!             .content(Files.readString(Paths.get("path/to/my.js")))
//!             .build());
//! 
//!         var exampleTrigger = new WorkerCronTrigger("exampleTrigger", WorkerCronTriggerArgs.builder()        
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
//!     type: cloudflare:WorkerScript
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example-script
//!       content:
//!         fn::readFile: path/to/my.js
//!   exampleTrigger:
//!     type: cloudflare:WorkerCronTrigger
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
//! $ pulumi import cloudflare:index/workerCronTrigger:WorkerCronTrigger example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkerCronTriggerArgs {
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

pub struct WorkerCronTriggerResult {
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
pub fn create(name: &str, args: WorkerCronTriggerArgs) -> WorkerCronTriggerResult {

    let result = crate::bindings::pulumi::cloudflare::worker_cron_trigger::invoke(name, &crate::bindings::pulumi::cloudflare::worker_cron_trigger::Args {
        account_id: &args.account_id.get_inner(),
        schedules: &args.schedules.get_inner(),
        script_name: &args.script_name.get_inner(),
    });

    WorkerCronTriggerResult {
        account_id: crate::into_domain(result.account_id),
        schedules: crate::into_domain(result.schedules),
        script_name: crate::into_domain(result.script_name),
    }
}
