//! Provides a Cloudflare worker script resource. In order for a script to be active, you'll also need to setup a `cloudflare.WorkerRoute`.
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
//! const myNamespace = new cloudflare.WorkersKvNamespace("myNamespace", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     title: "example",
//! });
//! // Sets the script with the name "script_1"
//! const myScript = new cloudflare.WorkerScript("myScript", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "script_1",
//!     content: fs.readFileSync("script.js", "utf8"),
//!     kvNamespaceBindings: [{
//!         name: "MY_EXAMPLE_KV_NAMESPACE",
//!         namespaceId: myNamespace.id,
//!     }],
//!     plainTextBindings: [{
//!         name: "MY_EXAMPLE_PLAIN_TEXT",
//!         text: "foobar",
//!     }],
//!     secretTextBindings: [{
//!         name: "MY_EXAMPLE_SECRET_TEXT",
//!         text: _var.secret_foo_value,
//!     }],
//!     webassemblyBindings: [{
//!         name: "MY_EXAMPLE_WASM",
//!         module: fs.readFileSync("example.wasm", { encoding: "base64" }),
//!     }],
//!     serviceBindings: [{
//!         name: "MY_SERVICE_BINDING",
//!         service: "MY_SERVICE",
//!         environment: "production",
//!     }],
//!     r2BucketBindings: [{
//!         name: "MY_BUCKET",
//!         bucketName: "MY_BUCKET_NAME",
//!     }],
//!     analyticsEngineBindings: [{
//!         name: "MY_DATASET",
//!         dataset: "dataset1",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import base64
//! import pulumi_cloudflare as cloudflare
//! 
//! my_namespace = cloudflare.WorkersKvNamespace("myNamespace",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     title="example")
//! # Sets the script with the name "script_1"
//! my_script = cloudflare.WorkerScript("myScript",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="script_1",
//!     content=(lambda path: open(path).read())("script.js"),
//!     kv_namespace_bindings=[cloudflare.WorkerScriptKvNamespaceBindingArgs(
//!         name="MY_EXAMPLE_KV_NAMESPACE",
//!         namespace_id=my_namespace.id,
//!     )],
//!     plain_text_bindings=[cloudflare.WorkerScriptPlainTextBindingArgs(
//!         name="MY_EXAMPLE_PLAIN_TEXT",
//!         text="foobar",
//!     )],
//!     secret_text_bindings=[cloudflare.WorkerScriptSecretTextBindingArgs(
//!         name="MY_EXAMPLE_SECRET_TEXT",
//!         text=var["secret_foo_value"],
//!     )],
//!     webassembly_bindings=[cloudflare.WorkerScriptWebassemblyBindingArgs(
//!         name="MY_EXAMPLE_WASM",
//!         module=(lambda path: base64.b64encode(open(path).read().encode()).decode())("example.wasm"),
//!     )],
//!     service_bindings=[cloudflare.WorkerScriptServiceBindingArgs(
//!         name="MY_SERVICE_BINDING",
//!         service="MY_SERVICE",
//!         environment="production",
//!     )],
//!     r2_bucket_bindings=[cloudflare.WorkerScriptR2BucketBindingArgs(
//!         name="MY_BUCKET",
//!         bucket_name="MY_BUCKET_NAME",
//!     )],
//!     analytics_engine_bindings=[cloudflare.WorkerScriptAnalyticsEngineBindingArgs(
//!         name="MY_DATASET",
//!         dataset="dataset1",
//!     )])
//! ```
//! ### C#
//! ```csharp
//! using System;
//! using System.Collections.Generic;
//! using System.IO;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! 	
//! string ReadFileBase64(string path) 
//! {
//!     return Convert.ToBase64String(Encoding.UTF8.GetBytes(File.ReadAllText(path)));
//! }
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var myNamespace = new Cloudflare.WorkersKvNamespace("myNamespace", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Title = "example",
//!     });
//! 
//!     // Sets the script with the name "script_1"
//!     var myScript = new Cloudflare.WorkerScript("myScript", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "script_1",
//!         Content = File.ReadAllText("script.js"),
//!         KvNamespaceBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptKvNamespaceBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_KV_NAMESPACE",
//!                 NamespaceId = myNamespace.Id,
//!             },
//!         },
//!         PlainTextBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptPlainTextBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_PLAIN_TEXT",
//!                 Text = "foobar",
//!             },
//!         },
//!         SecretTextBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptSecretTextBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_SECRET_TEXT",
//!                 Text = @var.Secret_foo_value,
//!             },
//!         },
//!         WebassemblyBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptWebassemblyBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_WASM",
//!                 Module = ReadFileBase64("example.wasm"),
//!             },
//!         },
//!         ServiceBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptServiceBindingArgs
//!             {
//!                 Name = "MY_SERVICE_BINDING",
//!                 Service = "MY_SERVICE",
//!                 Environment = "production",
//!             },
//!         },
//!         R2BucketBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptR2BucketBindingArgs
//!             {
//!                 Name = "MY_BUCKET",
//!                 BucketName = "MY_BUCKET_NAME",
//!             },
//!         },
//!         AnalyticsEngineBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkerScriptAnalyticsEngineBindingArgs
//!             {
//!                 Name = "MY_DATASET",
//!                 Dataset = "dataset1",
//!             },
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
//! 	"encoding/base64"
//! 	"os"
//! 
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func filebase64OrPanic(path string) string {
//! 	if fileData, err := os.ReadFile(path); err == nil {
//! 		return base64.StdEncoding.EncodeToString(fileData[:])
//! 	} else {
//! 		panic(err.Error())
//! 	}
//! }
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
//! 		myNamespace, err := cloudflare.NewWorkersKvNamespace(ctx, "myNamespace", &cloudflare.WorkersKvNamespaceArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Title:     pulumi.String("example"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Sets the script with the name "script_1"
//! 		_, err = cloudflare.NewWorkerScript(ctx, "myScript", &cloudflare.WorkerScriptArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("script_1"),
//! 			Content:   readFileOrPanic("script.js"),
//! 			KvNamespaceBindings: cloudflare.WorkerScriptKvNamespaceBindingArray{
//! 				&cloudflare.WorkerScriptKvNamespaceBindingArgs{
//! 					Name:        pulumi.String("MY_EXAMPLE_KV_NAMESPACE"),
//! 					NamespaceId: myNamespace.ID(),
//! 				},
//! 			},
//! 			PlainTextBindings: cloudflare.WorkerScriptPlainTextBindingArray{
//! 				&cloudflare.WorkerScriptPlainTextBindingArgs{
//! 					Name: pulumi.String("MY_EXAMPLE_PLAIN_TEXT"),
//! 					Text: pulumi.String("foobar"),
//! 				},
//! 			},
//! 			SecretTextBindings: cloudflare.WorkerScriptSecretTextBindingArray{
//! 				&cloudflare.WorkerScriptSecretTextBindingArgs{
//! 					Name: pulumi.String("MY_EXAMPLE_SECRET_TEXT"),
//! 					Text: pulumi.Any(_var.Secret_foo_value),
//! 				},
//! 			},
//! 			WebassemblyBindings: cloudflare.WorkerScriptWebassemblyBindingArray{
//! 				&cloudflare.WorkerScriptWebassemblyBindingArgs{
//! 					Name:   pulumi.String("MY_EXAMPLE_WASM"),
//! 					Module: filebase64OrPanic("example.wasm"),
//! 				},
//! 			},
//! 			ServiceBindings: cloudflare.WorkerScriptServiceBindingArray{
//! 				&cloudflare.WorkerScriptServiceBindingArgs{
//! 					Name:        pulumi.String("MY_SERVICE_BINDING"),
//! 					Service:     pulumi.String("MY_SERVICE"),
//! 					Environment: pulumi.String("production"),
//! 				},
//! 			},
//! 			R2BucketBindings: cloudflare.WorkerScriptR2BucketBindingArray{
//! 				&cloudflare.WorkerScriptR2BucketBindingArgs{
//! 					Name:       pulumi.String("MY_BUCKET"),
//! 					BucketName: pulumi.String("MY_BUCKET_NAME"),
//! 				},
//! 			},
//! 			AnalyticsEngineBindings: cloudflare.WorkerScriptAnalyticsEngineBindingArray{
//! 				&cloudflare.WorkerScriptAnalyticsEngineBindingArgs{
//! 					Name:    pulumi.String("MY_DATASET"),
//! 					Dataset: pulumi.String("dataset1"),
//! 				},
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
//! import com.pulumi.cloudflare.WorkersKvNamespace;
//! import com.pulumi.cloudflare.WorkersKvNamespaceArgs;
//! import com.pulumi.cloudflare.WorkerScript;
//! import com.pulumi.cloudflare.WorkerScriptArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptKvNamespaceBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptPlainTextBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptSecretTextBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptWebassemblyBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptServiceBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptR2BucketBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkerScriptAnalyticsEngineBindingArgs;
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
//!         var myNamespace = new WorkersKvNamespace("myNamespace", WorkersKvNamespaceArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .title("example")
//!             .build());
//! 
//!         // Sets the script with the name "script_1"
//!         var myScript = new WorkerScript("myScript", WorkerScriptArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("script_1")
//!             .content(Files.readString(Paths.get("script.js")))
//!             .kvNamespaceBindings(WorkerScriptKvNamespaceBindingArgs.builder()
//!                 .name("MY_EXAMPLE_KV_NAMESPACE")
//!                 .namespaceId(myNamespace.id())
//!                 .build())
//!             .plainTextBindings(WorkerScriptPlainTextBindingArgs.builder()
//!                 .name("MY_EXAMPLE_PLAIN_TEXT")
//!                 .text("foobar")
//!                 .build())
//!             .secretTextBindings(WorkerScriptSecretTextBindingArgs.builder()
//!                 .name("MY_EXAMPLE_SECRET_TEXT")
//!                 .text(var_.secret_foo_value())
//!                 .build())
//!             .webassemblyBindings(WorkerScriptWebassemblyBindingArgs.builder()
//!                 .name("MY_EXAMPLE_WASM")
//!                 .module(Base64.getEncoder().encodeToString(Files.readAllBytes(Paths.get("example.wasm"))))
//!                 .build())
//!             .serviceBindings(WorkerScriptServiceBindingArgs.builder()
//!                 .name("MY_SERVICE_BINDING")
//!                 .service("MY_SERVICE")
//!                 .environment("production")
//!                 .build())
//!             .r2BucketBindings(WorkerScriptR2BucketBindingArgs.builder()
//!                 .name("MY_BUCKET")
//!                 .bucketName("MY_BUCKET_NAME")
//!                 .build())
//!             .analyticsEngineBindings(WorkerScriptAnalyticsEngineBindingArgs.builder()
//!                 .name("MY_DATASET")
//!                 .dataset("dataset1")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workerScript:WorkerScript example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkerScriptArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
    /// The date to use for the compatibility flag.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub compatibility_flags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The script content.
    #[builder(into)]
    pub content: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// The base64 encoded wasm module you want to store.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
}

pub struct WorkerScriptResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
    /// The date to use for the compatibility flag.
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
    /// The script content.
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// The base64 encoded wasm module you want to store.
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The global variable for the binding in your Worker code.
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
    pub tags: pulumi_wasm_rust::Output<Vec<String>>,
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkerScriptArgs) -> WorkerScriptResult {

    let result = crate::bindings::pulumi::cloudflare::worker_script::invoke(name, &crate::bindings::pulumi::cloudflare::worker_script::Args {
        account_id: &args.account_id.get_inner(),
        analytics_engine_bindings: &args.analytics_engine_bindings.get_inner(),
        compatibility_date: &args.compatibility_date.get_inner(),
        compatibility_flags: &args.compatibility_flags.get_inner(),
        content: &args.content.get_inner(),
        d1_database_bindings: &args.d1_database_bindings.get_inner(),
        dispatch_namespace: &args.dispatch_namespace.get_inner(),
        kv_namespace_bindings: &args.kv_namespace_bindings.get_inner(),
        logpush: &args.logpush.get_inner(),
        module: &args.module.get_inner(),
        name: &args.name.get_inner(),
        placements: &args.placements.get_inner(),
        plain_text_bindings: &args.plain_text_bindings.get_inner(),
        queue_bindings: &args.queue_bindings.get_inner(),
        r2_bucket_bindings: &args.r2_bucket_bindings.get_inner(),
        secret_text_bindings: &args.secret_text_bindings.get_inner(),
        service_bindings: &args.service_bindings.get_inner(),
        tags: &args.tags.get_inner(),
        webassembly_bindings: &args.webassembly_bindings.get_inner(),
    });

    WorkerScriptResult {
        account_id: crate::into_domain(result.account_id),
        analytics_engine_bindings: crate::into_domain(result.analytics_engine_bindings),
        compatibility_date: crate::into_domain(result.compatibility_date),
        compatibility_flags: crate::into_domain(result.compatibility_flags),
        content: crate::into_domain(result.content),
        d1_database_bindings: crate::into_domain(result.d1_database_bindings),
        dispatch_namespace: crate::into_domain(result.dispatch_namespace),
        kv_namespace_bindings: crate::into_domain(result.kv_namespace_bindings),
        logpush: crate::into_domain(result.logpush),
        module: crate::into_domain(result.module),
        name: crate::into_domain(result.name),
        placements: crate::into_domain(result.placements),
        plain_text_bindings: crate::into_domain(result.plain_text_bindings),
        queue_bindings: crate::into_domain(result.queue_bindings),
        r2_bucket_bindings: crate::into_domain(result.r2_bucket_bindings),
        secret_text_bindings: crate::into_domain(result.secret_text_bindings),
        service_bindings: crate::into_domain(result.service_bindings),
        tags: crate::into_domain(result.tags),
        webassembly_bindings: crate::into_domain(result.webassembly_bindings),
    }
}
