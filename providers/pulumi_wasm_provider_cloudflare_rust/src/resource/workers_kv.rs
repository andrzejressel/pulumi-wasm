//! Provides a resource to manage a Cloudflare Workers KV Pair.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const exampleNs = new cloudflare.WorkersKvNamespace("exampleNs", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     title: "test-namespace",
//! });
//! const example = new cloudflare.WorkersKv("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     namespaceId: exampleNs.id,
//!     key: "test-key",
//!     value: "test value",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example_ns = cloudflare.WorkersKvNamespace("exampleNs",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     title="test-namespace")
//! example = cloudflare.WorkersKv("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     namespace_id=example_ns.id,
//!     key="test-key",
//!     value="test value")
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
//!     var exampleNs = new Cloudflare.WorkersKvNamespace("exampleNs", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Title = "test-namespace",
//!     });
//! 
//!     var example = new Cloudflare.WorkersKv("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         NamespaceId = exampleNs.Id,
//!         Key = "test-key",
//!         Value = "test value",
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
//! 		exampleNs, err := cloudflare.NewWorkersKvNamespace(ctx, "exampleNs", &cloudflare.WorkersKvNamespaceArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Title:     pulumi.String("test-namespace"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewWorkersKv(ctx, "example", &cloudflare.WorkersKvArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			NamespaceId: exampleNs.ID(),
//! 			Key:         pulumi.String("test-key"),
//! 			Value:       pulumi.String("test value"),
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
//! import com.pulumi.cloudflare.WorkersKv;
//! import com.pulumi.cloudflare.WorkersKvArgs;
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
//!         var exampleNs = new WorkersKvNamespace("exampleNs", WorkersKvNamespaceArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .title("test-namespace")
//!             .build());
//! 
//!         var example = new WorkersKv("example", WorkersKvArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .namespaceId(exampleNs.id())
//!             .key("test-key")
//!             .value("test value")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleNs:
//!     type: cloudflare:WorkersKvNamespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       title: test-namespace
//!   example:
//!     type: cloudflare:WorkersKv
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       namespaceId: ${exampleNs.id}
//!       key: test-key
//!       value: test value
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersKv:WorkersKv example <account_id>/<namespace_id>/<key_name>
//! ```
//! 

pub struct WorkersKvArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    pub value: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    pub value: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersKvArgs) -> WorkersKvResult {

    let result = crate::bindings::pulumi::cloudflare::workers_kv::invoke(name, &crate::bindings::pulumi::cloudflare::workers_kv::Args {
        account_id: args.account_id.get_inner(),
        key: args.key.get_inner(),
        namespace_id: args.namespace_id.get_inner(),
        value: args.value.get_inner(),
    });

    WorkersKvResult {
        account_id: crate::into_domain(result.account_id),
        key: crate::into_domain(result.key),
        namespace_id: crate::into_domain(result.namespace_id),
        value: crate::into_domain(result.value),
    }
}
