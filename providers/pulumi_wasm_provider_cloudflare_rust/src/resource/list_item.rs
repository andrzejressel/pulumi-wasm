//! Provides individual list items (IPs, Redirects, ASNs, Hostnames) to be used in Edge Rules Engine
//! across all zones within the same account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // IP List
//! const exampleIpList = new cloudflare.List("example_ip_list", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example_list",
//!     description: "example IPs for a list",
//!     kind: "ip",
//! });
//! // IP List Item
//! const exampleIpItem = new cloudflare.ListItem("example_ip_item", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     listId: exampleIpList.id,
//!     comment: "List Item Comment",
//!     ip: "192.0.2.0",
//! });
//! // Redirect List
//! const exampleRedirectList = new cloudflare.List("example_redirect_list", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example_list",
//!     description: "example Redirects for a list",
//!     kind: "redirect",
//! });
//! // Redirect List Item
//! const exampleRedirectItem = new cloudflare.ListItem("example_redirect_item", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     listId: exampleIpList.id,
//!     redirect: {
//!         sourceUrl: "https://source.tld/",
//!         targetUrl: "https://target.tld",
//!         statusCode: 302,
//!         subpathMatching: true,
//!     },
//! });
//! // ASN List
//! const exampleAsnList = new cloudflare.List("example_asn_list", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example_asn_list",
//!     description: "example ASNs for a list",
//!     kind: "asn",
//! });
//! // ASN List Item
//! const exampleAsnItem = new cloudflare.ListItem("example_asn_item", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     listId: exampleAsnList.id,
//!     comment: "List Item Comment",
//!     asn: 6789,
//! });
//! // Hostname List
//! const exampleHostnameList = new cloudflare.List("example_hostname_list", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example_hostname_list",
//!     description: "example Hostnames for a list",
//!     kind: "hostname",
//! });
//! // Hostname List Item
//! const exampleHostnameItem = new cloudflare.ListItem("example_hostname_item", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     listId: exampleHostnameList.id,
//!     comment: "List Item Comment",
//!     hostname: {
//!         urlHostname: "example.com",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # IP List
//! example_ip_list = cloudflare.List("example_ip_list",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example_list",
//!     description="example IPs for a list",
//!     kind="ip")
//! # IP List Item
//! example_ip_item = cloudflare.ListItem("example_ip_item",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     list_id=example_ip_list.id,
//!     comment="List Item Comment",
//!     ip="192.0.2.0")
//! # Redirect List
//! example_redirect_list = cloudflare.List("example_redirect_list",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example_list",
//!     description="example Redirects for a list",
//!     kind="redirect")
//! # Redirect List Item
//! example_redirect_item = cloudflare.ListItem("example_redirect_item",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     list_id=example_ip_list.id,
//!     redirect={
//!         "source_url": "https://source.tld/",
//!         "target_url": "https://target.tld",
//!         "status_code": 302,
//!         "subpath_matching": True,
//!     })
//! # ASN List
//! example_asn_list = cloudflare.List("example_asn_list",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example_asn_list",
//!     description="example ASNs for a list",
//!     kind="asn")
//! # ASN List Item
//! example_asn_item = cloudflare.ListItem("example_asn_item",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     list_id=example_asn_list.id,
//!     comment="List Item Comment",
//!     asn=6789)
//! # Hostname List
//! example_hostname_list = cloudflare.List("example_hostname_list",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example_hostname_list",
//!     description="example Hostnames for a list",
//!     kind="hostname")
//! # Hostname List Item
//! example_hostname_item = cloudflare.ListItem("example_hostname_item",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     list_id=example_hostname_list.id,
//!     comment="List Item Comment",
//!     hostname={
//!         "url_hostname": "example.com",
//!     })
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
//!     // IP List
//!     var exampleIpList = new Cloudflare.List("example_ip_list", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example_list",
//!         Description = "example IPs for a list",
//!         Kind = "ip",
//!     });
//! 
//!     // IP List Item
//!     var exampleIpItem = new Cloudflare.ListItem("example_ip_item", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ListId = exampleIpList.Id,
//!         Comment = "List Item Comment",
//!         Ip = "192.0.2.0",
//!     });
//! 
//!     // Redirect List
//!     var exampleRedirectList = new Cloudflare.List("example_redirect_list", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example_list",
//!         Description = "example Redirects for a list",
//!         Kind = "redirect",
//!     });
//! 
//!     // Redirect List Item
//!     var exampleRedirectItem = new Cloudflare.ListItem("example_redirect_item", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ListId = exampleIpList.Id,
//!         Redirect = new Cloudflare.Inputs.ListItemRedirectArgs
//!         {
//!             SourceUrl = "https://source.tld/",
//!             TargetUrl = "https://target.tld",
//!             StatusCode = 302,
//!             SubpathMatching = true,
//!         },
//!     });
//! 
//!     // ASN List
//!     var exampleAsnList = new Cloudflare.List("example_asn_list", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example_asn_list",
//!         Description = "example ASNs for a list",
//!         Kind = "asn",
//!     });
//! 
//!     // ASN List Item
//!     var exampleAsnItem = new Cloudflare.ListItem("example_asn_item", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ListId = exampleAsnList.Id,
//!         Comment = "List Item Comment",
//!         Asn = 6789,
//!     });
//! 
//!     // Hostname List
//!     var exampleHostnameList = new Cloudflare.List("example_hostname_list", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example_hostname_list",
//!         Description = "example Hostnames for a list",
//!         Kind = "hostname",
//!     });
//! 
//!     // Hostname List Item
//!     var exampleHostnameItem = new Cloudflare.ListItem("example_hostname_item", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ListId = exampleHostnameList.Id,
//!         Comment = "List Item Comment",
//!         Hostname = new Cloudflare.Inputs.ListItemHostnameArgs
//!         {
//!             UrlHostname = "example.com",
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
//! 		// IP List
//! 		exampleIpList, err := cloudflare.NewList(ctx, "example_ip_list", &cloudflare.ListArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("example_list"),
//! 			Description: pulumi.String("example IPs for a list"),
//! 			Kind:        pulumi.String("ip"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// IP List Item
//! 		_, err = cloudflare.NewListItem(ctx, "example_ip_item", &cloudflare.ListItemArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ListId:    exampleIpList.ID(),
//! 			Comment:   pulumi.String("List Item Comment"),
//! 			Ip:        pulumi.String("192.0.2.0"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Redirect List
//! 		_, err = cloudflare.NewList(ctx, "example_redirect_list", &cloudflare.ListArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("example_list"),
//! 			Description: pulumi.String("example Redirects for a list"),
//! 			Kind:        pulumi.String("redirect"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Redirect List Item
//! 		_, err = cloudflare.NewListItem(ctx, "example_redirect_item", &cloudflare.ListItemArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ListId:    exampleIpList.ID(),
//! 			Redirect: &cloudflare.ListItemRedirectArgs{
//! 				SourceUrl:       pulumi.String("https://source.tld/"),
//! 				TargetUrl:       pulumi.String("https://target.tld"),
//! 				StatusCode:      pulumi.Int(302),
//! 				SubpathMatching: pulumi.Bool(true),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// ASN List
//! 		exampleAsnList, err := cloudflare.NewList(ctx, "example_asn_list", &cloudflare.ListArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("example_asn_list"),
//! 			Description: pulumi.String("example ASNs for a list"),
//! 			Kind:        pulumi.String("asn"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// ASN List Item
//! 		_, err = cloudflare.NewListItem(ctx, "example_asn_item", &cloudflare.ListItemArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ListId:    exampleAsnList.ID(),
//! 			Comment:   pulumi.String("List Item Comment"),
//! 			Asn:       pulumi.Int(6789),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Hostname List
//! 		exampleHostnameList, err := cloudflare.NewList(ctx, "example_hostname_list", &cloudflare.ListArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("example_hostname_list"),
//! 			Description: pulumi.String("example Hostnames for a list"),
//! 			Kind:        pulumi.String("hostname"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Hostname List Item
//! 		_, err = cloudflare.NewListItem(ctx, "example_hostname_item", &cloudflare.ListItemArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ListId:    exampleHostnameList.ID(),
//! 			Comment:   pulumi.String("List Item Comment"),
//! 			Hostname: &cloudflare.ListItemHostnameArgs{
//! 				UrlHostname: pulumi.String("example.com"),
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
//! import com.pulumi.cloudflare.List;
//! import com.pulumi.cloudflare.ListArgs;
//! import com.pulumi.cloudflare.ListItem;
//! import com.pulumi.cloudflare.ListItemArgs;
//! import com.pulumi.cloudflare.inputs.ListItemRedirectArgs;
//! import com.pulumi.cloudflare.inputs.ListItemHostnameArgs;
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
//!         // IP List
//!         var exampleIpList = new List("exampleIpList", ListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example_list")
//!             .description("example IPs for a list")
//!             .kind("ip")
//!             .build());
//! 
//!         // IP List Item
//!         var exampleIpItem = new ListItem("exampleIpItem", ListItemArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .listId(exampleIpList.id())
//!             .comment("List Item Comment")
//!             .ip("192.0.2.0")
//!             .build());
//! 
//!         // Redirect List
//!         var exampleRedirectList = new List("exampleRedirectList", ListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example_list")
//!             .description("example Redirects for a list")
//!             .kind("redirect")
//!             .build());
//! 
//!         // Redirect List Item
//!         var exampleRedirectItem = new ListItem("exampleRedirectItem", ListItemArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .listId(exampleIpList.id())
//!             .redirect(ListItemRedirectArgs.builder()
//!                 .sourceUrl("https://source.tld/")
//!                 .targetUrl("https://target.tld")
//!                 .statusCode(302)
//!                 .subpathMatching(true)
//!                 .build())
//!             .build());
//! 
//!         // ASN List
//!         var exampleAsnList = new List("exampleAsnList", ListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example_asn_list")
//!             .description("example ASNs for a list")
//!             .kind("asn")
//!             .build());
//! 
//!         // ASN List Item
//!         var exampleAsnItem = new ListItem("exampleAsnItem", ListItemArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .listId(exampleAsnList.id())
//!             .comment("List Item Comment")
//!             .asn(6789)
//!             .build());
//! 
//!         // Hostname List
//!         var exampleHostnameList = new List("exampleHostnameList", ListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example_hostname_list")
//!             .description("example Hostnames for a list")
//!             .kind("hostname")
//!             .build());
//! 
//!         // Hostname List Item
//!         var exampleHostnameItem = new ListItem("exampleHostnameItem", ListItemArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .listId(exampleHostnameList.id())
//!             .comment("List Item Comment")
//!             .hostname(ListItemHostnameArgs.builder()
//!                 .urlHostname("example.com")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # IP List
//!   exampleIpList:
//!     type: cloudflare:List
//!     name: example_ip_list
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_list
//!       description: example IPs for a list
//!       kind: ip
//!   # IP List Item
//!   exampleIpItem:
//!     type: cloudflare:ListItem
//!     name: example_ip_item
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       listId: ${exampleIpList.id}
//!       comment: List Item Comment
//!       ip: 192.0.2.0
//!   # Redirect List
//!   exampleRedirectList:
//!     type: cloudflare:List
//!     name: example_redirect_list
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_list
//!       description: example Redirects for a list
//!       kind: redirect
//!   # Redirect List Item
//!   exampleRedirectItem:
//!     type: cloudflare:ListItem
//!     name: example_redirect_item
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       listId: ${exampleIpList.id}
//!       redirect:
//!         sourceUrl: https://source.tld/
//!         targetUrl: https://target.tld
//!         statusCode: 302
//!         subpathMatching: true
//!   # ASN List
//!   exampleAsnList:
//!     type: cloudflare:List
//!     name: example_asn_list
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_asn_list
//!       description: example ASNs for a list
//!       kind: asn
//!   # ASN List Item
//!   exampleAsnItem:
//!     type: cloudflare:ListItem
//!     name: example_asn_item
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       listId: ${exampleAsnList.id}
//!       comment: List Item Comment
//!       asn: 6789
//!   # Hostname List
//!   exampleHostnameList:
//!     type: cloudflare:List
//!     name: example_hostname_list
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_hostname_list
//!       description: example Hostnames for a list
//!       kind: hostname
//!   # Hostname List Item
//!   exampleHostnameItem:
//!     type: cloudflare:ListItem
//!     name: example_hostname_item
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       listId: ${exampleHostnameList.id}
//!       comment: List Item Comment
//!       hostname:
//!         urlHostname: example.com
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/listItem:ListItem example <account_id>/<list_id>/<item_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ListItemArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    /// An optional comment for the item.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
    /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    /// The list identifier to target for the resource.
    #[builder(into)]
    pub list_id: pulumi_wasm_rust::Output<String>,
    /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
}

pub struct ListItemResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub asn: pulumi_wasm_rust::Output<Option<i32>>,
    /// An optional comment for the item.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
    /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub ip: pulumi_wasm_rust::Output<Option<String>>,
    /// The list identifier to target for the resource.
    pub list_id: pulumi_wasm_rust::Output<String>,
    /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
    pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ListItemArgs) -> ListItemResult {

    let result = crate::bindings::pulumi::cloudflare::list_item::invoke(name, &crate::bindings::pulumi::cloudflare::list_item::Args {
        account_id: &args.account_id.get_inner(),
        asn: &args.asn.get_inner(),
        comment: &args.comment.get_inner(),
        hostname: &args.hostname.get_inner(),
        ip: &args.ip.get_inner(),
        list_id: &args.list_id.get_inner(),
        redirect: &args.redirect.get_inner(),
    });

    ListItemResult {
        account_id: crate::into_domain(result.account_id),
        asn: crate::into_domain(result.asn),
        comment: crate::into_domain(result.comment),
        hostname: crate::into_domain(result.hostname),
        ip: crate::into_domain(result.ip),
        list_id: crate::into_domain(result.list_id),
        redirect: crate::into_domain(result.redirect),
    }
}
