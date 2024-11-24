//! Provides a Cloudflare Worker secret resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const mySecret = new cloudflare.WorkersSecret("my_secret", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "MY_EXAMPLE_SECRET_TEXT",
//!     scriptName: "script_1",
//!     secretText: "my_secret_value",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! my_secret = cloudflare.WorkersSecret("my_secret",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="MY_EXAMPLE_SECRET_TEXT",
//!     script_name="script_1",
//!     secret_text="my_secret_value")
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
//!     var mySecret = new Cloudflare.WorkersSecret("my_secret", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "MY_EXAMPLE_SECRET_TEXT",
//!         ScriptName = "script_1",
//!         SecretText = "my_secret_value",
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
//! 		_, err := cloudflare.NewWorkersSecret(ctx, "my_secret", &cloudflare.WorkersSecretArgs{
//! 			AccountId:  pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:       pulumi.String("MY_EXAMPLE_SECRET_TEXT"),
//! 			ScriptName: pulumi.String("script_1"),
//! 			SecretText: pulumi.String("my_secret_value"),
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
//! import com.pulumi.cloudflare.WorkersSecret;
//! import com.pulumi.cloudflare.WorkersSecretArgs;
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
//!         var mySecret = new WorkersSecret("mySecret", WorkersSecretArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("MY_EXAMPLE_SECRET_TEXT")
//!             .scriptName("script_1")
//!             .secretText("my_secret_value")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   mySecret:
//!     type: cloudflare:WorkersSecret
//!     name: my_secret
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: MY_EXAMPLE_SECRET_TEXT
//!       scriptName: script_1
//!       secretText: my_secret_value
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersSecret:WorkersSecret example <account_id>/<script_name>/<secret_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersSecretArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersSecretResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersSecretArgs) -> WorkersSecretResult {

    let result = crate::bindings::pulumi::cloudflare::workers_secret::invoke(name, &crate::bindings::pulumi::cloudflare::workers_secret::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
        script_name: &args.script_name.get_inner(),
        secret_text: &args.secret_text.get_inner(),
    });

    WorkersSecretResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        script_name: crate::into_domain(result.script_name),
        secret_text: crate::into_domain(result.secret_text),
    }
}
