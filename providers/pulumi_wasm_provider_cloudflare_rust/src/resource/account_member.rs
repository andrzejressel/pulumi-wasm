//! Provides a resource which manages Cloudflare account members.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.AccountMember("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     emailAddress: "user@example.com",
//!     roleIds: [
//!         "68b329da9893e34099c7d8ad5cb9c940",
//!         "d784fa8b6d98d27699781bd9a7cf19f0",
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.AccountMember("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     email_address="user@example.com",
//!     role_ids=[
//!         "68b329da9893e34099c7d8ad5cb9c940",
//!         "d784fa8b6d98d27699781bd9a7cf19f0",
//!     ])
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
//!     var example = new Cloudflare.AccountMember("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         EmailAddress = "user@example.com",
//!         RoleIds = new[]
//!         {
//!             "68b329da9893e34099c7d8ad5cb9c940",
//!             "d784fa8b6d98d27699781bd9a7cf19f0",
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
//! 		_, err := cloudflare.NewAccountMember(ctx, "example", &cloudflare.AccountMemberArgs{
//! 			AccountId:    pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			EmailAddress: pulumi.String("user@example.com"),
//! 			RoleIds: pulumi.StringArray{
//! 				pulumi.String("68b329da9893e34099c7d8ad5cb9c940"),
//! 				pulumi.String("d784fa8b6d98d27699781bd9a7cf19f0"),
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
//! import com.pulumi.cloudflare.AccountMember;
//! import com.pulumi.cloudflare.AccountMemberArgs;
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
//!         var example = new AccountMember("example", AccountMemberArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .emailAddress("user@example.com")
//!             .roleIds(            
//!                 "68b329da9893e34099c7d8ad5cb9c940",
//!                 "d784fa8b6d98d27699781bd9a7cf19f0")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:AccountMember
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       emailAddress: user@example.com
//!       roleIds:
//!         - 68b329da9893e34099c7d8ad5cb9c940
//!         - d784fa8b6d98d27699781bd9a7cf19f0
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accountMember:AccountMember example <account_id>/<member_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccountMemberArgs {
    /// Account ID to create the account member in.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
    #[builder(into)]
    pub email_address: pulumi_wasm_rust::Output<String>,
    /// List of account role IDs that you want to assign to a member.
    #[builder(into)]
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// A member's status in the account. Available values: `accepted`, `pending`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub status: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccountMemberResult {
    /// Account ID to create the account member in.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
    pub email_address: pulumi_wasm_rust::Output<String>,
    /// List of account role IDs that you want to assign to a member.
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// A member's status in the account. Available values: `accepted`, `pending`.
    pub status: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccountMemberArgs) -> AccountMemberResult {

    let result = crate::bindings::pulumi::cloudflare::account_member::invoke(name, &crate::bindings::pulumi::cloudflare::account_member::Args {
        account_id: &args.account_id.get_inner(),
        email_address: &args.email_address.get_inner(),
        role_ids: &args.role_ids.get_inner(),
        status: &args.status.get_inner(),
    });

    AccountMemberResult {
        account_id: crate::into_domain(result.account_id),
        email_address: crate::into_domain(result.email_address),
        role_ids: crate::into_domain(result.role_ids),
        status: crate::into_domain(result.status),
    }
}
