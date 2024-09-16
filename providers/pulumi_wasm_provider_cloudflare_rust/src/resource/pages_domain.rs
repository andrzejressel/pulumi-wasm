//! Provides a resource for managing Cloudflare Pages domains.
//! 
//! > A DNS record for the domain is not automatically created. You need to create
//!    a `cloudflare.Record` resource for the domain you want to use.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const my_domain = new cloudflare.PagesDomain("my-domain", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     domain: "example.com",
//!     projectName: "my-example-project",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! my_domain = cloudflare.PagesDomain("my-domain",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     domain="example.com",
//!     project_name="my-example-project")
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
//!     var my_domain = new Cloudflare.PagesDomain("my-domain", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Domain = "example.com",
//!         ProjectName = "my-example-project",
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
//! 		_, err := cloudflare.NewPagesDomain(ctx, "my-domain", &cloudflare.PagesDomainArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Domain:      pulumi.String("example.com"),
//! 			ProjectName: pulumi.String("my-example-project"),
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
//! import com.pulumi.cloudflare.PagesDomain;
//! import com.pulumi.cloudflare.PagesDomainArgs;
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
//!         var my_domain = new PagesDomain("my-domain", PagesDomainArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .domain("example.com")
//!             .projectName("my-example-project")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   my-domain:
//!     type: cloudflare:PagesDomain
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       domain: example.com
//!       projectName: my-example-project
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pagesDomain:PagesDomain example <account_id>/<project_name>/<domain-name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct PagesDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub project_name: pulumi_wasm_rust::Output<String>,
}

pub struct PagesDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    pub project_name: pulumi_wasm_rust::Output<String>,
    /// Status of the custom domain.
    pub status: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PagesDomainArgs) -> PagesDomainResult {

    let result = crate::bindings::pulumi::cloudflare::pages_domain::invoke(name, &crate::bindings::pulumi::cloudflare::pages_domain::Args {
        account_id: &args.account_id.get_inner(),
        domain: &args.domain.get_inner(),
        project_name: &args.project_name.get_inner(),
    });

    PagesDomainResult {
        account_id: crate::into_domain(result.account_id),
        domain: crate::into_domain(result.domain),
        project_name: crate::into_domain(result.project_name),
        status: crate::into_domain(result.status),
    }
}
