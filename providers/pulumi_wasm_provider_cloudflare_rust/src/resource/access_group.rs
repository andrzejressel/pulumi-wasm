//! Provides a Cloudflare Access Group resource. Access Groups are used
//! in conjunction with Access Policies to restrict access to a
//! particular resource based on group membership.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Allowing access to `test@example.com` email address only
//! const exampleAccessGroup = new cloudflare.AccessGroup("exampleAccessGroup", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "staging group",
//!     includes: [{
//!         emails: ["test@example.com"],
//!     }],
//! });
//! // Allowing `test@example.com` to access but only when coming from a
//! // specific IP.
//! const exampleIndex_accessGroupAccessGroup = new cloudflare.AccessGroup("exampleIndex/accessGroupAccessGroup", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "staging group",
//!     includes: [{
//!         emails: ["test@example.com"],
//!     }],
//!     requires: [{
//!         ips: [_var.office_ip],
//!     }],
//! });
//! // Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//! const exampleCloudflareIndex_accessGroupAccessGroup = new cloudflare.AccessGroup("exampleCloudflareIndex/accessGroupAccessGroup", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "test_group",
//!     includes: [{
//!         azures: [{
//!             identityProviderId: "ca298b82-93b5-41bf-bc2d-10493f09b761",
//!             ids: ["86773093-5feb-48dd-814b-7ccd3676ff50"],
//!         }],
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Allowing access to `test@example.com` email address only
//! example_access_group = cloudflare.AccessGroup("exampleAccessGroup",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="staging group",
//!     includes=[cloudflare.AccessGroupIncludeArgs(
//!         emails=["test@example.com"],
//!     )])
//! # Allowing `test@example.com` to access but only when coming from a
//! # specific IP.
//! example_index_access_group_access_group = cloudflare.AccessGroup("exampleIndex/accessGroupAccessGroup",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="staging group",
//!     includes=[cloudflare.AccessGroupIncludeArgs(
//!         emails=["test@example.com"],
//!     )],
//!     requires=[cloudflare.AccessGroupRequireArgs(
//!         ips=[var["office_ip"]],
//!     )])
//! # Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//! example_cloudflare_index_access_group_access_group = cloudflare.AccessGroup("exampleCloudflareIndex/accessGroupAccessGroup",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="test_group",
//!     includes=[cloudflare.AccessGroupIncludeArgs(
//!         azures=[cloudflare.AccessGroupIncludeAzureArgs(
//!             identity_provider_id="ca298b82-93b5-41bf-bc2d-10493f09b761",
//!             ids=["86773093-5feb-48dd-814b-7ccd3676ff50"],
//!         )],
//!     )])
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
//!     // Allowing access to `test@example.com` email address only
//!     var exampleAccessGroup = new Cloudflare.AccessGroup("exampleAccessGroup", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "staging group",
//!         Includes = new[]
//!         {
//!             new Cloudflare.Inputs.AccessGroupIncludeArgs
//!             {
//!                 Emails = new[]
//!                 {
//!                     "test@example.com",
//!                 },
//!             },
//!         },
//!     });
//! 
//!     // Allowing `test@example.com` to access but only when coming from a
//!     // specific IP.
//!     var exampleIndex_accessGroupAccessGroup = new Cloudflare.AccessGroup("exampleIndex/accessGroupAccessGroup", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "staging group",
//!         Includes = new[]
//!         {
//!             new Cloudflare.Inputs.AccessGroupIncludeArgs
//!             {
//!                 Emails = new[]
//!                 {
//!                     "test@example.com",
//!                 },
//!             },
//!         },
//!         Requires = new[]
//!         {
//!             new Cloudflare.Inputs.AccessGroupRequireArgs
//!             {
//!                 Ips = new[]
//!                 {
//!                     @var.Office_ip,
//!                 },
//!             },
//!         },
//!     });
//! 
//!     // Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//!     var exampleCloudflareIndex_accessGroupAccessGroup = new Cloudflare.AccessGroup("exampleCloudflareIndex/accessGroupAccessGroup", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "test_group",
//!         Includes = new[]
//!         {
//!             new Cloudflare.Inputs.AccessGroupIncludeArgs
//!             {
//!                 Azures = new[]
//!                 {
//!                     new Cloudflare.Inputs.AccessGroupIncludeAzureArgs
//!                     {
//!                         IdentityProviderId = "ca298b82-93b5-41bf-bc2d-10493f09b761",
//!                         Ids = new[]
//!                         {
//!                             "86773093-5feb-48dd-814b-7ccd3676ff50",
//!                         },
//!                     },
//!                 },
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
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Allowing access to `test@example.com` email address only
//! 		_, err := cloudflare.NewAccessGroup(ctx, "exampleAccessGroup", &cloudflare.AccessGroupArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("staging group"),
//! 			Includes: cloudflare.AccessGroupIncludeArray{
//! 				&cloudflare.AccessGroupIncludeArgs{
//! 					Emails: pulumi.StringArray{
//! 						pulumi.String("test@example.com"),
//! 					},
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Allowing `test@example.com` to access but only when coming from a
//! 		// specific IP.
//! 		_, err = cloudflare.NewAccessGroup(ctx, "exampleIndex/accessGroupAccessGroup", &cloudflare.AccessGroupArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("staging group"),
//! 			Includes: cloudflare.AccessGroupIncludeArray{
//! 				&cloudflare.AccessGroupIncludeArgs{
//! 					Emails: pulumi.StringArray{
//! 						pulumi.String("test@example.com"),
//! 					},
//! 				},
//! 			},
//! 			Requires: cloudflare.AccessGroupRequireArray{
//! 				&cloudflare.AccessGroupRequireArgs{
//! 					Ips: pulumi.StringArray{
//! 						_var.Office_ip,
//! 					},
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//! 		_, err = cloudflare.NewAccessGroup(ctx, "exampleCloudflareIndex/accessGroupAccessGroup", &cloudflare.AccessGroupArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("test_group"),
//! 			Includes: cloudflare.AccessGroupIncludeArray{
//! 				&cloudflare.AccessGroupIncludeArgs{
//! 					Azures: cloudflare.AccessGroupIncludeAzureArray{
//! 						&cloudflare.AccessGroupIncludeAzureArgs{
//! 							IdentityProviderId: pulumi.String("ca298b82-93b5-41bf-bc2d-10493f09b761"),
//! 							Ids: pulumi.StringArray{
//! 								pulumi.String("86773093-5feb-48dd-814b-7ccd3676ff50"),
//! 							},
//! 						},
//! 					},
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
//! import com.pulumi.cloudflare.AccessGroup;
//! import com.pulumi.cloudflare.AccessGroupArgs;
//! import com.pulumi.cloudflare.inputs.AccessGroupIncludeArgs;
//! import com.pulumi.cloudflare.inputs.AccessGroupRequireArgs;
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
//!         // Allowing access to `test@example.com` email address only
//!         var exampleAccessGroup = new AccessGroup("exampleAccessGroup", AccessGroupArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("staging group")
//!             .includes(AccessGroupIncludeArgs.builder()
//!                 .emails("test@example.com")
//!                 .build())
//!             .build());
//! 
//!         // Allowing `test@example.com` to access but only when coming from a
//!         // specific IP.
//!         var exampleIndex_accessGroupAccessGroup = new AccessGroup("exampleIndex/accessGroupAccessGroup", AccessGroupArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("staging group")
//!             .includes(AccessGroupIncludeArgs.builder()
//!                 .emails("test@example.com")
//!                 .build())
//!             .requires(AccessGroupRequireArgs.builder()
//!                 .ips(var_.office_ip())
//!                 .build())
//!             .build());
//! 
//!         // Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//!         var exampleCloudflareIndex_accessGroupAccessGroup = new AccessGroup("exampleCloudflareIndex/accessGroupAccessGroup", AccessGroupArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("test_group")
//!             .includes(AccessGroupIncludeArgs.builder()
//!                 .azures(AccessGroupIncludeAzureArgs.builder()
//!                     .identityProviderId("ca298b82-93b5-41bf-bc2d-10493f09b761")
//!                     .ids("86773093-5feb-48dd-814b-7ccd3676ff50")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Allowing access to `test@example.com` email address only
//!   exampleAccessGroup:
//!     type: cloudflare:AccessGroup
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: staging group
//!       includes:
//!         - emails:
//!             - test@example.com
//!   # Allowing `test@example.com` to access but only when coming from a
//!   # specific IP.
//!   exampleIndex/accessGroupAccessGroup:
//!     type: cloudflare:AccessGroup
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: staging group
//!       includes:
//!         - emails:
//!             - test@example.com
//!       requires:
//!         - ips:
//!             - ${var.office_ip}
//!   # Allow members of an Azure Group. The ID is the group UUID (id) in Azure.
//!   exampleCloudflareIndex/accessGroupAccessGroup:
//!     type: cloudflare:AccessGroup
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: test_group
//!       includes:
//!         - azures:
//!             - identityProviderId: ca298b82-93b5-41bf-bc2d-10493f09b761
//!               ids:
//!                 - 86773093-5feb-48dd-814b-7ccd3676ff50
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessGroup:AccessGroup example <account_id>/<group_id>
//! ```
//! 

pub struct AccessGroupArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessGroupResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessGroupArgs) -> AccessGroupResult {

    let result = crate::bindings::pulumi::cloudflare::access_group::invoke(name, &crate::bindings::pulumi::cloudflare::access_group::Args {
        account_id: args.account_id.get_inner(),
        excludes: args.excludes.get_inner(),
        includes: args.includes.get_inner(),
        name: args.name.get_inner(),
        requires: args.requires.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    AccessGroupResult {
        account_id: crate::into_domain(result.account_id),
        excludes: crate::into_domain(result.excludes),
        includes: crate::into_domain(result.includes),
        name: crate::into_domain(result.name),
        requires: crate::into_domain(result.requires),
        zone_id: crate::into_domain(result.zone_id),
    }
}
