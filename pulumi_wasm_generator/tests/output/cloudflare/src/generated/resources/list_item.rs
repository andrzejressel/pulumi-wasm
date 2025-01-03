/// Provides individual list items (IPs, Redirects, ASNs, Hostnames) to be used in Edge Rules Engine
/// across all zones within the same account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let exampleAsnItem = list_item::create(
///         "exampleAsnItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .asn(6789)
///             .comment("List Item Comment")
///             .list_id("${exampleAsnList.id}")
///             .build_struct(),
///     );
///     let exampleAsnList = list::create(
///         "exampleAsnList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example ASNs for a list")
///             .kind("asn")
///             .name("example_asn_list")
///             .build_struct(),
///     );
///     let exampleHostnameItem = list_item::create(
///         "exampleHostnameItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("List Item Comment")
///             .hostname(
///                 ListItemHostname::builder().urlHostname("example.com").build_struct(),
///             )
///             .list_id("${exampleHostnameList.id}")
///             .build_struct(),
///     );
///     let exampleHostnameList = list::create(
///         "exampleHostnameList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example Hostnames for a list")
///             .kind("hostname")
///             .name("example_hostname_list")
///             .build_struct(),
///     );
///     let exampleIpItem = list_item::create(
///         "exampleIpItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("List Item Comment")
///             .ip("192.0.2.0")
///             .list_id("${exampleIpList.id}")
///             .build_struct(),
///     );
///     let exampleIpList = list::create(
///         "exampleIpList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example IPs for a list")
///             .kind("ip")
///             .name("example_list")
///             .build_struct(),
///     );
///     let exampleRedirectItem = list_item::create(
///         "exampleRedirectItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .list_id("${exampleIpList.id}")
///             .redirect(
///                 ListItemRedirect::builder()
///                     .sourceUrl("https://source.tld/")
///                     .statusCode(302)
///                     .subpathMatching(true)
///                     .targetUrl("https://target.tld")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleRedirectList = list::create(
///         "exampleRedirectList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example Redirects for a list")
///             .kind("redirect")
///             .name("example_list")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/listItem:ListItem example <account_id>/<list_id>/<item_id>
/// ```
///
pub mod list_item {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListItemArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub asn: pulumi_wasm_rust::Output<Option<i32>>,
        /// An optional comment for the item.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub hostname: pulumi_wasm_rust::Output<Option<super::types::ListItemHostname>>,
        /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub ip: pulumi_wasm_rust::Output<Option<String>>,
        /// The list identifier to target for the resource.
        #[builder(into)]
        pub list_id: pulumi_wasm_rust::Output<String>,
        /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub redirect: pulumi_wasm_rust::Output<Option<super::types::ListItemRedirect>>,
    }
    #[allow(dead_code)]
    pub struct ListItemResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub asn: pulumi_wasm_rust::Output<Option<i32>>,
        /// An optional comment for the item.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub hostname: pulumi_wasm_rust::Output<Option<super::types::ListItemHostname>>,
        /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub ip: pulumi_wasm_rust::Output<Option<String>>,
        /// The list identifier to target for the resource.
        pub list_id: pulumi_wasm_rust::Output<String>,
        /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub redirect: pulumi_wasm_rust::Output<Option<super::types::ListItemRedirect>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListItemArgs) -> ListItemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let asn_binding = args.asn.get_inner();
        let comment_binding = args.comment.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let ip_binding = args.ip.get_inner();
        let list_id_binding = args.list_id.get_inner();
        let redirect_binding = args.redirect.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/listItem:ListItem".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "asn".into(),
                    value: &asn_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ip".into(),
                    value: &ip_binding,
                },
                register_interface::ObjectField {
                    name: "listId".into(),
                    value: &list_id_binding,
                },
                register_interface::ObjectField {
                    name: "redirect".into(),
                    value: &redirect_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "asn".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "ip".into(),
                },
                register_interface::ResultField {
                    name: "listId".into(),
                },
                register_interface::ResultField {
                    name: "redirect".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListItemResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("asn").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            ip: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ip").unwrap()),
            list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listId").unwrap(),
            ),
            redirect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redirect").unwrap(),
            ),
        }
    }
}
