//! Use this data source to retrieve information about the currently authenticated user.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const me = cloudflare.getUser({});
//! const all = cloudflare.getApiTokenPermissionGroups({});
//! const example = new cloudflare.ApiToken("example", {
//!     name: "Terraform Cloud (Terraform)",
//!     policies: [{
//!         permissionGroups: [all.then(all => all.user?.["User Details Read"])],
//!         resources: me.then(me => {
//!             [`com.cloudflare.api.user.${me.id}`]: "*",
//!         }),
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! me = cloudflare.get_user()
//! all = cloudflare.get_api_token_permission_groups()
//! example = cloudflare.ApiToken("example",
//!     name="Terraform Cloud (Terraform)",
//!     policies=[cloudflare.ApiTokenPolicyArgs(
//!         permission_groups=[all.user["User Details Read"]],
//!         resources={
//!             f"com.cloudflare.api.user.{me.id}": "*",
//!         },
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
//!     var me = Cloudflare.GetUser.Invoke();
//! 
//!     var all = Cloudflare.GetApiTokenPermissionGroups.Invoke();
//! 
//!     var example = new Cloudflare.ApiToken("example", new()
//!     {
//!         Name = "Terraform Cloud (Terraform)",
//!         Policies = new[]
//!         {
//!             new Cloudflare.Inputs.ApiTokenPolicyArgs
//!             {
//!                 PermissionGroups = new[]
//!                 {
//!                     all.Apply(getApiTokenPermissionGroupsResult => getApiTokenPermissionGroupsResult.User?.User_Details_Read),
//!                 },
//!                 Resources = 
//!                 {
//!                     { $"com.cloudflare.api.user.{me.Apply(getUserResult => getUserResult.Id)}", "*" },
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
//! 	"fmt"
//! 
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! func main() {
//! pulumi.Run(func(ctx *pulumi.Context) error {
//! me, err := cloudflare.GetUser(ctx, nil, nil);
//! if err != nil {
//! return err
//! }
//! all, err := cloudflare.GetApiTokenPermissionGroups(ctx, nil, nil);
//! if err != nil {
//! return err
//! }
//! _, err = cloudflare.NewApiToken(ctx, "example", &cloudflare.ApiTokenArgs{
//! Name: pulumi.String("Terraform Cloud (Terraform)"),
//! Policies: cloudflare.ApiTokenPolicyArray{
//! &cloudflare.ApiTokenPolicyArgs{
//! PermissionGroups: pulumi.StringArray{
//! pulumi.String(all.User.User Details Read),
//! },
//! Resources: pulumi.StringMap{
//! fmt.Sprintf("com.cloudflare.api.user.%v", me.Id): pulumi.String("*"),
//! },
//! },
//! },
//! })
//! if err != nil {
//! return err
//! }
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
//! import com.pulumi.cloudflare.ApiToken;
//! import com.pulumi.cloudflare.ApiTokenArgs;
//! import com.pulumi.cloudflare.inputs.ApiTokenPolicyArgs;
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
//!         final var me = CloudflareFunctions.getUser();
//! 
//!         final var all = CloudflareFunctions.getApiTokenPermissionGroups();
//! 
//!         var example = new ApiToken("example", ApiTokenArgs.builder()        
//!             .name("Terraform Cloud (Terraform)")
//!             .policies(ApiTokenPolicyArgs.builder()
//!                 .permissionGroups(all.applyValue(getApiTokenPermissionGroupsResult -> getApiTokenPermissionGroupsResult.user().User Details Read()))
//!                 .resources(Map.of(String.format("com.cloudflare.api.user.%s", me.applyValue(getUserResult -> getUserResult.id())), "*"))
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ApiToken
//!     properties:
//!       name: Terraform Cloud (Terraform)
//!       policies:
//!         - permissionGroups:
//!             - ${all.user"User Details Read"[%!s(MISSING)]}
//!           resources:
//!             com.cloudflare.api.user.${me.id}: '*'
//! variables:
//!   me:
//!     fn::invoke:
//!       Function: cloudflare:getUser
//!       Arguments: {}
//!   all:
//!     fn::invoke:
//!       Function: cloudflare:getApiTokenPermissionGroups
//!       Arguments: {}
//! ```
//! <!--End PulumiCodeChooser -->


pub struct GetUserResult {
    /// The user's email address.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The user's unique identifier.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The user's username.
    pub username: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> GetUserResult {

    let result = crate::bindings::pulumi::cloudflare::get_user::invoke(
    );

    GetUserResult {
        email: crate::into_domain(result.email),
        id: crate::into_domain(result.id),
        username: crate::into_domain(result.username),
    }
}
