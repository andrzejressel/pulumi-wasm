//! Creates a Worker Custom Domain.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.WorkerDomain("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     hostname: "subdomain.example.com",
//!     service: "my-service",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.WorkerDomain("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     hostname="subdomain.example.com",
//!     service="my-service",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     var example = new Cloudflare.WorkerDomain("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Hostname = "subdomain.example.com",
//!         Service = "my-service",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
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
//! 		_, err := cloudflare.NewWorkerDomain(ctx, "example", &cloudflare.WorkerDomainArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Hostname:  pulumi.String("subdomain.example.com"),
//! 			Service:   pulumi.String("my-service"),
//! 			ZoneId:    pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.WorkerDomain;
//! import com.pulumi.cloudflare.WorkerDomainArgs;
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
//!         var example = new WorkerDomain("example", WorkerDomainArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostname("subdomain.example.com")
//!             .service("my-service")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:WorkerDomain
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       hostname: subdomain.example.com
//!       service: my-service
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workerDomain:WorkerDomain example <account_id>/<worker_domain_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkerDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    #[builder(into)]
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkerDomainArgs) -> WorkerDomainResult {

    let result = crate::bindings::pulumi::cloudflare::worker_domain::invoke(name, &crate::bindings::pulumi::cloudflare::worker_domain::Args {
        account_id: &args.account_id.get_inner(),
        environment: &args.environment.get_inner(),
        hostname: &args.hostname.get_inner(),
        service: &args.service.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WorkerDomainResult {
        account_id: crate::into_domain(result.account_id),
        environment: crate::into_domain(result.environment),
        hostname: crate::into_domain(result.hostname),
        service: crate::into_domain(result.service),
        zone_id: crate::into_domain(result.zone_id),
    }
}
