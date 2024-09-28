//! Use this data source to retrieve all DLP datasets for an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getDlpDatasets({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_dlp_datasets(account_id="f037e56e89293a057740de681ac9abbe")
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
//!     var example = Cloudflare.GetDlpDatasets.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
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
//! 		_, err := cloudflare.GetDlpDatasets(ctx, &cloudflare.GetDlpDatasetsArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetDlpDatasetsArgs;
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
//!         final var example = CloudflareFunctions.getDlpDatasets(GetDlpDatasetsArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getDlpDatasets
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetDlpDatasetsArgs {
    /// The account ID to fetch DLP Datasets from.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetDlpDatasetsResult {
    /// The account ID to fetch DLP Datasets from.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of DLP Datasets.
    pub datasets: pulumi_wasm_rust::Output<Vec<crate::types::GetDlpDatasetsDataset>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDlpDatasetsArgs
) -> GetDlpDatasetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_dlp_datasets::invoke(
        &crate::bindings::pulumi::cloudflare::get_dlp_datasets::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetDlpDatasetsResult {
        account_id: crate::into_domain(result.account_id),
        datasets: crate::into_domain(result.datasets),
        id: crate::into_domain(result.id),
    }
}
