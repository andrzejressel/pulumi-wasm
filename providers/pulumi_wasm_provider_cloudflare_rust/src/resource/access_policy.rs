//! Provides a Cloudflare Access Policy resource. Access Policies are
//! used in conjunction with Access Applications to restrict access to
//! a particular resource.
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
//! const testPolicyAccessPolicy = new cloudflare.AccessPolicy("testPolicyAccessPolicy", {
//!     applicationId: "cb029e245cfdd66dc8d2e570d5dd3322",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "staging policy",
//!     precedence: 1,
//!     decision: "allow",
//!     includes: [{
//!         emails: ["test@example.com"],
//!     }],
//!     requires: [{
//!         emails: ["test@example.com"],
//!     }],
//! });
//! // Allowing `test@example.com` to access but only when coming from a
//! // specific IP.
//! const testPolicyIndex_accessPolicyAccessPolicy = new cloudflare.AccessPolicy("testPolicyIndex/accessPolicyAccessPolicy", {
//!     applicationId: "cb029e245cfdd66dc8d2e570d5dd3322",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "staging policy",
//!     precedence: 1,
//!     decision: "allow",
//!     includes: [{
//!         emails: ["test@example.com"],
//!     }],
//!     requires: [{
//!         ips: [_var.office_ip],
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Allowing access to `test@example.com` email address only
//! test_policy_access_policy = cloudflare.AccessPolicy("testPolicyAccessPolicy",
//!     application_id="cb029e245cfdd66dc8d2e570d5dd3322",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="staging policy",
//!     precedence=1,
//!     decision="allow",
//!     includes=[cloudflare.AccessPolicyIncludeArgs(
//!         emails=["test@example.com"],
//!     )],
//!     requires=[cloudflare.AccessPolicyRequireArgs(
//!         emails=["test@example.com"],
//!     )])
//! # Allowing `test@example.com` to access but only when coming from a
//! # specific IP.
//! test_policy_index_access_policy_access_policy = cloudflare.AccessPolicy("testPolicyIndex/accessPolicyAccessPolicy",
//!     application_id="cb029e245cfdd66dc8d2e570d5dd3322",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="staging policy",
//!     precedence=1,
//!     decision="allow",
//!     includes=[cloudflare.AccessPolicyIncludeArgs(
//!         emails=["test@example.com"],
//!     )],
//!     requires=[cloudflare.AccessPolicyRequireArgs(
//!         ips=[var["office_ip"]],
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
//!     var testPolicyAccessPolicy = new Cloudflare.AccessPolicy("testPolicyAccessPolicy", new()
//!     {
//!         ApplicationId = "cb029e245cfdd66dc8d2e570d5dd3322",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "staging policy",
//!         Precedence = 1,
//!         Decision = "allow",
//!         Includes = new[]
//!         {
//!             new Cloudflare.Inputs.AccessPolicyIncludeArgs
//!             {
//!                 Emails = new[]
//!                 {
//!                     "test@example.com",
//!                 },
//!             },
//!         },
//!         Requires = new[]
//!         {
//!             new Cloudflare.Inputs.AccessPolicyRequireArgs
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
//!     var testPolicyIndex_accessPolicyAccessPolicy = new Cloudflare.AccessPolicy("testPolicyIndex/accessPolicyAccessPolicy", new()
//!     {
//!         ApplicationId = "cb029e245cfdd66dc8d2e570d5dd3322",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "staging policy",
//!         Precedence = 1,
//!         Decision = "allow",
//!         Includes = new[]
//!         {
//!             new Cloudflare.Inputs.AccessPolicyIncludeArgs
//!             {
//!                 Emails = new[]
//!                 {
//!                     "test@example.com",
//!                 },
//!             },
//!         },
//!         Requires = new[]
//!         {
//!             new Cloudflare.Inputs.AccessPolicyRequireArgs
//!             {
//!                 Ips = new[]
//!                 {
//!                     @var.Office_ip,
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
//! 		_, err := cloudflare.NewAccessPolicy(ctx, "testPolicyAccessPolicy", &cloudflare.AccessPolicyArgs{
//! 			ApplicationId: pulumi.String("cb029e245cfdd66dc8d2e570d5dd3322"),
//! 			ZoneId:        pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:          pulumi.String("staging policy"),
//! 			Precedence:    pulumi.Int(1),
//! 			Decision:      pulumi.String("allow"),
//! 			Includes: cloudflare.AccessPolicyIncludeArray{
//! 				&cloudflare.AccessPolicyIncludeArgs{
//! 					Emails: pulumi.StringArray{
//! 						pulumi.String("test@example.com"),
//! 					},
//! 				},
//! 			},
//! 			Requires: cloudflare.AccessPolicyRequireArray{
//! 				&cloudflare.AccessPolicyRequireArgs{
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
//! 		_, err = cloudflare.NewAccessPolicy(ctx, "testPolicyIndex/accessPolicyAccessPolicy", &cloudflare.AccessPolicyArgs{
//! 			ApplicationId: pulumi.String("cb029e245cfdd66dc8d2e570d5dd3322"),
//! 			ZoneId:        pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:          pulumi.String("staging policy"),
//! 			Precedence:    pulumi.Int(1),
//! 			Decision:      pulumi.String("allow"),
//! 			Includes: cloudflare.AccessPolicyIncludeArray{
//! 				&cloudflare.AccessPolicyIncludeArgs{
//! 					Emails: pulumi.StringArray{
//! 						pulumi.String("test@example.com"),
//! 					},
//! 				},
//! 			},
//! 			Requires: cloudflare.AccessPolicyRequireArray{
//! 				&cloudflare.AccessPolicyRequireArgs{
//! 					Ips: pulumi.StringArray{
//! 						_var.Office_ip,
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
//! import com.pulumi.cloudflare.AccessPolicy;
//! import com.pulumi.cloudflare.AccessPolicyArgs;
//! import com.pulumi.cloudflare.inputs.AccessPolicyIncludeArgs;
//! import com.pulumi.cloudflare.inputs.AccessPolicyRequireArgs;
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
//!         var testPolicyAccessPolicy = new AccessPolicy("testPolicyAccessPolicy", AccessPolicyArgs.builder()        
//!             .applicationId("cb029e245cfdd66dc8d2e570d5dd3322")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("staging policy")
//!             .precedence("1")
//!             .decision("allow")
//!             .includes(AccessPolicyIncludeArgs.builder()
//!                 .emails("test@example.com")
//!                 .build())
//!             .requires(AccessPolicyRequireArgs.builder()
//!                 .emails("test@example.com")
//!                 .build())
//!             .build());
//! 
//!         // Allowing `test@example.com` to access but only when coming from a
//!         // specific IP.
//!         var testPolicyIndex_accessPolicyAccessPolicy = new AccessPolicy("testPolicyIndex/accessPolicyAccessPolicy", AccessPolicyArgs.builder()        
//!             .applicationId("cb029e245cfdd66dc8d2e570d5dd3322")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("staging policy")
//!             .precedence("1")
//!             .decision("allow")
//!             .includes(AccessPolicyIncludeArgs.builder()
//!                 .emails("test@example.com")
//!                 .build())
//!             .requires(AccessPolicyRequireArgs.builder()
//!                 .ips(var_.office_ip())
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
//!   testPolicyAccessPolicy:
//!     type: cloudflare:AccessPolicy
//!     properties:
//!       applicationId: cb029e245cfdd66dc8d2e570d5dd3322
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: staging policy
//!       precedence: '1'
//!       decision: allow
//!       includes:
//!         - emails:
//!             - test@example.com
//!       requires:
//!         - emails:
//!             - test@example.com
//!   # Allowing `test@example.com` to access but only when coming from a
//!   # specific IP.
//!   testPolicyIndex/accessPolicyAccessPolicy:
//!     type: cloudflare:AccessPolicy
//!     properties:
//!       applicationId: cb029e245cfdd66dc8d2e570d5dd3322
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: staging policy
//!       precedence: '1'
//!       decision: allow
//!       includes:
//!         - emails:
//!             - test@example.com
//!       requires:
//!         - ips:
//!             - ${var.office_ip}
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Account level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessPolicy:AccessPolicy example account/<account_id>/<application_id>/<policy_id>
//! ```
//! 
//! Zone level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessPolicy:AccessPolicy example zone/<zone_id>/<application_id>/<policy_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the application the policy is associated with.
    #[builder(into)]
    pub application_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub approval_groups: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    #[builder(into)]
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
    /// A series of access conditions, see Access Groups.
    #[builder(into)]
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application.
    #[builder(into)]
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessPolicyResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The ID of the application the policy is associated with.
    pub application_id: pulumi_wasm_rust::Output<String>,
    pub approval_groups: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
    /// A series of access conditions, see Access Groups.
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application.
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {

    let result = crate::bindings::pulumi::cloudflare::access_policy::invoke(name, &crate::bindings::pulumi::cloudflare::access_policy::Args {
        account_id: &args.account_id.get_inner(),
        application_id: &args.application_id.get_inner(),
        approval_groups: &args.approval_groups.get_inner(),
        approval_required: &args.approval_required.get_inner(),
        decision: &args.decision.get_inner(),
        excludes: &args.excludes.get_inner(),
        includes: &args.includes.get_inner(),
        isolation_required: &args.isolation_required.get_inner(),
        name: &args.name.get_inner(),
        precedence: &args.precedence.get_inner(),
        purpose_justification_prompt: &args.purpose_justification_prompt.get_inner(),
        purpose_justification_required: &args.purpose_justification_required.get_inner(),
        requires: &args.requires.get_inner(),
        session_duration: &args.session_duration.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessPolicyResult {
        account_id: crate::into_domain(result.account_id),
        application_id: crate::into_domain(result.application_id),
        approval_groups: crate::into_domain(result.approval_groups),
        approval_required: crate::into_domain(result.approval_required),
        decision: crate::into_domain(result.decision),
        excludes: crate::into_domain(result.excludes),
        includes: crate::into_domain(result.includes),
        isolation_required: crate::into_domain(result.isolation_required),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        purpose_justification_prompt: crate::into_domain(result.purpose_justification_prompt),
        purpose_justification_required: crate::into_domain(result.purpose_justification_required),
        requires: crate::into_domain(result.requires),
        session_duration: crate::into_domain(result.session_duration),
        zone_id: crate::into_domain(result.zone_id),
    }
}
