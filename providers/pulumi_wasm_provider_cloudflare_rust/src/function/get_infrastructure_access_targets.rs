//! Use this data source to retrieve all Infrastructure Access Targets.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getInfrastructureAccessTargets({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     hostnameContains: "example",
//!     ipv4: "198.51.100.1",
//! });
//! export const targets = example.then(example => example.targets);
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_infrastructure_access_targets(account_id="f037e56e89293a057740de681ac9abbe",
//!     hostname_contains="example",
//!     ipv4="198.51.100.1")
//! pulumi.export("targets", example.targets)
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
//!     var example = Cloudflare.GetInfrastructureAccessTargets.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         HostnameContains = "example",
//!         Ipv4 = "198.51.100.1",
//!     });
//! 
//!     return new Dictionary<string, object?>
//!     {
//!         ["targets"] = example.Apply(getInfrastructureAccessTargetsResult => getInfrastructureAccessTargetsResult.Targets),
//!     };
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
//! 		example, err := cloudflare.GetInfrastructureAccessTargets(ctx, &cloudflare.GetInfrastructureAccessTargetsArgs{
//! 			AccountId:        "f037e56e89293a057740de681ac9abbe",
//! 			HostnameContains: pulumi.StringRef("example"),
//! 			Ipv4:             pulumi.StringRef("198.51.100.1"),
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		ctx.Export("targets", example.Targets)
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
//! import com.pulumi.cloudflare.inputs.GetInfrastructureAccessTargetsArgs;
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
//!         final var example = CloudflareFunctions.getInfrastructureAccessTargets(GetInfrastructureAccessTargetsArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .hostnameContains("example")
//!             .ipv4("198.51.100.1")
//!             .build());
//! 
//!         ctx.export("targets", example.applyValue(getInfrastructureAccessTargetsResult -> getInfrastructureAccessTargetsResult.targets()));
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getInfrastructureAccessTargets
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         hostnameContains: example
//!         ipv4: 198.51.100.1
//! outputs:
//!   # output the list of targets the data source contains
//!   targets: ${example.targets}
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetInfrastructureAccessTargetsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A date and time after a target was created to filter on.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub created_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostname of the target.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// Partial match to the hostname of a target
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname_contains: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv4 address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ipv4: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv6 address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ipv6: pulumi_wasm_rust::Output<Option<String>>,
    /// A date and time after a target was modified to filter on.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub modified_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The private virtual network identifier for the target.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetInfrastructureAccessTargetsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A date and time after a target was created to filter on.
    pub created_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostname of the target.
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// Partial match to the hostname of a target
    pub hostname_contains: pulumi_wasm_rust::Output<Option<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The target's IPv4 address.
    pub ipv4: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv6 address.
    pub ipv6: pulumi_wasm_rust::Output<Option<String>>,
    /// A date and time after a target was modified to filter on.
    pub modified_after: pulumi_wasm_rust::Output<Option<String>>,
    pub targets: pulumi_wasm_rust::Output<Vec<crate::types::GetInfrastructureAccessTargetsTarget>>,
    /// The private virtual network identifier for the target.
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetInfrastructureAccessTargetsArgs
) -> GetInfrastructureAccessTargetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_infrastructure_access_targets::invoke(
        &crate::bindings::pulumi::cloudflare::get_infrastructure_access_targets::Args {
                account_id: &args.account_id.get_inner(),
                created_after: &args.created_after.get_inner(),
                hostname: &args.hostname.get_inner(),
                hostname_contains: &args.hostname_contains.get_inner(),
                ipv4: &args.ipv4.get_inner(),
                ipv6: &args.ipv6.get_inner(),
                modified_after: &args.modified_after.get_inner(),
                virtual_network_id: &args.virtual_network_id.get_inner(),
        }
    );

    GetInfrastructureAccessTargetsResult {
        account_id: crate::into_domain(result.account_id),
        created_after: crate::into_domain(result.created_after),
        hostname: crate::into_domain(result.hostname),
        hostname_contains: crate::into_domain(result.hostname_contains),
        id: crate::into_domain(result.id),
        ipv4: crate::into_domain(result.ipv4),
        ipv6: crate::into_domain(result.ipv6),
        modified_after: crate::into_domain(result.modified_after),
        targets: crate::into_domain(result.targets),
        virtual_network_id: crate::into_domain(result.virtual_network_id),
    }
}
