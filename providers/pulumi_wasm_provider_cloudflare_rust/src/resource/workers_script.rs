//! Provides a Cloudflare worker script resource. In order for a script to be active, you'll also need to setup a `cloudflare.WorkerRoute`.
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
//! const myNamespace = new cloudflare.WorkersKvNamespace("my_namespace", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     title: "example",
//! });
//! // Sets the script with the name "script_1"
//! const myScript = new cloudflare.WorkersScript("my_script", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "script_1",
//!     content: std.file({
//!         input: "script.js",
//!     }).then(invoke => invoke.result),
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
//!         text: secretFooValue,
//!     }],
//!     webassemblyBindings: [{
//!         name: "MY_EXAMPLE_WASM",
//!         module: std.filebase64({
//!             input: "example.wasm",
//!         }).then(invoke => invoke.result),
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
//! import pulumi_cloudflare as cloudflare
//! import pulumi_std as std
//! 
//! my_namespace = cloudflare.WorkersKvNamespace("my_namespace",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     title="example")
//! # Sets the script with the name "script_1"
//! my_script = cloudflare.WorkersScript("my_script",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="script_1",
//!     content=std.file(input="script.js").result,
//!     kv_namespace_bindings=[{
//!         "name": "MY_EXAMPLE_KV_NAMESPACE",
//!         "namespace_id": my_namespace.id,
//!     }],
//!     plain_text_bindings=[{
//!         "name": "MY_EXAMPLE_PLAIN_TEXT",
//!         "text": "foobar",
//!     }],
//!     secret_text_bindings=[{
//!         "name": "MY_EXAMPLE_SECRET_TEXT",
//!         "text": secret_foo_value,
//!     }],
//!     webassembly_bindings=[{
//!         "name": "MY_EXAMPLE_WASM",
//!         "module": std.filebase64(input="example.wasm").result,
//!     }],
//!     service_bindings=[{
//!         "name": "MY_SERVICE_BINDING",
//!         "service": "MY_SERVICE",
//!         "environment": "production",
//!     }],
//!     r2_bucket_bindings=[{
//!         "name": "MY_BUCKET",
//!         "bucket_name": "MY_BUCKET_NAME",
//!     }],
//!     analytics_engine_bindings=[{
//!         "name": "MY_DATASET",
//!         "dataset": "dataset1",
//!     }])
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
//!     var myNamespace = new Cloudflare.WorkersKvNamespace("my_namespace", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Title = "example",
//!     });
//! 
//!     // Sets the script with the name "script_1"
//!     var myScript = new Cloudflare.WorkersScript("my_script", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "script_1",
//!         Content = Std.File.Invoke(new()
//!         {
//!             Input = "script.js",
//!         }).Apply(invoke => invoke.Result),
//!         KvNamespaceBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptKvNamespaceBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_KV_NAMESPACE",
//!                 NamespaceId = myNamespace.Id,
//!             },
//!         },
//!         PlainTextBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptPlainTextBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_PLAIN_TEXT",
//!                 Text = "foobar",
//!             },
//!         },
//!         SecretTextBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptSecretTextBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_SECRET_TEXT",
//!                 Text = secretFooValue,
//!             },
//!         },
//!         WebassemblyBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptWebassemblyBindingArgs
//!             {
//!                 Name = "MY_EXAMPLE_WASM",
//!                 Module = Std.Filebase64.Invoke(new()
//!                 {
//!                     Input = "example.wasm",
//!                 }).Apply(invoke => invoke.Result),
//!             },
//!         },
//!         ServiceBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptServiceBindingArgs
//!             {
//!                 Name = "MY_SERVICE_BINDING",
//!                 Service = "MY_SERVICE",
//!                 Environment = "production",
//!             },
//!         },
//!         R2BucketBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptR2BucketBindingArgs
//!             {
//!                 Name = "MY_BUCKET",
//!                 BucketName = "MY_BUCKET_NAME",
//!             },
//!         },
//!         AnalyticsEngineBindings = new[]
//!         {
//!             new Cloudflare.Inputs.WorkersScriptAnalyticsEngineBindingArgs
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
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi-std/sdk/go/std"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		myNamespace, err := cloudflare.NewWorkersKvNamespace(ctx, "my_namespace", &cloudflare.WorkersKvNamespaceArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Title:     pulumi.String("example"),
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
//! 		invokeFilebase641, err := std.Filebase64(ctx, &std.Filebase64Args{
//! 			Input: "example.wasm",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Sets the script with the name "script_1"
//! 		_, err = cloudflare.NewWorkersScript(ctx, "my_script", &cloudflare.WorkersScriptArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("script_1"),
//! 			Content:   pulumi.String(invokeFile.Result),
//! 			KvNamespaceBindings: cloudflare.WorkersScriptKvNamespaceBindingArray{
//! 				&cloudflare.WorkersScriptKvNamespaceBindingArgs{
//! 					Name:        pulumi.String("MY_EXAMPLE_KV_NAMESPACE"),
//! 					NamespaceId: myNamespace.ID(),
//! 				},
//! 			},
//! 			PlainTextBindings: cloudflare.WorkersScriptPlainTextBindingArray{
//! 				&cloudflare.WorkersScriptPlainTextBindingArgs{
//! 					Name: pulumi.String("MY_EXAMPLE_PLAIN_TEXT"),
//! 					Text: pulumi.String("foobar"),
//! 				},
//! 			},
//! 			SecretTextBindings: cloudflare.WorkersScriptSecretTextBindingArray{
//! 				&cloudflare.WorkersScriptSecretTextBindingArgs{
//! 					Name: pulumi.String("MY_EXAMPLE_SECRET_TEXT"),
//! 					Text: pulumi.Any(secretFooValue),
//! 				},
//! 			},
//! 			WebassemblyBindings: cloudflare.WorkersScriptWebassemblyBindingArray{
//! 				&cloudflare.WorkersScriptWebassemblyBindingArgs{
//! 					Name:   pulumi.String("MY_EXAMPLE_WASM"),
//! 					Module: pulumi.String(invokeFilebase641.Result),
//! 				},
//! 			},
//! 			ServiceBindings: cloudflare.WorkersScriptServiceBindingArray{
//! 				&cloudflare.WorkersScriptServiceBindingArgs{
//! 					Name:        pulumi.String("MY_SERVICE_BINDING"),
//! 					Service:     pulumi.String("MY_SERVICE"),
//! 					Environment: pulumi.String("production"),
//! 				},
//! 			},
//! 			R2BucketBindings: cloudflare.WorkersScriptR2BucketBindingArray{
//! 				&cloudflare.WorkersScriptR2BucketBindingArgs{
//! 					Name:       pulumi.String("MY_BUCKET"),
//! 					BucketName: pulumi.String("MY_BUCKET_NAME"),
//! 				},
//! 			},
//! 			AnalyticsEngineBindings: cloudflare.WorkersScriptAnalyticsEngineBindingArray{
//! 				&cloudflare.WorkersScriptAnalyticsEngineBindingArgs{
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
//! import com.pulumi.cloudflare.WorkersScript;
//! import com.pulumi.cloudflare.WorkersScriptArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptKvNamespaceBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptPlainTextBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptSecretTextBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptWebassemblyBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptServiceBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptR2BucketBindingArgs;
//! import com.pulumi.cloudflare.inputs.WorkersScriptAnalyticsEngineBindingArgs;
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
//!         var myScript = new WorkersScript("myScript", WorkersScriptArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("script_1")
//!             .content(StdFunctions.file(FileArgs.builder()
//!                 .input("script.js")
//!                 .build()).result())
//!             .kvNamespaceBindings(WorkersScriptKvNamespaceBindingArgs.builder()
//!                 .name("MY_EXAMPLE_KV_NAMESPACE")
//!                 .namespaceId(myNamespace.id())
//!                 .build())
//!             .plainTextBindings(WorkersScriptPlainTextBindingArgs.builder()
//!                 .name("MY_EXAMPLE_PLAIN_TEXT")
//!                 .text("foobar")
//!                 .build())
//!             .secretTextBindings(WorkersScriptSecretTextBindingArgs.builder()
//!                 .name("MY_EXAMPLE_SECRET_TEXT")
//!                 .text(secretFooValue)
//!                 .build())
//!             .webassemblyBindings(WorkersScriptWebassemblyBindingArgs.builder()
//!                 .name("MY_EXAMPLE_WASM")
//!                 .module(StdFunctions.filebase64(Filebase64Args.builder()
//!                     .input("example.wasm")
//!                     .build()).result())
//!                 .build())
//!             .serviceBindings(WorkersScriptServiceBindingArgs.builder()
//!                 .name("MY_SERVICE_BINDING")
//!                 .service("MY_SERVICE")
//!                 .environment("production")
//!                 .build())
//!             .r2BucketBindings(WorkersScriptR2BucketBindingArgs.builder()
//!                 .name("MY_BUCKET")
//!                 .bucketName("MY_BUCKET_NAME")
//!                 .build())
//!             .analyticsEngineBindings(WorkersScriptAnalyticsEngineBindingArgs.builder()
//!                 .name("MY_DATASET")
//!                 .dataset("dataset1")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   myNamespace:
//!     type: cloudflare:WorkersKvNamespace
//!     name: my_namespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       title: example
//!   # Sets the script with the name "script_1"
//!   myScript:
//!     type: cloudflare:WorkersScript
//!     name: my_script
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: script_1
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: script.js
//!           Return: result
//!       kvNamespaceBindings:
//!         - name: MY_EXAMPLE_KV_NAMESPACE
//!           namespaceId: ${myNamespace.id}
//!       plainTextBindings:
//!         - name: MY_EXAMPLE_PLAIN_TEXT
//!           text: foobar
//!       secretTextBindings:
//!         - name: MY_EXAMPLE_SECRET_TEXT
//!           text: ${secretFooValue}
//!       webassemblyBindings:
//!         - name: MY_EXAMPLE_WASM
//!           module:
//!             fn::invoke:
//!               Function: std:filebase64
//!               Arguments:
//!                 input: example.wasm
//!               Return: result
//!       serviceBindings:
//!         - name: MY_SERVICE_BINDING
//!           service: MY_SERVICE
//!           environment: production
//!       r2BucketBindings:
//!         - name: MY_BUCKET
//!           bucketName: MY_BUCKET_NAME
//!       analyticsEngineBindings:
//!         - name: MY_DATASET
//!           dataset: dataset1
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersScript:WorkersScript example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptAnalyticsEngineBinding>>>,
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
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptHyperdriveConfigBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlacement>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlainTextBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptQueueBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptR2BucketBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptSecretTextBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptServiceBinding>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptWebassemblyBinding>>>,
}

