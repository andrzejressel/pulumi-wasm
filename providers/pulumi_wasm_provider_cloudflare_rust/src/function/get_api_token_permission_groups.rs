//! Use this data source to look up [API Token Permission Groups](https://developers.cloudflare.com/api/tokens/create/permissions).
//! Commonly used as references within [`cloudflare_token`](https://www.terraform.io/docs/providers/cloudflare/r/api_token.html) resources.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const all = cloudflare.getApiTokenPermissionGroups({});
//! export const dnsReadPermissionId = all.then(all => all.zone?.["DNS Read"]);
//! export const accountLbMonitorsAndReadId = all.then(all => all.account?.["Load Balancing: Monitors and Pools Read"]);
//! export const userMembershipsReadId = all.then(all => all.user?.["Memberships Read"]);
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! all = cloudflare.get_api_token_permission_groups()
//! pulumi.export("dnsReadPermissionId", all.zone["DNS Read"])
//! pulumi.export("accountLbMonitorsAndReadId", all.account["Load Balancing: Monitors and Pools Read"])
//! pulumi.export("userMembershipsReadId", all.user["Memberships Read"])
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
//!     var all = Cloudflare.GetApiTokenPermissionGroups.Invoke();
//! 
//!     return new Dictionary<string, object?>
//!     {
//!         ["dnsReadPermissionId"] = all.Apply(getApiTokenPermissionGroupsResult => getApiTokenPermissionGroupsResult.Zone?.DNS_Read),
//!         ["accountLbMonitorsAndReadId"] = all.Apply(getApiTokenPermissionGroupsResult => getApiTokenPermissionGroupsResult.Account?.Load_Balancing__Monitors_and_Pools_Read),
//!         ["userMembershipsReadId"] = all.Apply(getApiTokenPermissionGroupsResult => getApiTokenPermissionGroupsResult.User?.Memberships_Read),
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
//! func main() {
//! pulumi.Run(func(ctx *pulumi.Context) error {
//! all, err := cloudflare.GetApiTokenPermissionGroups(ctx, nil, nil);
//! if err != nil {
//! return err
//! }
//! ctx.Export("dnsReadPermissionId", all.Zone.DNS Read)
//! ctx.Export("accountLbMonitorsAndReadId", all.Account.Load Balancing: Monitors and Pools Read)
//! ctx.Export("userMembershipsReadId", all.User.Memberships Read)
//! return nil
//! })
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
//!         final var all = CloudflareFunctions.getApiTokenPermissionGroups();
//! 
//!         ctx.export("dnsReadPermissionId", all.applyValue(getApiTokenPermissionGroupsResult -> getApiTokenPermissionGroupsResult.zone().DNS Read()));
//!         ctx.export("accountLbMonitorsAndReadId", all.applyValue(getApiTokenPermissionGroupsResult -> getApiTokenPermissionGroupsResult.account().Load Balancing: Monitors and Pools Read()));
//!         ctx.export("userMembershipsReadId", all.applyValue(getApiTokenPermissionGroupsResult -> getApiTokenPermissionGroupsResult.user().Memberships Read()));
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   all:
//!     fn::invoke:
//!       Function: cloudflare:getApiTokenPermissionGroups
//!       Arguments: {}
//! outputs:
//!   # Get zone level DNS read permission ID.
//!   dnsReadPermissionId: ${all.zone"DNS Read"[%!s(MISSING)]}
//!   # Get account level "Load Balancing: Monitors and Pools Read" permission ID.
//!   accountLbMonitorsAndReadId: '${all.account"Load Balancing: Monitors and Pools Read"[%!s(MISSING)]}'
//!   # Get user level "Memberships Read" permission ID.
//!   userMembershipsReadId: ${all.user"Memberships Read"[%!s(MISSING)]}
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetApiTokenPermissionGroupsArgs {
}

pub struct GetApiTokenPermissionGroupsResult {
    /// Map of permissions for account level resources.
    pub account: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Checksum of permissions.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Map of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes.
    pub permissions: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Map of permissions for r2 level resources.
    pub r2: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Map of permissions for user level resources.
    pub user: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Map of permissions for zone level resources.
    pub zone: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetApiTokenPermissionGroupsArgs) -> GetApiTokenPermissionGroupsResult {

    let result = crate::bindings::pulumi::cloudflare::get_api_token_permission_groups::invoke(&crate::bindings::pulumi::cloudflare::get_api_token_permission_groups::Args {
    });

    GetApiTokenPermissionGroupsResult {
        account: crate::into_domain(result.account),
        id: crate::into_domain(result.id),
        permissions: crate::into_domain(result.permissions),
        r2: crate::into_domain(result.r2),
        user: crate::into_domain(result.user),
        zone: crate::into_domain(result.zone),
    }
}
