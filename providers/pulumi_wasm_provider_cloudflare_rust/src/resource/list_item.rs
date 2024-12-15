//! Provides individual list items (IPs, Redirects, ASNs, Hostnames) to be used in Edge Rules Engine
//! across all zones within the same account.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let exampleAsnItem = list_item::create(
//!         "exampleAsnItem",
//!         ListItemArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .asn(6789)
//!             .comment("List Item Comment")
//!             .list_id("${exampleAsnList.id}")
//!             .build_struct(),
//!     );
//!     let exampleAsnList = list::create(
//!         "exampleAsnList",
//!         ListArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("example ASNs for a list")
//!             .kind("asn")
//!             .name("example_asn_list")
//!             .build_struct(),
//!     );
//!     let exampleHostnameItem = list_item::create(
//!         "exampleHostnameItem",
//!         ListItemArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .comment("List Item Comment")
//!             .hostname(
//!                 ListItemHostname::builder().urlHostname("example.com").build_struct(),
//!             )
//!             .list_id("${exampleHostnameList.id}")
//!             .build_struct(),
//!     );
//!     let exampleHostnameList = list::create(
//!         "exampleHostnameList",
//!         ListArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("example Hostnames for a list")
//!             .kind("hostname")
//!             .name("example_hostname_list")
//!             .build_struct(),
//!     );
//!     let exampleIpItem = list_item::create(
//!         "exampleIpItem",
//!         ListItemArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .comment("List Item Comment")
//!             .ip("192.0.2.0")
//!             .list_id("${exampleIpList.id}")
//!             .build_struct(),
//!     );
//!     let exampleIpList = list::create(
//!         "exampleIpList",
//!         ListArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("example IPs for a list")
//!             .kind("ip")
//!             .name("example_list")
//!             .build_struct(),
//!     );
//!     let exampleRedirectItem = list_item::create(
//!         "exampleRedirectItem",
//!         ListItemArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .list_id("${exampleIpList.id}")
//!             .redirect(
//!                 ListItemRedirect::builder()
//!                     .sourceUrl("https://source.tld/")
//!                     .statusCode(302)
//!                     .subpathMatching(true)
//!                     .targetUrl("https://target.tld")
//!                     .build_struct(),
//!             )
//!             .build_struct(),
//!     );
//!     let exampleRedirectList = list::create(
//!         "exampleRedirectList",
//!         ListArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("example Redirects for a list")
//!             .kind("redirect")
//!             .name("example_list")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/listItem:ListItem example <account_id>/<list_id>/<item_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
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