pub struct WorkersScriptResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub analytics_engine_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptAnalyticsEngineBinding>>>,
    /// The date to use for the compatibility flag.
    pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
    /// Compatibility flags used for Worker Scripts.
    pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
    /// The script content.
    pub content: pulumi_wasm_rust::Output<String>,
    pub d1_database_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptD1DatabaseBinding>>>,
    /// Name of the Workers for Platforms dispatch namespace.
    pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
    pub hyperdrive_config_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptHyperdriveConfigBinding>>>,
    pub kv_namespace_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptKvNamespaceBinding>>>,
    /// Enabling allows Worker events to be sent to a defined Logpush destination.
    pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to upload Worker as a module.
    pub module: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name for the script. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlacement>>>,
    pub plain_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptPlainTextBinding>>>,
    pub queue_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptQueueBinding>>>,
    pub r2_bucket_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptR2BucketBinding>>>,
    pub secret_text_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptSecretTextBinding>>>,
    pub service_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptServiceBinding>>>,
    pub tags: pulumi_wasm_rust::Output<Vec<String>>,
    pub webassembly_bindings: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkersScriptWebassemblyBinding>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersScriptArgs) -> WorkersScriptResult {

    let result = crate::bindings::pulumi::cloudflare::workers_script::invoke(name, &crate::bindings::pulumi::cloudflare::workers_script::Args {
        account_id: &args.account_id.get_inner(),
        analytics_engine_bindings: &args.analytics_engine_bindings.get_inner(),
        compatibility_date: &args.compatibility_date.get_inner(),
        compatibility_flags: &args.compatibility_flags.get_inner(),
        content: &args.content.get_inner(),
        d1_database_bindings: &args.d1_database_bindings.get_inner(),
        dispatch_namespace: &args.dispatch_namespace.get_inner(),
        hyperdrive_config_bindings: &args.hyperdrive_config_bindings.get_inner(),
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

    WorkersScriptResult {
        account_id: crate::into_domain(result.account_id),
        analytics_engine_bindings: crate::into_domain(result.analytics_engine_bindings),
        compatibility_date: crate::into_domain(result.compatibility_date),
        compatibility_flags: crate::into_domain(result.compatibility_flags),
        content: crate::into_domain(result.content),
        d1_database_bindings: crate::into_domain(result.d1_database_bindings),
        dispatch_namespace: crate::into_domain(result.dispatch_namespace),
        hyperdrive_config_bindings: crate::into_domain(result.hyperdrive_config_bindings),
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