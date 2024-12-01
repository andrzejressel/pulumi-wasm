//! Provides individual list items (IPs, Redirects, ASNs, Hostnames) to be used in Edge Rules Engine
//! across all zones within the same account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
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
